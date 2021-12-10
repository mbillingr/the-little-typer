use crate::basics::Core;

pub fn is_alpha_equiv(e1: &Core, e2: &Core) -> bool {
    alpha_equiv_aux(0, Bindings::new(), Bindings::new(), e1, e2)
}

type Bindings = Vec<()>;

pub fn alpha_equiv_aux(_lvl: usize, _b1: Bindings, _b2: Bindings, e1: &Core, e2: &Core) -> bool {
    use Core::*;
    match (e1, e2) {
        (U, U) | (Atom, Atom) => true,

        // these should go into a general false case, but i don't want to miss anything important now
        (Atom, Nat) => false,

        _ => todo!("{:?} ?= {:?}", e1, e2),
    }
}
