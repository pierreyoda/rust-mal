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
    'repl: loop {
        input.clear();
        print!("user> ");
        io::stdout().flush().ok().expect("output error");
        io::stdin()
            .read_line(input)
            .ok()
            .expect("input : failed to read line");
        println!("{}", rep(input));
    }
}
