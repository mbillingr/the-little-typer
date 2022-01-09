use crate::basics::{Core, CoreInterface};
use crate::symbol::Symbol;
use crate::types::cores;
use crate::types::functions::{Fun, Lambda, LambdaStar, PiStar};

pub fn resugar(term: &Core) -> Core {
    term.resugar().1
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

pub fn add_pi(x: Symbol, arg_type: Core, term: Core) -> Core {
    if let Some(PiStar { binders, res_type }) = term.try_as::<PiStar>() {
        let mut params = Vec::with_capacity(binders.len() + 1);
        params.push((x, arg_type));
        params.extend(binders.iter().cloned());
        cores::pi_star(params, res_type.clone())
    } else {
        Core::pi_star(vec![(x, arg_type)], term)
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
