use rust_mal_lib::{reader, readline, types};

fn read(string: &str) -> types::MalResult {
    reader::read_str(string)
}

fn eval(ast: types::MalValue) -> types::MalResult {
    Ok(ast)
}

fn print(expr: types::MalValue) -> String {
    expr.pr_str(true)
}

fn rep(string: &str) -> Result<String, types::MalError> {
    let ast = read(string)?;
    let expr = eval(ast)?;
    Ok(print(expr))
}

fn main() {
    let prompt = "user> ";
    let mut input = String::new();
    loop {
        readline::read_line(prompt, &mut input);
        match rep(&input) {
            Ok(result) => println!("{}", result),
            Err(types::MalError::ErrEmptyLine) => continue,
            Err(types::MalError::ErrString(why)) => println!("error : {}", why),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rep;
    use rust_mal_lib::env;
    use rust_mal_steps::spec::{checker::check_against_mal_spec, parser::load_and_parse_mal_spec};

    #[test]
    fn test_step1_spec() {
        let lines = load_and_parse_mal_spec("step1_repl.mal").unwrap();
        let env = env::new(None);
        check_against_mal_spec(&lines, env, &|input, _| rep(input)).unwrap();
    }
}
