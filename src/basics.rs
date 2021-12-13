use crate::errors::{Error, Result};
use crate::fresh::freshen;
use crate::normalize::val_of;
use crate::sexpr::Sexpr;
use crate::symbol::Symbol;
use sexpr_parser::parse;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;
use std::sync::MutexGuard;
pub use std::sync::{Arc as R, Mutex, RwLock};

#[derive(Debug, Clone, PartialEq)]
pub enum Core {
    The(R<Core>, R<Core>),
    U,
    Nat,
    Zero,
    Symbol(Symbol),
    Add1(R<Core>),
    Fun(Vec<Core>),
    Pi(Symbol, R<Core>, R<Core>),
    Lambda(Symbol, R<Core>),
    Atom,
    Quote(Symbol),
}

impl Core {
    pub fn the(t: impl Into<R<Core>>, e: impl Into<R<Core>>) -> Self {
        Core::The(t.into(), e.into())
    }

    pub fn fun(arg_types: Vec<Core>, ret_type: Core) -> Self {
        assert!(arg_types.len() > 0);
        let mut types = arg_types;
        types.push(ret_type);
        Core::Fun(types)
    }

    pub fn pi(x: impl Into<Symbol>, xt: impl Into<R<Core>>, rt: impl Into<R<Core>>) -> Self {
        Self::Pi(x.into(), xt.into(), rt.into())
    }

    pub fn lambda(x: impl Into<Symbol>, body: impl Into<R<Core>>) -> Self {
        Self::Lambda(x.into(), body.into())
    }

    pub fn symbol(s: impl Into<Symbol>) -> Self {
        Core::Symbol(s.into())
    }

    pub fn quote(s: impl Into<Symbol>) -> Self {
        Core::Quote(s.into())
    }

    pub fn nat(mut x: u64) -> Self {
        let mut n = Core::Zero;
        while x > 0 {
            x -= 1;
            n = Core::Add1(R::new(n))
        }
        n
    }
}

impl Display for Core {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Core::*;
        match self {
            The(t, v) => write!(f, "(the {} {})", t, v),
            U => write!(f, "U"),
            Nat => write!(f, "Nat"),
            Zero => write!(f, "Zero"),
            Symbol(s) => write!(f, "{}", s.name()),
            Add1(n) => write!(f, "(add1 {})", n),
            Fun(ts) => {
                write!(f, "(->")?;
                for t in &**ts {
                    write!(f, " {}", t)?;
                }
                write!(f, ")")
            }
            Pi(param, pt, rt) => write!(f, "(Π (({} {})) {})", param.name(), pt, rt),
            Lambda(param, body) => write!(f, "(λ ({}) {})", param.name(), body),
            Atom => write!(f, "Atom"),
            Quote(s) => write!(f, "'{}", s.name()),
        }
    }
}

impl FromStr for Core {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let sexpr = parse::<Sexpr>(s).map_err(|e| format!("{:?}", e))?;
        Ok((&sexpr).into())
    }
}

