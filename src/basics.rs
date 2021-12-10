use crate::symbol::Symbol;
use std::cell::RefCell;
use std::fmt::Formatter;
use std::rc::Rc as R;

#[derive(Debug, Clone, PartialEq)]
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

impl Core {
    pub fn the(t: impl Into<R<Core>>, e: impl Into<R<Core>>) -> Self {
        Core::The(t.into(), e.into())
    }

    pub fn quote(s: impl Into<Symbol>) -> Self {
        Core::Quote(s.into())
    }
}

#[derive(Debug)]
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
        result_type: R<Closure>,
    },
    Lam {
        arg_name: Symbol,
        result_type: R<Closure>,
    },
    Delay(R<RefCell<Delayed>>),
}

#[derive(Debug)]
pub enum Delayed {
    Value(Value),
    Later(Env, Core),
}

pub enum Closure {
    FirstOrder { env: Env, var: Symbol, expr: Core },
    HigherOrder(R<dyn Fn(Value) -> Value>),
}

impl std::fmt::Debug for Closure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub enum Ctx {
    Nil,
    Entry(Symbol, Binder),
}

impl Ctx {
    pub const fn new() -> Self {
        Ctx::Nil
    }
}

pub enum Binder {}

#[derive(Debug)]
pub enum Env {
    Nil,
    Entry(Symbol, Value),
}

pub struct Renaming {}

impl Renaming {
    pub const fn new() -> Self {
        Renaming {}
    }
}

pub fn ctx_to_env(ctx: &Ctx) -> Env {
    match ctx {
        Ctx::Nil => Env::Nil,
        _ => todo!(),
    }
}
