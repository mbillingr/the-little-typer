use crate::basics::Core;
use crate::symbol::Symbol;
use std::collections::HashSet;
use std::sync::Arc;

pub fn resugar(term: &Core) -> Core {
    resugar_(term).1
}

fn resugar_(term: &Core) -> (HashSet<Symbol>, Core) {
    use Core::*;
    match term {
        The(t, v) => {
            let t = resugar_(t);
            let v = resugar_(v);
            (&t.0 | &v.0, Core::the(t.1, v.1))
        }
        PiStar(bindings, result_type) => match &bindings[..] {
            [(x, arg_type)] => resugar_unary_pi(x, arg_type, result_type),
            _ => todo!(),
        },
        Pi(x, arg_type, result_type)=> resugar_unary_pi(x, arg_type, result_type),
        any_term => (HashSet::new(), any_term.clone()),
    }
}

fn resugar_unary_pi(x: &Symbol, arg_type: &Core, result_type: &Arc<Core>) -> (HashSet<Symbol>, Core) {
    let arg = resugar_(arg_type);
    let res = resugar_(result_type);
    if res.0.contains(x) {
        todo!()
    } else {
        (&arg.0 | &res.0, add_fun(arg.1, res.1))
    }
}

fn add_fun(arg_type: Core, term: Core) -> Core {
    match term {
        Core::Fun(mut types) => {
            types.insert(0, arg_type);
            Core::Fun(types)
        }
        _ => Core::fun(vec![arg_type], term),
    }
}
