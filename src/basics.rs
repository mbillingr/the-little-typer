use crate::symbol::Symbol;
use std::rc::Rc as R;

#[derive(Debug, Clone)]
pub enum Core {
    The(R<Core>, R<Core>),
    U,
    Nat,
    Zero,
    Symbol(Symbol),
    Add1(R<Core>),
    Pi(Symbol, R<Core>, R<Core>),
    Lambda(Symbol, R<Core>),
    Atom,
    Quote(Symbol),
}

pub enum Value {
    Universe,
    Nat,
    Zero,
    Add1(R<Value>),
    Quote(Symbol),
    Atom,
    Pi {
        arg_name: Symbol,
        arg_type: R<Value>,
        result_type: Closure,
    },
    Lam {
        arg_name: Symbol,
        result_type: Closure,
    },
}

pub enum Closure {
    FirstOrder { env: Env, var: Symbol, expr: Core },
    HigherOrder(R<dyn Fn(Value) -> Value>),
}

pub struct Env {}
