use crate::basics::Core;
use crate::symbol::Symbol;
use crate::types::cores;
use crate::types::functions::{Fun, Lambda, LambdaStar};
use std::collections::HashSet;

pub fn resugar(term: &Core) -> Core {
    resugar_(term).1
}

pub fn resugar_(term: &Core) -> (HashSet<Symbol>, Core) {
    use Core::*;
    match term {
        Object(obj) => obj.resugar(),
    }
}

pub fn add_lambda(x: Symbol, term: Core) -> Core {
    match term {
        Core::Object(obj) => {
            if let Some(l) = obj.as_any().downcast_ref::<Lambda<Core>>() {
                cores::lambda_star(vec![x, l.arg_name.clone()], l.body.clone())
            } else if let Some(l) = obj.as_any().downcast_ref::<LambdaStar>() {
                let mut xs = Vec::with_capacity(l.params.len() + 1);
                xs.push(x);
                xs.extend(l.params.iter().cloned());
                cores::lambda_star(xs, l.body.clone())
            } else {
                Core::lambda(x, Core::Object(obj))
            }
        }
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
