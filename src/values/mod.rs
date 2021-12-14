use crate::basics::{Closure, Core, Env, Value};

mod delay;
pub mod functions;
mod natural;
mod neutral;
mod universe;
mod atom;

use crate::symbol::Symbol;
use atom::{Atom, Quote};
use delay::Delay;
use functions::{Lambda, Pi};
use natural::{Add1, Nat, Zero};
pub use neutral::neutral;
use universe::Universe;

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

pub fn pi(arg_name: Symbol, arg_type: Value, res_type: Closure) -> Value {
    Value::new(Pi {
        arg_name,
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