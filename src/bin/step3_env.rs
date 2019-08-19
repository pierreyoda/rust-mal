extern crate mal;
use mal::{types, env, reader, readline};
use mal::types::{MalValue, MalResult, MalError, new_symbol, new_function,
        new_integer, err_str, err_string};
use mal::types::MalType::*;

fn read(string: &str) -> MalResult {
    reader::read_str(string)
}

fn eval_ast(ast: MalValue, env: &env::Env) -> MalResult {
    match *ast {
        Symbol(_) => env::get(&env, &ast),
        List(ref seq) | Vector(ref seq) => {
            let mut ast_ev = vec!();
            for value in seq {
                ast_ev.push(eval(value.clone(), env)?);
            }
            Ok(match *ast { List(_) => types::new_list(ast_ev),
                                 _  => types::new_vector(ast_ev)})
        },
        _ => Ok(ast.clone()),
    }
}

fn eval(ast: MalValue, env: &env::Env) -> MalResult {
    let ast_temp = ast.clone();
    let (arg0_symbol, args): (Option<&str>, &Vec<MalValue>) = match *ast_temp {
        List(ref seq) => {
            if seq.len() == 0 { return Ok(ast); }
            match *seq[0] {
                Symbol(ref symbol) => (Some(&symbol[..]), seq),
                _                  => (None, seq),
            }
        },
        _ => return eval_ast(ast, env)
    };


    match arg0_symbol {
        Some(slice) => {
            match slice {
                // (def! key value) ; key must be a Symbol
                // bind the evaluated value in env with the unevaluated key
                "def!" => {
                    if args.len() != 3 {
                        return err_str("wrong arity for \"def!\", should be 2");
                    }
                    let key = args[1].clone();
                    let value = eval(args[2].clone(), env)?;
                    match *key {
                        Symbol(_) => {
                            env::set(env, key, value.clone());
                            return Ok(value);
                        },
                        _         => {
                            return err_str("def! with non-symbol as a key");
                        },
                    }
                },
                // (let* (key0 value0 key1 value1 ...) value)
                // evaluate value in a temporary sub-environment where
                // the given (key: Symbol / value: _) pairs are set
                "let*" => {
                    if args.len() != 3 {
                        return err_str("wrong arity for \"let*\", should be 2");
                    }
                    let env_let = env::new(Some(env.clone()));
                    let bindings = args[1].clone();
                    match *bindings {
                        List(ref bindings_seq) => {
                            if bindings_seq.len() % 2 != 0 {
                                return err_str(concat!("missing key or value ",
                                    "in the let* binding list"));
                            }
                            let mut it = bindings_seq.iter();
                            while it.len() >= 2 {
                                let key = it.next().unwrap();
                                let expr = it.next().unwrap();
                                match **key {
                                    Symbol(_) => {
                                        let value = eval(expr.clone(), &env_let)?;
                                        env::set(&env_let, key.clone(), value);
                                    },
                                    _ => return err_str("non-symbol key in the let* binding list"),
                                }
                            }
                        },
                        _ => return err_str("let* with non-list binding"),
                    }
                    return eval(args[2].clone(), &env_let);
                },
        // otherwise : apply the first item to the other
                _ => (),
            }
        },
        None => (),
    }

    let list_ev = eval_ast(ast.clone(), env)?;
    let items = match *list_ev {
        List(ref seq) => seq,
        _             => return types::err_str("can only apply on a list"),
    };
    if items.len() == 0 { return Ok(list_ev.clone()); }
    let ref f = items[0];
    f.apply(items[1..].to_vec())
}

fn print(expr: MalValue) -> String {
    expr.pr_str(true)
}

fn rep(string: &str, env: &env::Env) -> Result<String, MalError> {
    let ast = read(string.into())?;
    let expr = eval(ast, env)?;
    Ok(print(expr))
}

fn int_op<F>(f: F, args: Vec<MalValue>) -> MalResult
    where F: FnOnce(i32, i32) -> i32 {
    if args.len() != 2 {
        return err_string(
            format!("wrong arity ({}) for operation between 2 integers",
                    args.len()));
    }
    match *args[0] {
        Integer(left) => match *args[1] {
            Integer(right) => Ok(new_integer(f(left, right))),
            _ => err_str("right argument must be an integer"),
        },
        _ => err_str("left argument must be an integer")
    }
}
fn add(args: Vec<MalValue>) -> MalResult { int_op(|a, b| { a+b }, args) }
fn sub(args: Vec<MalValue>) -> MalResult { int_op(|a, b| { a-b }, args) }
fn mul(args: Vec<MalValue>) -> MalResult { int_op(|a, b| { a*b }, args) }

fn div(args: Vec<MalValue>) -> MalResult {
    if args.len() != 2 {
        return err_string(format!(
            "wrong arity ({}) for operation between 2 integers", args.len()));
    }
    match *args[0] {
        Integer(left) => match *args[1] {
            Integer(right) => {
                if right == 0 { err_str("cannot divide by 0") }
                else { Ok(types::new_integer(left / right)) }
            },
            _ => err_str("right argument must be an integer"),
        },
        _ => err_str("left argument must be an integer")
    }
}

fn main() {
    // REPL environment
    let repl_env = env::new(None);
    env::set(&repl_env, new_symbol("+".into()), new_function(add, Some(2), "+"));
    env::set(&repl_env, new_symbol("-".into()), new_function(sub, Some(2), "-"));
    env::set(&repl_env, new_symbol("*".into()), new_function(mul, Some(2), "*"));
    env::set(&repl_env, new_symbol("/".into()), new_function(div, Some(2), "/"));

    // REPL
    let prompt = "user> ";
    let mut input = String::new();
    'repl: loop {
        readline::read_line(prompt, &mut input);
        match rep(&input, &repl_env) {
            Ok(result) => println!("{}", result),
            Err(MalError::ErrEmptyLine) => continue,
            Err(MalError::ErrString(why)) => println!("error : {}", why),
        }
    }
}
