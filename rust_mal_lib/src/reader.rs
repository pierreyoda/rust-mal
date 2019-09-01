/// Module permitting the construction of an Abstract Syntax Tree from an
/// input string.
use regex::Regex;

use super::types;
use super::types::{err_str, MalError, MalResult, MalValue};

pub fn read_str(string: &str) -> MalResult {
    let tokens = tokenize(string);
    if tokens.is_empty() {
        Err(MalError::ErrEmptyLine)
    } else {
        let mut reader = MalReader {
            tokens,
            position: 0,
        };
        read_form(&mut reader)
    }
}

struct MalReader {
    tokens: Vec<String>,
    position: usize,
}

impl MalReader {
    /// Get the token at the current position then increment the position.
    pub fn next(&mut self) -> Option<&str> {
        let token = if self.position < self.tokens.len() {
            Some(&self.tokens[self.position][..])
        } else {
            None
        };
        self.position += 1;
        token
    }

    /// Get the token at the current position.
    pub fn peek(&self) -> Option<&str> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position][..])
        } else {
            None
        }
    }
}

const MATCH_TOKEN_PCRE: &str =
    r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"#;
const MATCH_INTEGER_LITERAL_PCRE: &str = r#"[+-]?\b[0-9]+\b"#;
const MATCH_STRING_LITERAL_PCRE: &str = r#""(?:\\.|[^\\"])*""#;

/// Construct the 'Vec' containing the tokens in the given string.
fn tokenize(string: &str) -> Vec<String> {
    let mut tokens = vec![];
    let re = mal_regex!(MATCH_TOKEN_PCRE);
    for caps in re.captures_iter(string) {
        let group = &caps[1];
        if group == "" {
            break;
        }
        if group.starts_with(';') {
            continue;
        }
        tokens.push(group.to_string());
    }
    tokens
}

/// Try to construct the 'MalValue' corresponding to the current token stored
/// in the given 'MalReader'.
fn read_form(reader: &mut MalReader) -> MalResult {
    match reader.peek().unwrap() {
        ")" => err_str("unexpected ')'"),
        "(" => read_list(reader),
        "]" => err_str("unexpected ']'"),
        "[" => read_vector(reader),
        "}" => err_str("unexpected '}'"),
        "{" => read_hash(reader),
        _ => read_atom(reader),
    }
}

/// Build a scalar 'MalValue' from the consumed token in the given 'MalReader'.
fn read_atom(reader: &mut MalReader) -> MalResult {
    let token = match reader.next() {
        Some(token) => token,
        None => return err_str("read_atom underflow"),
    };
    if token == "nil" {
        Ok(types::new_nil())
    } else if token == "true" {
        Ok(types::new_true())
    } else if token == "false" {
        Ok(types::new_false())
    } else if mal_regex!(MATCH_STRING_LITERAL_PCRE).is_match(token) {
        Ok(types::new_str_from_slice(&token[1..token.len() - 1]))
    } else if mal_regex!(MATCH_INTEGER_LITERAL_PCRE).is_match(token) {
        let integer: i32 = token.parse().ok().unwrap();
        Ok(types::new_integer(integer))
    } else {
        Ok(types::new_symbol(token.to_string()))
    }
}

/// Read a sequence in 'MalReader' beginning and ending with the given
/// symbols, consuming all the used tokens.
fn read_seq(reader: &mut MalReader, start: &str, end: &str) -> Result<Vec<MalValue>, MalError> {
    let oerror = match reader.next() {
        Some(start_token) => {
            if start_token != start {
                Some(format!("expected '{}'", start))
            } else {
                None
            }
        }
        None => Some("read_seq underflow".into()),
    };
    if let Some(error) = oerror {
        return Err(MalError::ErrString(error));
    }
    let mut seq: Vec<MalValue> = vec![];
    loop {
        match reader.peek() {
            Some(token) if token == end => break,
            None => return Err(MalError::ErrString(format!("expected '{}', got EOF", end))),
            _ => match read_form(reader) {
                Ok(value) => seq.push(value),
                Err(why) => return Err(why),
            },
        }
    }
    reader.next();
    Ok(seq)
}

fn read_list(reader: &mut MalReader) -> MalResult {
    read_seq(reader, "(", ")").map(types::new_list)
}

fn read_vector(reader: &mut MalReader) -> MalResult {
    read_seq(reader, "[", "]").map(types::new_vector)
}

fn read_hash(reader: &mut MalReader) -> MalResult {
    let mut map = types::MalHashContainer::new();
    let seq = read_seq(reader, "{", "}")?;
    let mut iter = seq.iter();
    while let Some(key) = iter.next() {
        let k = match **key {
            types::MalType::Str(ref string) => string.clone(),
            _ => return err_str("expected str key for hash map"),
        };
        let v = match iter.next() {
            Some(next) => next,
            None => return err_str("unbalanced hash map (key with no value)"),
        };
        map.insert(k, v.clone());
    }
    Ok(types::new_hash(map))
}
