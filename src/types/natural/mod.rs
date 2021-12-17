mod add1;
mod ind_nat;
mod nat;
mod which_nat;
mod zero;

use crate::alpha;
use crate::basics::{occurring_names, Core};
use crate::symbol::Symbol;
pub use add1::Add1;
pub use ind_nat::IndNat;
pub use nat::Nat;
use std::collections::HashSet;
pub use which_nat::WhichNat;
pub use zero::Zero;

#[derive(Debug, Clone, PartialEq)]
enum MaybeTyped {
    Plain(Core),
    The(Core, Core),
}

impl MaybeTyped {
    pub fn occurring_names(&self) -> HashSet<Symbol> {
        match self {
            MaybeTyped::Plain(b) => occurring_names(b),
            MaybeTyped::The(bt, b) => &occurring_names(bt) | &occurring_names(b),
        }
    }

    fn alpha_equiv_aux(
        &self,
        other: &Self,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool {
        match (self, other) {
            (MaybeTyped::Plain(bs1), MaybeTyped::Plain(bs2)) => {
                alpha::alpha_equiv_aux(lvl, b1, b2, bs1, bs2)
            }
            (MaybeTyped::The(bt1, bs1), MaybeTyped::The(bt2, bs2)) => {
                alpha::alpha_equiv_aux(lvl, b1, b2, bt1, bt2)
                    && alpha::alpha_equiv_aux(lvl, b1, b2, bs1, bs2)
            }
            _ => false,
        }
    }
}
