use rust_mal_lib::env::{Env, Environment};
use rust_mal_lib::reader;
use rust_mal_lib::types::{MalError, MalResult, MalValue};

use rust_mal_steps::scaffold::*;

fn read(string: &str) -> MalResult {
    reader::read_str(string)
}

fn eval(ast: MalValue) -> MalResult {
    Ok(ast)
}

fn print(expr: MalValue) -> String {
    expr.pr_str(true)
}

struct Step1ReadPrint;
impl InterpreterScaffold<Env> for Step1ReadPrint {
    const STEP_NAME: &'static str = "step1_read_print";

    fn create_env() -> Result<Env, MalError> {
        Ok(Environment::new(None))
    }

    fn rep(input: &str, _: &Env) -> Result<String, MalError> {
        let ast = read(input)?;
        let expr = eval(ast)?;
        Ok(print(expr))
    }
}

fn main() -> Result<(), String> {
    cli_loop::<Env, Step1ReadPrint>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step1_spec() {
        assert_eq!(
            validate_against_spec::<Env, Step1ReadPrint>("step1_read_print.mal"),
            Ok(())
        );
    }
}
