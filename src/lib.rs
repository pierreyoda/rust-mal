extern crate regex;
macro_rules! mal_regex { ($re:expr) => { Regex::new($re).unwrap() } }

pub mod printer;
pub mod reader;
pub mod types;