impl From<&Sexpr> for Core {
    fn from(sexpr: &Sexpr) -> Self {
        match sexpr {
            Sexpr::Symbol(s) => match s.name() {
                "U" => Core::U,
                "Nat" => Core::Nat,
                "Atom" => Core::Atom,

                "x" | "y" | "z" => Core::Symbol(s.clone()),
                name => todo!("{}", name),
            },
            Sexpr::SmallNat(x) => Core::nat(*x),
            Sexpr::List(list) => match &list[..] {
                [Sexpr::Symbol(s), args @ ..] => match (s.name(), args) {
                    ("the", [t, v]) => Core::the(Core::from(t), Core::from(v)),
                    ("quote", [Sexpr::Symbol(s)]) => Core::quote(s.clone()),
                    ("->", [ts @ .., rt]) => {
                        Core::fun(ts.iter().map(Core::from).collect(), Core::from(rt))
                    }
                    ("lambda", [Sexpr::List(params), body]) => match &params[..] {
                        [Sexpr::Symbol(x)] => Core::lambda(x.clone(), Core::from(body)),
                        _ => todo!(),
                    },
                    (key, _) => todo!("{}", key),
                },
                _ => unimplemented!("{:?}", list),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
        body: R<Closure>,
    },
    Neu(R<Value>, N),
    Delay(SharedBox<Delayed>),
}

impl Value {
    pub fn pi(
        arg_name: Symbol,
        arg_type: impl Into<R<Value>>,
        result_type: impl Into<R<Closure>>,
    ) -> Self {
        Value::Pi {
            arg_name,
            arg_type: arg_type.into(),
            result_type: result_type.into(),
        }
    }

    pub fn lam(arg_name: Symbol, body: impl Into<R<Closure>>) -> Self {
        Value::Lam {
            arg_name,
            body: body.into(),
        }
    }

    pub fn neu(t: impl Into<R<Value>>, neutral: N) -> Self {
        Value::Neu(t.into(), neutral)
    }
}

#[derive(Debug, PartialEq)]
pub enum Delayed {
    Value(Value),
    Later(Env, Core),
}

pub enum Closure {
    FirstOrder { env: Env, var: Symbol, expr: Core },
    HigherOrder(R<dyn Sync + Send + Fn(Value) -> Value>),
}

impl Closure {
    pub fn val_of(&self, v: Value) -> Value {
        match self {
            Closure::FirstOrder { env, var, expr } => val_of(&env.extend(var.clone(), v), expr),
            Closure::HigherOrder(fun) => fun(v),
        }
    }
}

impl std::fmt::Debug for Closure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<CLOSURE>")
    }
}

impl std::cmp::PartialEq for Closure {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum N {
    Var(Symbol),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ctx(R<CtxImpl>);

#[derive(Debug, PartialEq)]
pub enum CtxImpl {
    Nil,
    Entry(Symbol, Binder, Ctx),
}

impl Ctx {
    pub fn new() -> Self {
        Ctx(R::new(CtxImpl::Nil))
    }

    pub fn bind_free(&self, x: Symbol, tv: Value) -> Result<Self> {
        if self.0.assv(&x).is_some() {
            Err(Error::AlreadyBound(x.clone(), self.clone()))
        } else {
            Ok(Ctx(R::new(CtxImpl::Entry(
                x,
                Binder::Free(tv),
                self.clone(),
            ))))
        }
    }

    pub fn names_only(&self) -> HashSet<Symbol> {
        match &*self.0 {
            CtxImpl::Nil => HashSet::new(),
            CtxImpl::Entry(name, _, next) => {
                let mut names = next.names_only();
                names.insert(name.clone());
                names
            }
        }
    }

