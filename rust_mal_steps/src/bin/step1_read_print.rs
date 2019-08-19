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
