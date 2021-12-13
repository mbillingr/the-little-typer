use crate::basics::Core;

pub fn is_alpha_equiv(e1: &Core, e2: &Core) -> bool {
    alpha_equiv_aux(0, &Bindings::new(), &Bindings::new(), e1, e2)
}

type Bindings = Vec<()>;

fn alpha_equiv_aux(lvl: usize, b1: &Bindings, b2: &Bindings, e1: &Core, e2: &Core) -> bool {
    use Core::*;
    match (e1, e2) {
        (U, U) | (Nat, Nat) | (Zero, Zero) | (Atom, Atom) => true,

        (The(t1, e1), The(t2, e2)) => {
            alpha_equiv_aux(lvl, b1, b2, t1, t2) && alpha_equiv_aux(lvl, b1, b2, e1, e2)
        }

        (Quote(a), Quote(b)) => a == b,

        // these should go into a general false case, but i don't want to miss anything important now
        (Atom, Nat) => false,
        (Nat, U) => false,

        _ => todo!("{:?} ?= {:?}", e1, e2),
    }
}
