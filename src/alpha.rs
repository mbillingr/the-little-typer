use crate::basics::{is_var_name, Core};
use crate::symbol::Symbol;

pub fn is_alpha_equiv(e1: &Core, e2: &Core) -> bool {
    alpha_equiv_aux(0, &Bindings::new(), &Bindings::new(), e1, e2)
}

pub fn alpha_equiv_aux(lvl: usize, b1: &Bindings, b2: &Bindings, e1: &Core, e2: &Core) -> bool {
    use Core::*;
    match (e1, e2) {
        (Symbol(x), Symbol(y)) if is_var_name(x) && is_var_name(y) => {
            let x_binding = b1.assv(x);
            let y_binding = b1.assv(x);
            match (x_binding, y_binding) {
                // both bound
                (Some((_, lvlx)), Some((_, lvly))) => lvlx == lvly,
                // both free
                (None, None) => x == y,
                // one bound, one free
                (_, _) => false,
            }
        }

        (Symbol(x), Symbol(y)) => x == y,

        (Object(a), Object(b)) => a.alpha_equiv_aux(&**b, lvl, b1, b2),
        (Object(_), _) | (_, Object(_)) => false,

        _ => todo!("{:?} ?= {:?}", e1, e2),
    }
}

pub enum Bindings<'a> {
    Nil,
    B(&'a Symbol, usize, &'a Bindings<'a>),
}

impl<'a> Bindings<'a> {
    pub fn new() -> Self {
        Bindings::Nil
    }

    pub fn bind(&'a self, x: &'a Symbol, lvl: usize) -> Self {
        Bindings::B(x, lvl, self)
    }

    pub fn assv(&self, x: &Symbol) -> Option<(&'a Symbol, usize)> {
        match self {
            Bindings::Nil => None,
            Bindings::B(y, lvl, _) if x == *y => Some((y, *lvl)),
            Bindings::B(_, _, next) => next.assv(x),
        }
    }
}
