use crate::basics::{Core, CoreInterface};
use crate::symbol::Symbol;
use crate::types::cores;
use crate::types::functions::{Fun, Lambda, LambdaStar};
use std::collections::HashSet;

pub fn resugar(term: &Core) -> Core {
    resugar_(term).1
}

pub fn resugar_(term: &Core) -> (HashSet<Symbol>, Core) {
    term.resugar()
}

pub fn add_lambda(x: Symbol, term: Core) -> Core {
    if let Some(l) = term.try_as::<Lambda<Core>>() {
        cores::lambda_star(vec![x, l.arg_name.clone()], l.body.clone())
    } else if let Some(l) = term.try_as::<LambdaStar>() {
        let mut xs = Vec::with_capacity(l.params.len() + 1);
        xs.push(x);
        xs.extend(l.params.iter().cloned());
        cores::lambda_star(xs, l.body.clone())
    } else {
        Core::lambda(x, term)
    }
}

pub fn add_fun(arg_type: Core, term: Core) -> Core {
    if let Some(Fun(ts)) = term.try_as::<Fun>() {
        let mut types = Vec::with_capacity(ts.len() + 1);
        types.push(arg_type);
        types.extend(ts.iter().cloned());
        cores::fun(types)
    } else {
        Core::fun(vec![arg_type], term)
    }
}
