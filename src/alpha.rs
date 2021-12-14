use crate::basics::{is_var_name, Core};
use crate::symbol::Symbol;

pub fn is_alpha_equiv(e1: &Core, e2: &Core) -> bool {
    alpha_equiv_aux(0, &Bindings::new(), &Bindings::new(), e1, e2)
}

fn alpha_equiv_aux(lvl: usize, b1: &Bindings, b2: &Bindings, e1: &Core, e2: &Core) -> bool {
    use Core::*;
    match (e1, e2) {
        (U, U) | (Nat, Nat) | (Zero, Zero) | (Atom, Atom) => true,

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

        (The(t1, e1), The(t2, e2)) => {
            alpha_equiv_aux(lvl, b1, b2, t1, t2) && alpha_equiv_aux(lvl, b1, b2, e1, e2)
        }

        (Add1(a), Add1(b)) => alpha_equiv_aux(lvl, b1, b2, a, b),

        (Quote(a), Quote(b)) => a == b,

        (Pi(x, a1, r1), Pi(y, a2, r2)) => {
            alpha_equiv_aux(lvl, b1, b2, a1, a2)
                && alpha_equiv_aux(1 + lvl, &b1.bind(x, lvl), &b2.bind(y, lvl), r1, r2)
        }

        (Lambda(x, body1), Lambda(y, body2)) => {
            alpha_equiv_aux(1 + lvl, &b1.bind(x, lvl), &b2.bind(y, lvl), body1, body2)
        }

        // these should go into a general false case, but i don't want to miss anything important now
        (Atom, Nat) => false,
        (Nat, U) => false,

        _ => todo!("{:?} ?= {:?}", e1, e2),
    }
}

enum Bindings<'a> {
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
