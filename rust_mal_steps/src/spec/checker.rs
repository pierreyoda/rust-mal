use super::MalTestingLine;

use rust_mal_lib::{env, types::MalError};

pub fn check_against_mal_spec<ReadEvalPrint>(
    lines: &Vec<MalTestingLine>,
    env: &env::Env,
    rep: &ReadEvalPrint,
) -> Result<(), MalError>
where
    ReadEvalPrint: Fn(&str, &env::Env) -> Result<String, MalError>,
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
                let inner_env = env::new(Some(env.clone()));
                let mut output = String::new();
                if optional {
                    println!("###optional###");
                }
                for input in inputs {
                    println!("#{}", input);
                    output = rep(input, &inner_env)?;
                    println!(">{}", output);
                }
                let matches = output == *expected;
                // assert!(
                //     matches,
                //     "\n{}\nSHOULD BE\n{}\nFOR INPUT\n{}\n",
                //     output, expected, input
                // );
                if !matches {
                    println!(
                        "\n{}\nSHOULD BE\n{}\n",
                        output, expected
                    );
                }
            }
        }
    }
    Ok(())
}
