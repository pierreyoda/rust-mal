use rust_mal_lib::types::MalType::*;
use rust_mal_lib::types::{
    err_str, new_list, new_mal_function, new_symbol, MalError, MalResult, MalValue, new_function, new_str
};
use rust_mal_lib::{core, env, reader, readline, types};

fn read(string: &str) -> MalResult {
    reader::read_str(string)
}

fn eval_ast(ast: MalValue, env: &env::Env) -> MalResult {
    match *ast {
        Symbol(_) => env::get(&env, &ast),
        List(ref seq) | Vector(ref seq) => {
            let mut ast_ev = vec![];
            for value in seq {
                ast_ev.push(eval(value.clone(), env.clone())?);
            }
            Ok(match *ast {
                List(_) => types::new_list(ast_ev),
                _ => types::new_vector(ast_ev),
            })
        }
        _ => Ok(ast.clone()),
    }
}

fn eval(ast: MalValue, env: env::Env) -> MalResult {
    let ast_temp = ast.clone();
    let (arg0_symbol, args): (Option<&str>, &Vec<MalValue>) = match *ast_temp {
        List(ref seq) => {
            if seq.is_empty() {
                return Ok(ast);
            }
            match *seq[0] {
                Symbol(ref symbol) => (Some(&symbol[..]), seq),
                _ => (None, seq),
            }
        }
        _ => return eval_ast(ast, &env),
    };

    if let Some(slice) = arg0_symbol {
        match slice {
            // (do items...) : evaluate all items and return the last one
            "do" => match *eval_ast(new_list(args[1..].to_vec()), &env)? {
                List(ref seq) => return Ok(seq[seq.len() - 1].clone()),
                _ => return err_str("invalid do call"),
            },
            // (if condition if_condition_not_nil_or_false otherwise)
            // if 'otherwise' is not provided, return nil if 'condition'
            // evaluates to nil or false
            "if" => {
                if args.len() < 3 || args.len() > 4 {
                    return err_str("wrong arity for if, should be 3 or 4");
                }
                match *eval(args[1].clone(), env.clone())? {
                    False | Nil => {
                        return if args.len() == 4 {
                            eval(args[3].clone(), env.clone())
                        } else {
                            Ok(types::new_nil())
                        }
                    }
                    _ => return eval(args[2].clone(), env.clone()),
                }
            }
            // (def! key value) ; key must be a Symbol
            // bind the evaluated value in env with the unevaluated key
            "def!" => {
                if args.len() != 3 {
                    return err_str("wrong arity for def!, should be 2");
                }
                let key = args[1].clone();
                let value = eval(args[2].clone(), env.clone())?;
                match *key {
                    Symbol(_) => {
                        env::set(&env, key, value.clone());
                        return Ok(value);
                    }
                    _ => {
                        return err_str("def! with non-symbol as a key");
                    }
                }
            }
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
                            return err_str(concat!(
                                "missing key or value ",
                                "in the let* binding list"
                            ));
                        }
                        let mut it = bindings_seq.iter();
                        while it.len() >= 2 {
                            let key = it.next().unwrap();
                            let expr = it.next().unwrap();
                            match **key {
                                Symbol(_) => {
                                    let value = eval(expr.clone(), env_let.clone())?;
                                    env::set(&env_let, key.clone(), value);
                                }
                                _ => return err_str("non-symbol key in the let* binding list"),
                            }
                        }
                    }
                    _ => return err_str("let* with non-list binding"),
                }
                return eval(args[2].clone(), env_let.clone());
            }
            // (fn* (args...) exp)
            "fn*" => {
                if args.len() != 3 {
                    return err_str("wrong arity for fn*, should be 2");
                }
                let fn_args = args[1].clone();
                match *fn_args {
                    List(_) => (),
                    _ => return err_str("fn* with non-list arguments"),
                }
                return Ok(new_mal_function(
                    self::eval,
                    env.clone(),
                    args[1].clone(),
                    args[2].clone(),
                ));
            }
            // otherwise : apply the first item to the other
            _ => (),
        }
    }

    let list_ev = eval_ast(ast.clone(), &env)?;
    let items = match *list_ev {
        List(ref seq) => seq,
        _ => return err_str("can only apply on a list"),
    };
    if items.is_empty() {
        return Ok(list_ev.clone());
    }
    let f = &items[0];
    f.apply(items[1..].to_vec())
}

fn print(expr: MalValue) -> String {
    expr.pr_str(true)
}

fn rep(string: &str, env: &env::Env) -> Result<String, MalError> {
    let ast = read(string)?;
    let expr = eval(ast, env.clone())?;
    Ok(print(expr))
}

fn create_repl_env() -> Result<env::Env, MalError> {
    let repl_env = env::new(None);
    for (symbol_string, core_function_value) in core::ns() {
        env::set(&repl_env, new_symbol(symbol_string), core_function_value);
    }
    // let prn = |v: Vec<MalValue>| Ok(print(v[0]));
    env::set(&repl_env, new_symbol("prn".into()), new_function(|v: Vec<MalValue>| Ok(new_str(print(v[0].clone()))), Some(1), "prn"));
    rep("(def! not (fn* (x) (if x false true)))", &repl_env)?;
    Ok(repl_env)
}

fn main() {
    let prompt = "user> ";
    let mut input = String::new();
    let repl_env = create_repl_env().unwrap();
    loop {
        readline::read_line(prompt, &mut input);
        match rep(&input, &repl_env) {
            Ok(result) => println!("{}", result),
            Err(MalError::ErrEmptyLine) => continue,
            Err(MalError::ErrString(why)) => println!("error : {}", why),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{create_repl_env, rep};
    use rust_mal_steps::spec::{checker::check_against_mal_spec, parser::load_and_parse_mal_spec};

    #[test]
    fn test_step4_spec() {
        let lines = load_and_parse_mal_spec("step4_repl.mal").unwrap();
        let env = create_repl_env().unwrap();
        check_against_mal_spec(&lines, &env, &rep).unwrap();
    }
}
