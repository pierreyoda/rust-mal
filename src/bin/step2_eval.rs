use std::collections::HashMap;

extern crate mal;
use mal::{types, reader, readline};
use mal::types::{MalValue, MalResult, MalError};
use mal::types::MalType::*;

pub type Env = HashMap<String, MalValue>;

fn read(string: &str) -> MalResult {
    reader::read_str(string)
}

fn eval_ast(ast: MalValue, env: &Env) -> MalResult {
    match *ast {
        Symbol(ref symbol) => match env.get(symbol) {
            Some(value) => Ok(value.clone()),
            None => types::err_string(format!("'{}' not found", symbol)),
        },
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

fn eval(ast: MalValue, env: &Env) -> MalResult {
    match *ast {
        List(_) => (),
        _       => return eval_ast(ast, env)
    }

    // ast is a list : apply the first item to the other
    let list_ev = eval_ast(ast.clone(), env)?;
    let items = match *list_ev {
        List(ref seq) => seq,
        _             => return types::err_str("can only apply on a List"),
    };
    if items.len() == 0 { return Ok(list_ev.clone()); }
    let ref f = items[0];
    f.apply(items[1..].to_vec())
}

fn print(expr: MalValue) -> String {
    expr.pr_str(true)
}

fn rep(string: &str, env: &Env) -> Result<String, MalError> {
    let ast = read(string.into())?;
    let expr = eval(ast, env)?;
    Ok(print(expr))
}

fn int_op<F>(f: F, args: Vec<MalValue>) -> MalResult
    where F: FnOnce(i32, i32) -> i32 {
    if args.len() != 2 {
        return types::err_string(
            format!("wrong arity ({}) for operation between 2 integers",
                    args.len()));
    }
    match *args[0] {
        types::MalType::Integer(left) => match *args[1] {
            types::MalType::Integer(right) => Ok(types::new_integer(
                f(left, right))),
            _ => types::err_str("right argument must be an integer"),
        },
        _ => types::err_str("left argument must be an integer")
    }
}
fn add(args: Vec<MalValue>) -> MalResult { int_op(|a, b| { a+b }, args) }
fn sub(args: Vec<MalValue>) -> MalResult { int_op(|a, b| { a-b }, args) }
fn mul(args: Vec<MalValue>) -> MalResult { int_op(|a, b| { a*b }, args) }

fn div(args: Vec<MalValue>) -> MalResult {
    if args.len() != 2 {
        return types::err_string(
            format!("wrong arity ({}) for operation between 2 integers",
                    args.len()));
    }
    match *args[0] {
        types::MalType::Integer(left) => match *args[1] {
            types::MalType::Integer(right) => {
                if right == 0 {
                    types::err_str("cannot divide by 0")
                } else {
                    Ok(types::new_integer(left / right))
                }
            },
            _ => types::err_str("right argument must be an integer"),
        },
        _ => types::err_str("left argument must be an integer")
    }
}

fn main() {
    // REPL environment
    let mut repl_env = Env::new();
    repl_env.insert("+".into(), types::new_function(add, Some(2), "+"));
    repl_env.insert("-".into(), types::new_function(sub, Some(2), "-"));
    repl_env.insert("*".into(), types::new_function(mul, Some(2), "*"));
    repl_env.insert("/".into(), types::new_function(div, Some(2), "/"));

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
