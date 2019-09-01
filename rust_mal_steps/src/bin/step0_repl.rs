use rust_mal_lib::env::{Env, Environment};
use rust_mal_lib::types::MalError;

use rust_mal_steps::scaffold::*;

fn read(string: String) -> String {
    string
}

fn eval(ast: String) -> String {
    ast
}

fn print(expr: String) -> String {
    expr
}

struct Step0Repl;
impl InterpreterScaffold<Env> for Step0Repl {
    const STEP_NAME: &'static str = "step0_repl";

    fn create_env() -> Result<Env, MalError> {
        Ok(Environment::new(None))
    }

    fn rep(input: &str, _: &Env) -> Result<String, MalError> {
        if input.is_empty() {
            Err(MalError::ErrEmptyLine)
        } else {
            Ok(print(eval(read(input.into()))))
        }
    }
}

fn main() -> Result<(), String> {
    cli_loop::<Env, Step0Repl>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step0_spec() {
        assert_eq!(
            validate_against_spec::<Env, Step0Repl>("step0_repl.mal"),
            Ok(())
        );
    }
}
