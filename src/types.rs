use std::fmt;
use std::rc::Rc;

use super::env::Env;
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
    MalFunction(MalFunctionData),
}

impl MalType {
    /// If the type defines a function, apply it to the given arguments.
    pub fn apply(&self, args: Vec<MalValue>) -> MalResult {
        match *self {
            Function(ref data) => {
                if data.arity.is_some() && args.len() != data.arity.unwrap() {
                    err_string(format!("wrong arity ({}) for {:?}",
                               args.len(), data))
                } else {
                    (data.function)(args)
                }
            },
            MalFunction(ref data) => {
                let exprs = new_list(args);
                match super::env::bind(&data.env, data.args.clone(), exprs) {
                    Ok(eval_env) => (data.eval)(data.exp.clone(), eval_env),
                    Err(why)     => err_string(why),
                }
            },
            _ => err_str("cannot call a non-function"),
        }
    }
}

impl PartialEq for MalType {
    fn eq(&self, other: &MalType) -> bool {
        match (self, other) {
            (&Nil, &Nil) => true,
            (&True, &True) => true,
            (&False, &False) => true,
            (&Integer(a), &Integer(b)) => a == b,
            (&Str(ref a), &Str(ref b)) => a == b,
            (&Symbol(ref a), &Symbol(ref b)) => a == b,
            (&List(ref a), &List(ref b)) => a == b,
            (&Vector(ref a), &Vector(ref b)) => a == b,
            (&Function(_), &Function(_)) => {
                warn!("cannot compare two functions");
                false
            },
            (&MalFunction(_), &MalFunction(_)) => {
                // data.eval could be ignored
                // but data.env would have to be compared :
                // equality doesn't really make sense
                warn!("cannot compare two functions");
                false
            },
            _ => false,
        }
    }
}

/// Metadata for a native Rust function operating on MAL values.
pub struct FunctionData<'a> {
    /// The Rust evaluating function.
    function : fn(Vec<MalValue>) -> MalResult,
    /// Its arity (the number of MAL values it takes as parameters).
    /// If none, can accept any number of parameters.
    /// Should be at least a minimum bound in the number of parameters,
    /// and optionally can be stricly respected if 'MalType::apply' only
    /// accept an equal number of parameters.
    arity    : Option<usize>,
    /// The name of the function (only a hint used for printing).
    name     : &'a str,
}

impl<'a> fmt::Debug for FunctionData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rust function \"{}\" ({} parameter(s))",
            self.name,
            match self.arity {
                Some(n) => n.to_string(),
                None    => "?".to_string(),
            })
    }
}

/// Metadata for a function defined in MAL (a lambda).
pub struct MalFunctionData {
    /// The Rust function used to evaluate the function body.
    eval: fn(MalValue, Env) -> MalResult,
    /// The function outer environment.
    env: Env,
    /// The function parameters (list of symbols).
    args: MalValue,
    /// The function body.
    exp: MalValue,
}

impl fmt::Debug for MalFunctionData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(fn* {:?} {:?})", self.args, self.exp)
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
pub fn new_function(function : fn(Vec<MalValue>) -> MalResult,
    arity: Option<usize>, name: &'static str) -> MalValue {
    Rc::new(Function(FunctionData {
        function: function,
        arity: arity,
        name: name,
    }))
}
pub fn new_mal_function(eval: fn(MalValue, Env) -> MalResult, env: Env,
    args: MalValue, exp: MalValue) -> MalValue {
    Rc::new(MalFunction(MalFunctionData {
        eval: eval,
        env: env,
        args: args,
        exp: exp,
    }))
}