    pub fn var_type(&self, x: &Symbol) -> Result<Value> {
        match &*self.0 {
            CtxImpl::Nil => Err(Error::UnknownVariable(x.clone())),
            CtxImpl::Entry(_, Binder::Claim(_), next) => next.var_type(x),
            CtxImpl::Entry(y, b, _) if x == y => Ok(b.get_type()),
            CtxImpl::Entry(_, _, next) => next.var_type(x),
        }
    }
}

impl CtxImpl {
    fn assv(&self, x: &Symbol) -> Option<(&Symbol, &Binder)> {
        match self {
            CtxImpl::Nil => None,
            CtxImpl::Entry(s, b, _) if s == x => Some((s, b)),
            CtxImpl::Entry(_, _, next) => next.0.assv(x),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Binder {
    Claim(Value),
    Def(Value, Value),
    Free(Value),
}

impl Binder {
    pub fn get_type(&self) -> Value {
        match self {
            Binder::Claim(tv) | Binder::Def(tv, _) | Binder::Free(tv) => tv.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Env(HashMap<Symbol, Value>);

impl Env {
    pub fn new() -> Self {
        Env(HashMap::new())
    }

    pub fn extend(&self, x: Symbol, v: Value) -> Self {
        let mut m = self.0.clone();
        m.insert(x, v);
        Env(m)
    }

    pub fn var_val(&self, x: &Symbol) -> Result<Value> {
        match self.0.get(x) {
            None => Err(Error::UnknownVariable(x.clone())),
            Some(v) => Ok(v.clone()),
        }
    }
}

pub struct Renaming {
    map: HashMap<Symbol, Symbol>,
}

impl Renaming {
    pub fn new() -> Self {
        Renaming {
            map: HashMap::new(),
        }
    }

    pub fn extend(&self, from: Symbol, to: Symbol) -> Self {
        let mut map = self.map.clone();
        map.insert(from, to);
        Renaming { map }
    }

    pub fn rename(&self, x: &Symbol) -> Symbol {
        match self.map.get(x) {
            None => x.clone(),
            Some(y) => y.clone(),
        }
    }
}

pub fn ctx_to_env(ctx: &Ctx) -> Env {
    match &*ctx.0 {
        CtxImpl::Nil => Env::new(),
        CtxImpl::Entry(x, Binder::Def(_, v), next) => {
            let mut env = ctx_to_env(next);
            env.0.insert(x.clone(), v.clone());
            env
        }
        CtxImpl::Entry(x, Binder::Free(tv), next) => {
            let mut env = ctx_to_env(next);
            env.0
                .insert(x.clone(), Value::Neu(R::new(tv.clone()), N::Var(x.clone())));
            env
        }
        CtxImpl::Entry(_, Binder::Claim(_), next) => ctx_to_env(next),
    }
}

pub fn fresh(ctx: &Ctx, x: &Symbol) -> Symbol {
    freshen(&ctx.names_only(), x)
}

pub fn fresh_binder(ctx: &Ctx, expr: &Core, x: &Symbol) -> Symbol {
    freshen(&(&ctx.names_only() | &occurring_names(expr)), x)
}

pub fn occurring_names(expr: &Core) -> HashSet<Symbol> {
    match expr {
        Core::Fun(types) => {
            let mut names = HashSet::new();
            for t in types {
                names = &names | &occurring_names(t)
            }
            names
        }
        Core::Atom => HashSet::new(),
        _ => todo!("{:?}", expr),
    }
}

#[derive(Debug)]
pub struct SharedBox<T>(R<Mutex<T>>);

pub struct SharedBoxGuard<'a, T: 'a>(MutexGuard<'a, T>);

impl<T> Clone for SharedBox<T> {
    fn clone(&self) -> Self {
        SharedBox(self.0.clone())
    }
}

impl<T> SharedBox<T> {
    pub fn new(inner: T) -> Self {
        SharedBox(R::new(Mutex::new(inner)))
    }

    pub fn write_lock(&self) -> SharedBoxGuard<T> {
        SharedBoxGuard(self.0.lock().unwrap())
    }

    pub fn read_lock(&self) -> SharedBoxGuard<T> {
        self.write_lock()
    }
}

impl<T: PartialEq> std::cmp::PartialEq for SharedBox<T> {
    fn eq(&self, other: &Self) -> bool {
        let a = self.read_lock();
        let b = other.read_lock();
        *a == *b
    }
}

impl<'a, T: 'a> Deref for SharedBoxGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'a, T: 'a> SharedBoxGuard<'a, T> {
    pub fn replace(&mut self, value: T) -> T {
        std::mem::replace(&mut *self.0, value)
    }
}

pub fn is_var_name(x: &Symbol) -> bool {
    match x.name() {
        "U" | "Nat" | "zero" | "add1" | "which-Nat" | "ind-Nat" | "rec-Nat" | "iter-Nat" | "->"
        | "→" | "Π" | "Pi" | "∏" | "λ" | "lambda" | "quote" | "Atom" | "Σ" | "Sigma" | "Pair"
        | "cons" | "car" | "cdr" | "Trivial" | "sole" | "::" | "nil" | "List" | "rec-List"
        | "ind-List" | "Absurd" | "ind-Absurd" | "=" | "same" | "replace" | "symm" | "trans"
        | "cong" | "ind-=" | "Vec" | "vec::" | "vecnil" | "head" | "tail" | "ind-Vec"
        | "Either" | "left" | "right" | "ind-Either" | "the" | "TODO" => false,
        _ => true,
    }
}
