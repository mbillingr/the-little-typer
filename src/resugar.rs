use crate::basics::{Core, R};
use crate::symbol::Symbol;
use crate::types::cores;
use crate::types::functions::{Fun, Lambda};
use std::collections::HashSet;
use std::sync::Arc;

pub fn resugar(term: &Core) -> Core {
    resugar_(term).1
}

pub fn resugar_(term: &Core) -> (HashSet<Symbol>, Core) {
    use Core::*;
    match term {
        PiStar(bindings, result_type) => match &bindings[..] {
            [(x, arg_type)] => resugar_unary_pi(x, arg_type, result_type),
            _ => todo!(),
        },
        Object(obj) => obj.resugar(),
        any_term => (HashSet::new(), any_term.clone()),
    }
}

fn resugar_unary_pi(
    x: &Symbol,
    arg_type: &Core,
    result_type: &Arc<Core>,
) -> (HashSet<Symbol>, Core) {
    let arg = resugar_(arg_type);
    let res = resugar_(result_type);
    if res.0.contains(x) {
        todo!()
    } else {
        (&arg.0 | &res.0, add_fun(arg.1, res.1))
    }
}

pub fn add_lambda(x: Symbol, term: Core) -> Core {
    match term {
        Core::Object(obj) => {
            if let Some(l) = obj.as_any().downcast_ref::<Lambda<Core>>() {
                Core::LambdaStar(vec![x, l.arg_name.clone()], R::new(l.body.clone()))
            } else {
                Core::lambda(x, Core::Object(obj))
            }
        }
        Core::LambdaStar(mut xs, result) => {
            xs.insert(0, x);
            Core::LambdaStar(xs, result)
        }
        _ => Core::lambda(x, term),
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
