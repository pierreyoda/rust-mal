/// Module defining the core Rust functions made available in MAL.

use std::collections::HashMap;

use super::types;
use super::types::{MalValue, MalResult, err_str, err_string, new_integer,
    new_function, new_true, new_false};
use super::types::MalType::*;

fn eq_q(args: Vec<MalValue>) -> MalResult {
    if *args[0] == *args[1] { Ok(new_true()) } else { Ok(new_false()) }
}

// List operations

/// Create and return the list (args...).
fn list(args: Vec<MalValue>) -> MalResult { Ok(types::new_list(args)) }
/// Return true if the parameter is a list, false otherwise.
fn list_q(args: Vec<MalValue>) -> MalResult {
    match *args[0] {
        List(_) => Ok(new_true()),
        _       => Ok(new_false()),
    }
}

// Sequence operations

/// Return true if the parameter (must be a List/Vector) is empty, false otherwise.
fn empty_q(args: Vec<MalValue>) -> MalResult {
    match *args[0] {
        List(ref seq) | Vector(ref seq) => match seq.len() {
            0 => Ok(new_true()),
            _ => Ok(new_false()),
        },
        _ => err_str("empty? called on non-list/vector"),
    }
}
/// Return the number of items in the List/Vector parameter.
fn count(args: Vec<MalValue>) -> MalResult {
    match *args[0] {
        List(ref seq) | Vector(ref seq) => Ok(new_integer(seq.len() as i32)),
        _ => err_str("count called on non-list/vector"),
    }
}

// Integer operations

fn int_op<F>(f: F, args: Vec<MalValue>) -> MalResult
    where F: FnOnce(i32, i32) -> i32 {
    if args.len() != 2 {
        return err_string(
            format!("wrong arity ({}) for operation between 2 integers",
                    args.len()));
    }
    match *args[0] {
        Integer(left) => match *args[1] {
            Integer(right) => Ok(new_integer(f(left, right))),
            _ => err_str("right argument must be an integer"),
        },
        _ => err_str("left argument must be an integer")
    }
}
fn add(args: Vec<MalValue>) -> MalResult { int_op(|a, b| { a+b }, args) }
fn sub(args: Vec<MalValue>) -> MalResult { int_op(|a, b| { a-b }, args) }
fn mul(args: Vec<MalValue>) -> MalResult { int_op(|a, b| { a*b }, args) }

fn div(args: Vec<MalValue>) -> MalResult {
    if args.len() != 2 {
        return err_string(format!(
            "wrong arity ({}) for operation between 2 integers", args.len()));
    }
    match *args[0] {
        Integer(left) => match *args[1] {
            Integer(right) => {
                if right == 0 { err_str("cannot divide by 0") }
                else { Ok(new_integer(left / right)) }
            },
            _ => err_str("right argument must be an integer"),
        },
        _ => err_str("left argument must be an integer")
    }
}

fn int_cmp<F>(f: F, args: Vec<MalValue>) -> MalResult
    where F: FnOnce(i32, i32) -> bool {
    if args.len() != 2 {
        return err_string(
            format!("wrong arity ({}) for comparison between 2 integers",
                    args.len()));
    }
    match *args[0] {
        Integer(left) => match *args[1] {
            Integer(right) => match f(left, right) {
                true  => Ok(new_true()),
                false => Ok(new_false())
            },
            _ => err_str("right argument must be an integer"),
        },
        _ => err_str("left argument must be an integer")
    }
}
fn lt(args: Vec<MalValue>) -> MalResult { int_cmp(|a, b| { a<b}, args) }
fn gt(args: Vec<MalValue>) -> MalResult { int_cmp(|a, b| { a>b}, args) }
fn lte(args: Vec<MalValue>) -> MalResult { int_cmp(|a, b| { a<=b}, args) }
fn gte(args: Vec<MalValue>) -> MalResult { int_cmp(|a, b| { a>=b}, args) }

/// Helper macro, helps to avoid discrepencies between the function symbol and
/// name in the metadata.
macro_rules! core_function {
    ($hm: ident, $symbol: expr, $function: expr, $arity: expr) => (
        $hm.insert($symbol.to_string(), new_function($function, $arity, $symbol));
    )
}

/// Create and return the (symbol/function) 'HashMap' defining the core MAL
/// environment.
/// NB: if the arity indication is not stricly respected, may led to failed
/// assertions or panics in some functions (e.g. in "list?").
pub fn ns() -> HashMap<String, MalValue> {
    let mut ns = HashMap::new();

    core_function!(ns, "=", eq_q, Some(2));
    // list operations
    core_function!(ns, "list", list, None);
    core_function!(ns, "list?", list_q, Some(1));
    // sequence operations
    core_function!(ns, "empty?", empty_q, Some(1));
    core_function!(ns, "count", count, Some(1));

    // integer operations
    core_function!(ns, "+", add, Some(2));
    core_function!(ns, "-", sub, Some(2));
    core_function!(ns, "*", mul, Some(2));
    core_function!(ns, "/", div, Some(2));
    // integer comparisons
    core_function!(ns, "<", lt, Some(2));
    core_function!(ns, "<=", lte, Some(2));
    core_function!(ns, ">", gt, Some(2));
    core_function!(ns, ">=", gte, Some(2));

    ns
}
