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
