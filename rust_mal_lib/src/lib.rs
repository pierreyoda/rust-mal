extern crate regex;
macro_rules! mal_regex {
    ($re:expr) => {
        Regex::new($re).unwrap()
    };
}

#[macro_use]
extern crate log;

pub mod core;
pub mod env;
pub mod printer;
pub mod reader;
pub mod types;

pub mod readline {
    use std::io;
    use std::io::Write;

    pub fn read_line(prompt: &str, input: &mut String) {
        input.clear();
        print!("{}", prompt);
        io::stdout().flush().expect("readline : output error");
        io::stdin()
            .read_line(input)
            .expect("readline : failed to read line");
    }
}
