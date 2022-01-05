use crate::basics::{Closure, Core, Env, Value};
use crate::symbol::Symbol;
use crate::types::atom::{Atom, Quote};
use crate::types::delay::Delay;
use crate::types::functions::{Lambda, Pi};
use crate::types::natural::Add1;
use crate::types::natural::{Nat, Zero};
pub use crate::types::neutral::neutral;
use crate::types::pairs::{Cons, Sigma};
use crate::types::universe::Universe;

pub fn later(env: Env, exp: Core) -> Value {
    Value::new(Delay::new(env, exp))
}

pub fn universe() -> Value {
    Value::new(Universe)
}

pub fn nat() -> Value {
    Value::new(Nat)
}

pub fn zero() -> Value {
    Value::new(Zero)
}

pub fn add1(n: Value) -> Value {
    Value::new(Add1(n))
}

pub fn the_nat(n: u64) -> Value {
    let mut out = zero();
    for _ in 0..n {
        out = add1(out);
    }
    out
}

pub fn pi(x: impl Into<Symbol>, arg_type: Value, res_type: Closure) -> Value {
    Value::new(Pi {
        arg_name: x.into(),
        arg_type,
        res_type,
    })
}

pub fn lambda(arg_name: Symbol, body: Closure) -> Value {
    Value::new(Lambda { arg_name, body })
}

pub fn atom() -> Value {
    Value::new(Atom)
}

pub fn quote(s: impl Into<Symbol>) -> Value {
    Value::new(Quote(s.into()))
}

pub fn sigma(x: impl Into<Symbol>, car_type: Value, cdr_type: Closure) -> Value {
    Value::new(Sigma {
        arg_name: x.into(),
        car_type,
        cdr_type,
    })
}

pub fn cons(car: Value, cdr: Value) -> Value {
    Value::new(Cons(car, cdr))
}
