use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::types;
use super::types::MalType::{List, Symbol, Vector};
use super::types::{new_list, MalResult, MalValue};

pub struct EnvData {
    data: HashMap<String, MalValue>,
    outer: Option<Env>,
}

/// Main Trait to interact with a Make A Lisp Environment.
pub trait Environment: Clone + Sized {
    /// Create a new environment with an optional outer.
    fn new(outer: Option<&Self>) -> Self;

    /// Create a new inner environment, i.e. with the current environment as its outer.
    fn new_inner(&self) -> Self;

    /// Return the root outer of this environment (can be itself if no outer).
    ///
    /// TODO: find way to return reference without "reference to temporary value" error
    fn root(&self) -> Self;

    /// Try to found, if it exists, the value with the associated key (must be a
    /// Symbol) in the current environment and, if applicable, in its outer(s).
    fn get_env_value(&self, key: &MalValue) -> MalResult;

    /// Associate the given key (must be a Symbol) with the given MAL value in the
    /// current environment.
    fn set_env_value(&mut self, key: MalValue, val: MalValue) -> &mut Self;
}

/// Handler for an 'EnvData' instance.
pub type Env = Rc<RefCell<EnvData>>;

impl Environment for Env {
    fn new(outer: Option<&Self>) -> Self {
        Rc::new(RefCell::new(EnvData {
            data: HashMap::new(),
            outer: outer.cloned(),
        }))
    }

    fn new_inner(&self) -> Self {
        self::new(Some(self.clone()))
    }

    fn root(&self) -> Self {
        match self.borrow().outer {
            Some(ref outer) => outer.root(),
            None => self.clone(),
        }
    }

    fn get_env_value(&self, key: &MalValue) -> MalResult {
        match **key {
            Symbol(ref symbol) => match find(self, key) {
                Some(env) => Ok(env.borrow().data.get(symbol).unwrap().clone()),
                None => types::err_string(format!("env: cannot find {}", symbol)),
            },
            _ => types::err_str("env: cannot get with a non-symbol key"),
        }
    }

    fn set_env_value(&mut self, key: MalValue, val: MalValue) -> &mut Self {
        match *key {
            Symbol(ref symbol) => {
                self.borrow_mut().data.insert(symbol.clone(), val);
            }
            _ => warn!("env: cannot set with a non-symbol key"),
        }
        self
    }
}

/// Create a new 'Env' instance with the (optional) outer environment.
pub fn new(outer: Option<Env>) -> Env {
    Rc::new(RefCell::new(EnvData {
        data: HashMap::new(),
        outer,
    }))
}

/// Return the root environment of the given 'Env'.
pub fn root(env: &Env) -> Env {
    match env.borrow().outer {
        Some(ref outer) => root(outer),
        None => env.clone(),
    }
}

/// Return the given environment if it contains the given key (must be a Symbol)
/// or, if any, the first outer environment containing it.
pub fn find(env: &Env, key: &MalValue) -> Option<Env> {
    match **key {
        Symbol(ref symbol) => {
            let env_data = env.borrow();
            if env_data.data.contains_key(symbol) {
                Some(env.clone())
            } else {
                match env_data.outer {
                    Some(ref outer_env) => find(outer_env, key),
                    None => None,
                }
            }
        }
        _ => None,
    }
}

/// Bind the given lists on a key/value basis and return the new environment,
/// with the given environment as its outer.
/// The binding will be variadic if a '&' symbol is encountered in the bindings
/// list ; in this case the next symbol in the bindings list is bound to the
///  rest of the exprs.
pub fn bind(outer: &Env, binds: MalValue, exprs: MalValue) -> Result<Env, String> {
    let mut env = new(Some(outer.clone()));
    let mut variadic_pos: Option<usize> = None;
    match *binds {
        List(ref binds_seq) | Vector(ref binds_seq) => match *exprs {
            List(ref exprs_seq) | Vector(ref exprs_seq) => {
                for (i, bind) in binds_seq.iter().enumerate() {
                    match **bind {
                        Symbol(ref bind_key) => {
                            if *bind_key == "&" {
                                variadic_pos = Some(i);
                                break;
                            } else if i >= exprs_seq.len() {
                                return Err("not enough parameters for binding".into());
                            }
                            env.set_env_value(bind.clone(), exprs_seq[i].clone());
                        }
                        _ => return Err("non-symbol bind".into()),
                    }
                }

                if let Some(i) = variadic_pos {
                    if i >= binds_seq.len() {
                        return Err(
                            concat!("missing a symbol after '&'", " for variadic binding").into(),
                        );
                    }
                    let vbind = &binds_seq[i + 1];
                    match **vbind {
                        Symbol(_) => {
                            env.set_env_value(vbind.clone(), new_list(exprs_seq[i..].to_vec()));
                        }
                        _ => return Err("non-symbol variadic binding".into()),
                    }
                }
                Ok(env)
            }
            _ => Err("env: exprs must be a list/vector".into()),
        },
        _ => Err("env: binds must be a list/vector".into()),
    }
}
