use std::io;
use std::io::Write;

fn read(string: String) -> String {
    string
}

fn eval(ast: String) -> String {
    ast
}

fn print(expr: String) -> String {
    expr
}

fn rep(string: &str) -> String {
    print(eval(read(string.to_string())))
}

fn main() {
    let input = &mut String::new();
    loop {
        input.clear();
        print!("user> ");
        io::stdout().flush().expect("output error");
        io::stdin()
            .read_line(input)
            .expect("input : failed to read line");
        println!("{}", rep(input));
    }
}

#[cfg(test)]
mod tests {
    use super::rep;
    use rust_mal_lib::env;
    use rust_mal_steps::spec::{checker::check_against_mal_spec, parser::load_and_parse_mal_spec};

    #[test]
    fn test_step0_spec() {
        let lines = load_and_parse_mal_spec("step0_repl.mal").unwrap();
        let env = env::new(None);
        check_against_mal_spec(&lines, env, &|input, _| Ok(rep(input))).unwrap();
    }
}
