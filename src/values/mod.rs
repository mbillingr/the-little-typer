use crate::basics::{Value, R};

mod natural;
mod universe;

use natural::{Add1, Nat, Zero};
use universe::Universe;

pub fn universe() -> Value {
    Value::Obj(R::new(Universe))
}

pub fn nat() -> Value {
    Value::Obj(R::new(Nat))
}

pub fn zero() -> Value {
    Value::Obj(R::new(Zero))
}

pub fn add1(n: Value) -> Value {
    Value::Obj(R::new(Add1(n)))
}
