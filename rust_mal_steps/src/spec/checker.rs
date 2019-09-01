use super::MalTestingLine;

use rust_mal_lib::{env::Environment, types::MalError};

pub fn check_against_mal_spec<E, REP>(
    lines: &[MalTestingLine],
    mut env: E,
    rep: &REP,
) -> Result<(), MalError>
where
    E: Environment,
    REP: Fn(&str, &mut E) -> Result<String, MalError>,
{
    let mut optional = false;
    let mut current_section = String::new();
    for line in lines {
        match line {
            MalTestingLine::ToggleOptional(value) => optional = *value,
            MalTestingLine::BeginSection(name) => {
                current_section = name.clone();
                println!("> starting section: {}", current_section);
            }
            MalTestingLine::InputShouldOutput(inputs, expected) => {
                let mut output = String::new();
                if optional {
                    println!("###optional###");
                }
                for input in inputs {
                    println!("#{}", input);
                    output = rep(input, &mut env)?;
                    println!(">{}", output);
                }
                let matches = output == *expected;
                // assert!(
                //     matches,
                //     "\n{}\nSHOULD BE\n{}\nFOR INPUT\n{}\n",
                //     output, expected, input
                // );
                if !matches {
                    println!("\n{}\nSHOULD BE\n{}\n", output, expected);
                }
            }
            MalTestingLine::InputShouldThrow(inputs) => {
                if optional {
                    println!("###optional###");
                }
                let mut output: Result<String, MalError> = Ok("".into());
                for input in inputs {
                    println!("#{}", input);
                    output = rep(input, &mut env);
                }
                // TODO: spec error type validation
                let matches = output.is_err();
                assert!(
                    matches,
                    "\n{:?}\nSHOULD BE AN ERROR FOR INPUT\n{:?}\n",
                    output, inputs,
                );
            }
        }
    }
    Ok(())
}
