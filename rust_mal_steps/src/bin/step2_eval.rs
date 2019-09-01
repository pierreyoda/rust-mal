use std::collections::HashMap;

use rust_mal_lib::types::{MalError, MalResult, MalValue};
use rust_mal_lib::{env::Environment, reader, types};

use rust_mal_steps::scaffold::*;

#[derive(Clone)]
struct FlatEnv {
    data: HashMap<String, MalValue>,
}

impl Environment for FlatEnv {
    fn new(_outer: Option<&Self>) -> Self {
        FlatEnv {
            data: HashMap::new(),
        }
    }

    fn new_inner(&self) -> Self {
        panic!("step2 env: no environment nesting supported");
    }

    fn root(&self) -> Self {
        self.clone()
    }

    fn get_env_value(&self, key: &MalValue) -> MalResult {
        match **key {
            types::MalType::Symbol(ref symbol) => match self.data.get(symbol) {
                Some(value) => Ok(value.clone()),
                None => types::err_string(format!("env: cannot find {}", symbol)),
            },
            _ => types::err_str("env: cannot get with a non-symbol key"),
        }
    }

    fn set_env_value(&mut self, key: MalValue, val: MalValue) -> &mut Self {
        match *key {
            types::MalType::Symbol(ref symbol) => {
                self.data.insert(symbol.clone(), val);
            }
            _ => panic!("step2 env: env: cannot set with a non-symbol key"),
        }
        self
    }
}

fn read(string: &str) -> MalResult {
    reader::read_str(string)
}

fn eval_ast(ast: MalValue, env: &impl Environment) -> MalResult {
    use types::MalType::*;
    match *ast {
        Symbol(ref symbol) => env.get_env_value(&types::new_symbol(symbol.clone())),
        List(ref seq) | Vector(ref seq) => {
            let mut ast_ev = vec![];
            for value in seq {
                ast_ev.push(eval(value.clone(), env)?);
            }
            Ok(match *ast {
                List(_) => types::new_list(ast_ev),
                _ => types::new_vector(ast_ev),
            })
        }
        _ => Ok(ast.clone()),
    }
}

fn eval(ast: MalValue, env: &impl Environment) -> MalResult {
    use types::MalType::*;

    match *ast {
        List(_) => (),
        _ => return eval_ast(ast, env),
    }

    // ast is a list : apply the first item to the other
    let list_ev = eval_ast(ast.clone(), env)?;
    let items = match *list_ev {
        List(ref seq) => seq,
        _ => return types::err_str("can only apply on a List"),
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

fn int_op<F>(f: F, args: Vec<MalValue>) -> MalResult
where
    F: FnOnce(i32, i32) -> i32,
{
    if args.len() != 2 {
        return types::err_string(format!(
            "wrong arity ({}) for operation between 2 integers",
            args.len()
        ));
    }
    match *args[0] {
        types::MalType::Integer(left) => match *args[1] {
            types::MalType::Integer(right) => Ok(types::new_integer(f(left, right))),
            _ => types::err_str("right argument must be an integer"),
        },
        _ => types::err_str("left argument must be an integer"),
    }
}
fn add(args: Vec<MalValue>) -> MalResult {
    int_op(|a, b| a + b, args)
}
fn sub(args: Vec<MalValue>) -> MalResult {
    int_op(|a, b| a - b, args)
}
fn mul(args: Vec<MalValue>) -> MalResult {
    int_op(|a, b| a * b, args)
}

fn div(args: Vec<MalValue>) -> MalResult {
    if args.len() != 2 {
        return types::err_string(format!(
            "wrong arity ({}) for operation between 2 integers",
            args.len()
        ));
    }
    match *args[0] {
        types::MalType::Integer(left) => match *args[1] {
            types::MalType::Integer(right) => {
                if right == 0 {
                    types::err_str("cannot divide by 0")
                } else {
                    Ok(types::new_integer(left / right))
                }
            }
            _ => types::err_str("right argument must be an integer"),
        },
        _ => types::err_str("left argument must be an integer"),
    }
}

struct Step2Eval;
impl InterpreterScaffold<FlatEnv> for Step2Eval {
    const STEP_NAME: &'static str = "step2_eval";

    fn create_env() -> Result<FlatEnv, MalError> {
        let mut env = FlatEnv::new(None);
        env.set_env_value(
            types::new_symbol("+".into()),
            types::new_function(add, Some(2), "+"),
        )
        .set_env_value(
            types::new_symbol("-".into()),
            types::new_function(sub, Some(2), "-"),
        )
        .set_env_value(
            types::new_symbol("*".into()),
            types::new_function(mul, Some(2), "*"),
        )
        .set_env_value(
            types::new_symbol("/".into()),
            types::new_function(div, Some(2), "/"),
        );
        Ok(env)
    }

    fn rep(input: &str, env: &FlatEnv) -> Result<String, MalError> {
        let ast = read(input)?;
        let expr = eval(ast, env)?;
        Ok(print(expr))
    }
}

fn main() -> Result<(), String> {
    cli_loop::<FlatEnv, Step2Eval>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step2_spec() {
        assert_eq!(
            validate_against_spec::<FlatEnv, Step2Eval>("step2_repl.mal"),
            Ok(())
        );
    }
}
