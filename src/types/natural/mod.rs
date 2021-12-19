mod add1;
mod ind_nat;
mod nat;
mod which_nat;
mod zero;

use crate::alpha;
use crate::basics::{Core, CoreInterface};
use crate::symbol::Symbol;
pub use add1::Add1;
pub use ind_nat::IndNat;
pub use nat::Nat;
use std::collections::HashSet;
pub use which_nat::{NeutralWhichNat, WhichNat};
pub use zero::Zero;

#[derive(Debug, Clone, PartialEq)]
enum MaybeTyped {
    Plain(Core),
    The(Core, Core),
}

impl MaybeTyped {
    pub fn occurring_names(&self) -> HashSet<Symbol> {
        match self {
            MaybeTyped::Plain(b) => b.occurring_names(),
            MaybeTyped::The(bt, b) => &bt.occurring_names() | &b.occurring_names(),
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

    fn resugar(&self) -> (HashSet<Symbol>, Self) {
        match self {
            MaybeTyped::Plain(b) => {
                let term = b;
                let b = term.resugar();
                (b.0, MaybeTyped::Plain(b.1))
            }
            MaybeTyped::The(bt, b) => {
                let term = bt;
                let bt = term.resugar();
                let term = b;
                let b = term.resugar();
                (&bt.0 | &b.0, MaybeTyped::The(bt.1, b.1))
            }
        }
    }
}
