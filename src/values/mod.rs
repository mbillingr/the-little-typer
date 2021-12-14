use crate::basics::{Closure, Core, Env, Value, R};

mod delay;
pub mod functions;
mod natural;
mod universe;

use crate::symbol::Symbol;
use delay::Delay;
use functions::{Lambda, Pi};
use natural::{Add1, Nat, Zero};
use universe::Universe;

pub fn later(env: Env, exp: Core) -> Value {
    Value::Obj(R::new(Delay::new(env, exp)))
}

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

pub fn pi(arg_name: Symbol, arg_type: Value, res_type: Closure) -> Value {
    Value::Obj(R::new(Pi {
        arg_name,
        arg_type,
        res_type,
    }))
}

pub fn lambda(arg_name: Symbol, body: Closure) -> Value {
    Value::Obj(R::new(Lambda { arg_name, body }))
}
