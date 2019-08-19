/// Module permitting the construction of an Abstract Syntax Tree from an
/// input string.

use regex::Regex;

use super::types;
use super::types::{MalValue, MalError, MalResult, err_str};

pub fn read_str(string: &str) -> MalResult {
    let tokens = tokenize(string);
    if tokens.len() == 0 {
        Err(MalError::ErrEmptyLine)
    } else {
        let mut reader = MalReader { tokens: tokens, position: 0 };
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

const MATCH_TOKEN_PCRE: &'static str =
    r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"#;

/// Construct the 'Vec' containing the tokens in the given string.
fn tokenize(string: &str) -> Vec<String> {
    let mut tokens = vec!();
    let re = mal_regex!(MATCH_TOKEN_PCRE);
    for caps in re.captures_iter(string) {
        let group = &caps[1];
        if group == "" { break; }
        if group.starts_with(";") { continue; }
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
         _  => read_atom(reader),
    }
}

/// Build a scalar 'MalValue' from the consumed token in the given 'MalReader'.
fn read_atom(reader: &mut MalReader) -> MalResult {
    let token = match reader.next() {
        Some(token) => token,
        None        => return err_str("read_atom underflow"),
    };
    if token == "nil" { Ok(types::new_nil()) }
    else if token == "true" { Ok(types::new_true()) }
    else if token == "false" { Ok(types::new_false()) }
    else if mal_regex!(r"[+-]?\b[0-9]+\b").is_match(token) {
        let integer: i32 = token.parse().ok().unwrap();
        Ok(types::new_integer(integer))
    } else { Ok(types::new_symbol(token.to_string())) }
}

/// Read a sequence in 'MalReader' beginning and ending with the given
/// symbols, consuming all the used tokens.
fn read_seq(reader: &mut MalReader, start: &str, end: &str) ->
    Result<Vec<MalValue>, MalError> {
    let oerror = match reader.next() {
        Some(start_token) => {
            if start_token != start { Some(format!("expected '{}'", start)) }
                else { None }
        },
        None              => Some(format!("read_seq underflow")),
    };
    if !oerror.is_none() { return Err(MalError::ErrString(oerror.unwrap())) }
    let mut seq: Vec<MalValue> = vec!();
    loop {
        {
            let otoken = reader.peek();
            if otoken.is_none() {
                return Err(MalError::ErrString(
                    format!("expected '{}', got EOF", end)));
            } else if otoken.unwrap() == end { break; }
        }
        match read_form(reader) {
            Ok (value) => seq.push(value),
            Err(error) => return Err(error),
        }
    }
    reader.next();
    Ok(seq)
}

fn read_list(reader: &mut MalReader) -> MalResult {
    read_seq(reader, "(", ")").map(|seq| types::new_list(seq))
}

fn read_vector(reader: &mut MalReader) -> MalResult {
    read_seq(reader, "[", "]").map(|seq| types::new_vector(seq))
}
