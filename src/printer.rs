/// Module allowing to display an AST of 'MalValue'.

use std::fmt;

use super::types::MalValue;
use super::types::MalType::*;

impl super::types::MalType {
    pub fn pr_str(&self, print_readably: bool) -> String {
        match *self {
            Nil => "nil".to_string(),
            True => "true".to_string(),
            False => "false".to_string(),
            Integer(integer) => integer.to_string(),
            Str(ref string) => string.clone(),
            Symbol(ref string) => string.clone(),
            List(ref seq) => pr_seq(seq, print_readably, "(", ")", " "),
            Vector(ref seq) => pr_seq(seq, print_readably, "[", "]", " "),
            Function(ref data) => format!("{:?}", data),
            MalFunction(ref data) => format!("{:?}", data),
        }
    }
}

impl fmt::Debug for super::types::MalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pr_str(true))
    }
}

fn pr_seq(seq: &Vec<MalValue>, print_readably: bool,
           start: &str, end: &str, sep: &str) -> String {
    let mut string = String::new();
    string.push_str(start);

    let mut first = true;
    for value in seq {
        if first { first = false; }
        else { string.push_str(sep); }
        string.push_str(&value.pr_str(print_readably)[..])
    }

    string.push_str(end);
    string
}
