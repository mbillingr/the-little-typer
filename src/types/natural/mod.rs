mod add1;
mod ind_nat;
mod nat;
mod which_nat;
mod zero;

use crate::basics::Core;
pub use add1::Add1;
pub use ind_nat::IndNat;
pub use nat::Nat;
pub use which_nat::WhichNat;
pub use zero::Zero;

#[derive(Debug, Clone, PartialEq)]
enum MaybeTyped {
    Plain(Core),
    The(Core, Core),
}
