use std::fmt;
use std::rc::Rc;

use self::MalType::*;

/// The different types a MAL value can take.
#[allow(non_camel_case_types)]
pub enum MalType {
    Nil,
    True,
    False,
    Integer(i32),
    Str(String),
    Symbol(String),
    List(Vec<MalValue>),
    Vector(Vec<MalValue>),
    Function(FunctionData<'static>),
}

impl MalType {
    /// If the type defines a function, apply it to the given arguments.
    pub fn apply(&self, args: Vec<MalValue>) -> MalResult {
        match *self {
            Function(ref data) => (data.function)(args),
            _ => err_str("cannot call a non-function"),
        }
    }
}

/// Metadata representing a native Rust function operating on MAL values.
pub struct FunctionData<'a> {
    function : fn(Vec<MalValue>) -> MalResult,
    arity    : usize,
    name     : &'a str,
}

impl<'a> fmt::Debug for FunctionData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rust function \"{}\" ({} arguments)",
            self.name, self.arity)
    }
}

/// A reference-counted MAL value.
pub type MalValue = Rc<MalType>;

#[derive(Debug)]
pub enum MalError {
    ErrString(String),
    ErrEmptyLine,
}

/// Frequently used return type for functions dealing with MAL values.
pub type MalResult = Result<MalValue, MalError>;
pub fn err_str(error: &str) -> MalResult { Err(MalError::ErrString(error.into())) }
pub fn err_string(error: String) -> MalResult { Err(MalError::ErrString(error)) }

pub fn new_nil() -> MalValue { Rc::new(Nil) }
pub fn new_true() -> MalValue { Rc::new(True) }
pub fn new_false() -> MalValue { Rc::new(False) }
pub fn new_integer(integer: i32) -> MalValue { Rc::new(Integer(integer)) }
pub fn new_str(string: String) -> MalValue { Rc::new(Str(string)) }
pub fn new_str_from_slice(slice: &str) -> MalValue { Rc::new(Str(slice.into())) }
pub fn new_symbol(symbol: String) -> MalValue { Rc::new(Symbol(symbol)) }
pub fn new_list(seq: Vec<MalValue>) -> MalValue { Rc::new(List(seq)) }
pub fn new_vector(seq: Vec<MalValue>) -> MalValue { Rc::new(Vector(seq)) }
pub fn new_function(function : fn(Vec<MalValue>) -> MalResult, arity: usize,
    name: &'static str) -> MalValue {
    Rc::new(Function(FunctionData {
        function: function,
        arity: arity,
        name: name,
    }))
}
