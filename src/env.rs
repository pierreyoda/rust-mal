use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::types;
use super::types::{MalValue, MalResult};
use super::types::MalType::Symbol;

struct EnvData {
    data: HashMap<String, MalValue>,
    outer: Option<Env>,
}

/// Handler for an 'EnvData' instance.
pub type Env = Rc<RefCell<EnvData>>;

/// Create a new 'Env' instance with the (optional) outer environment.
pub fn new(outer: Option<Env>) -> Env {
    Rc::new(RefCell::new(EnvData { data: HashMap::new(), outer: outer}))
}

/// Return the root environment of the given 'Env'.
pub fn root(env: &Env) -> Env {
    match env.borrow().outer {
        Some(ref outer) => root(outer),
        None            => env.clone(),
    }
}

/// Return the given environment if it contains the given key (must be a Symbol)
/// or, if any, the first outer environment containing it.
pub fn find(env: &Env, key: &MalValue) -> Option<Env> {
    match **key {
        Symbol(ref symbol) => {
            let env_data = env.borrow();
            if env_data.data.contains_key(symbol) { Some(env.clone()) }
            else {
                match env_data.outer {
                    Some(ref outer_env) => find(outer_env, key),
                    None                => None,
                }
            }
        }
        _ => None,
    }
}

/// Associate the given key (must be a Symbol) with the given MAL value in the
/// given environment.
pub fn set(env: &Env, key: MalValue, val: MalValue) {
    match *key {
        Symbol(ref symbol) => { env.borrow_mut().data.insert(symbol.clone(), val); },
        _                  => warn!("env : cannot set with a non-symbol key"),
    }
}

/// Try to found, if it exists, the value with the associated key (must be a
/// Symbol) in the given environment and, if necessary, in its outer(s).
pub fn get(env: &Env, key: &MalValue) -> MalResult {
    match **key {
        Symbol(ref symbol) => match find(env, key) {
            Some(env) => Ok(env.borrow().data.get(symbol).unwrap().clone()),
            None      => types::err_string(format!("cannot find {}", symbol)),
        },
        _                  => types::err_str("env : cannot get with a non-symbol key"),
    }
}
