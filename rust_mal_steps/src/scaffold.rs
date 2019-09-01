use rustyline::{error::ReadlineError, Editor};

use rust_mal_lib::env::Environment;
use rust_mal_lib::types::MalError;

use crate::spec::{checker::check_against_mal_spec, parser::load_and_parse_mal_spec};

/// Scaffolding trait used to both produce a CLI-based REPL for the each Make A Lisp
/// step and convenience testing facilities.
pub trait InterpreterScaffold<E: Environment> {
    const STEP_NAME: &'static str;

    /// Create the initial Environment for the REPL.
    fn create_env() -> Result<E, MalError>;

    /// Read-Eval-Print the given input.
    fn rep(input: &str, env: &E) -> Result<String, MalError>;
}

/// Launch a Read-Eval-Print Loop.
pub fn cli_loop<E, S>() -> Result<(), String>
where
    E: Environment,
    S: InterpreterScaffold<E>,
{
    let repl_env = S::create_env().map_err(|err| format!("{:?}", err))?;
    let mut rl = Editor::<()>::new();
    let rl_history = format!("history-{}.txt", S::STEP_NAME);
    let _ = rl.load_history(&rl_history);
    let prompt = format!("{} >>", S::STEP_NAME);
    loop {
        match rl.readline(&prompt) {
            Ok(input) => {
                match S::rep(&input, &repl_env) {
                    Ok(result) => println!("{}", result),
                    Err(MalError::ErrEmptyLine) => continue,
                    Err(MalError::ErrString(why)) => println!("error: {}", why),
                }
                rl.add_history_entry(&input);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("readline error: {}", err);
                break;
            }
        }
    }
    rl.save_history(&rl_history)
        .map_err(|err| format!("cannot save history: {}", err))
}

pub fn validate_against_spec<E, S>(filename: &str) -> Result<(), String>
where
    E: Environment,
    S: InterpreterScaffold<E>,
{
    println!("########## validating {}... ##########", S::STEP_NAME);
    let lines = load_and_parse_mal_spec(filename)?;
    let env = S::create_env().map_err(|err| format!("{:?}", err))?;
    check_against_mal_spec(&lines, env, &|input, env| S::rep(input, env))
        .map_err(|err| format!("{:?}", err))
}
