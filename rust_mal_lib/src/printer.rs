use std::collections::HashMap;
/// Module allowing to display an AST of 'MalValue'.
use std::fmt;

use super::types::MalType::*;
use super::types::{new_str, MalHashContainer, MalValue};

lazy_static! {
    static ref STR_ESCAPED_CHARS_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('"', "\\\"");
        m.insert('\n', "\\n");
        m.insert('\\', "\\\\");
        m
    };
}

impl super::types::MalType {
    pub fn pr_str(&self, print_readably: bool) -> String {
        match *self {
            Nil => "nil".to_string(),
            True => "true".to_string(),
            False => "false".to_string(),
            Integer(integer) => integer.to_string(),
            Str(ref string) => {
                if print_readably {
                    let escaped = string
                        .chars()
                        .map(|c| match STR_ESCAPED_CHARS_MAP.get(&c) {
                            Some(escaped_char) => escaped_char.to_string(),
                            None => c.to_string(),
                        })
                        .collect::<Vec<String>>()
                        .join("");
                    format!("\"{}\"", escaped)
                } else {
                    string.clone()
                }
            }
            Symbol(ref string) => string.clone(),
            List(ref seq) => pr_seq(seq, print_readably, "(", ")", " "),
            Vector(ref seq) => pr_seq(seq, print_readably, "[", "]", " "),
            Hash(ref hash) => pr_hash(hash, print_readably, "{", "}", " "),
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

fn pr_seq(seq: &[MalValue], print_readably: bool, start: &str, end: &str, sep: &str) -> String {
    let mut string = String::new();
    string.push_str(start);

    let mut first = true;
    for value in seq {
        if first {
            first = false;
        } else {
            string.push_str(sep);
        }
        string.push_str(&value.pr_str(print_readably)[..])
    }

    string.push_str(end);
    string
}

fn pr_hash(
    hash: &MalHashContainer,
    print_readably: bool,
    start: &str,
    end: &str,
    sep: &str,
) -> String {
    let list: Vec<MalValue> = hash
        .iter()
        .flat_map(|(k, v)| vec![new_str(k.clone()), v.clone()])
        .collect();
    pr_seq(&list, print_readably, start, end, sep)
}
