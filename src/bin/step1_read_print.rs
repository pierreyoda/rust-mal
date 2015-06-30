use std::io;
use std::io::Write;

extern crate mal;
use mal::{types, printer, reader};

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
    let ast = try!(read(string.into()));
    let expr = try!(eval(ast));
    Ok(print(expr))
}

fn main() {
    let input = &mut String::new();
    'repl: loop {
        input.clear();
        print!("user> ");
        io::stdout().flush().ok().expect("output error");
        io::stdin().read_line(input)
            .ok().expect("input : failed to read line");
        match rep(input) {
            Ok(result) => println!("{}", result),
            Err(types::MalError::ErrEmptyLine) => continue,
            Err(types::MalError::ErrString(why)) => println!("error : {}", why),
        }
    }

    println!("Hello, world!");
}
