use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::types;
use super::types::{MalValue, MalResult, new_list};
use super::types::MalType::{Symbol, List, Vector};

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

/// Bind the given lists on a key/value basis and return the new environment,
/// with the given environment as its outer.
/// The binding will be variadic if a '&' symbol is encountered in the bindings
/// list ; in this case the next symbol in the bindings list is bound to the
//  rest of the exprs.
pub fn bind(outer: &Env, binds: MalValue, exprs: MalValue) -> Result<Env, String> {
    let env = new(Some(outer.clone()));
    let mut variadic_pos: Option<usize> = None;
    match *binds {
        List(ref binds_seq) | Vector(ref binds_seq) => {
            match *exprs {
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
                                set(&env, bind.clone(), exprs_seq[i].clone());
                            },
                            _ => return Err("non-symbol bind".into()),
                        }
                    }
                    match variadic_pos {
                        Some(i) => {
                            if i >= binds_seq.len() {
                                return Err(concat!("missing a symbol after '&'",
                                           " for variadic binding").into());
                            }
                            let ref vbind = binds_seq[i+1];
                            match **vbind {
                                Symbol(_) => {
                                    set(&env, vbind.clone(),
                                        new_list(exprs_seq[i..].to_vec()));
                                },
                                _ => return Err("non'symbol variadic binding".into()),
                            }
                        },
                        None => (),
                    }
                    Ok(env)
                },
                _ => Err("env : exprs must be a list/vector".into()),
            }
        },
        _ => Err("env : binds must be a list/vector".into()),
    }
}
