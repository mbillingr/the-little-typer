use crate::alpha;
use crate::errors::{Error, Result};
use crate::fresh::freshen;
use crate::normalize::{val_in_ctx, val_of};
use crate::sexpr::Sexpr;
use crate::symbol::Symbol;
use crate::typechecker::same_type;
use crate::types::{cores, values};
use maplit::hashset;
use sexpr_parser::parse;
use std::any::Any;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
pub use std::sync::{Arc as R, Mutex, RwLock};

pub trait CoreInterface: Any + Debug + Display + Sync + Send {
    fn as_any(&self) -> &dyn Any;
    fn same(&self, other: &dyn CoreInterface) -> bool;

    fn occurring_names(&self) -> HashSet<Symbol>;

    fn val_of(&self, env: &Env) -> Value;

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core>;

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)>;

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        let (t_out, e_out) = self.synth(ctx, r)?;
        same_type(ctx, &val_in_ctx(ctx, &t_out), tv)?;
        Ok(e_out)
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool;

    fn resugar(&self) -> (HashSet<Symbol>, Core);
}

#[derive(Debug, Clone)]
pub enum Core {
    Fun(Vec<Core>),
    PiStar(Vec<(Symbol, Core)>, R<Core>),
    LambdaStar(Vec<Symbol>, R<Core>),
    AppStar(R<Core>, Vec<Core>),
    Object(R<dyn CoreInterface>),
}

impl PartialEq for Core {
    fn eq(&self, other: &Self) -> bool {
        use Core::*;
        match (self, other) {
            (Fun(a), Fun(b)) => a == b,
            (PiStar(a, r1), PiStar(b, r2)) => a == b && r1 == r2,
            (LambdaStar(a, r1), LambdaStar(b, r2)) => a == b && r1 == r2,
            (AppStar(f1, a1), AppStar(f2, a2)) => f1 == f2 && a1 == a2,
            (Object(a), Object(b)) => R::ptr_eq(a, b) || a.same(&**b),
            _ => false,
        }
    }
}

impl Core {
    pub fn new(obj: impl CoreInterface) -> Self {
        Core::Object(R::new(obj))
    }

    pub fn the(t: Core, e: Core) -> Self {
        cores::the(t, e)
    }

    pub fn fun(arg_types: Vec<Core>, ret_type: Core) -> Self {
        assert!(arg_types.len() > 0);
        let mut types = arg_types;
        types.push(ret_type);
        Core::Fun(types)
    }

    pub fn pi_star(params: Vec<(Symbol, Core)>, rt: impl Into<R<Core>>) -> Self {
        Core::PiStar(params, rt.into())
    }

    pub fn pi(x: impl Into<Symbol>, xt: Core, rt: Core) -> Self {
        cores::pi(x, xt, rt)
    }

    pub fn lambda(x: impl Into<Symbol>, body: Core) -> Self {
        cores::lambda(x, body)
    }

    pub fn lambda_star(params: impl Into<Vec<Symbol>>, body: impl Into<R<Core>>) -> Self {
        Self::LambdaStar(params.into(), body.into())
    }

    pub fn app(f: Core, a: Core) -> Self {
        cores::app(f, a)
    }

    pub fn app_star(f: impl Into<R<Core>>, args: impl Into<Vec<Core>>) -> Self {
        Core::AppStar(f.into(), args.into())
    }

    pub fn symbol(s: impl Into<Symbol>) -> Self {
        cores::refer(s)
    }

    pub fn quote(s: impl Into<Symbol>) -> Self {
        cores::quote(s)
    }

    pub fn nat(mut x: u64) -> Self {
        let mut n = cores::zero();
        while x > 0 {
            x -= 1;
            n = Core::add1(n)
        }
        n
    }

    pub fn add1(n: Core) -> Self {
        cores::add1(n)
    }
}

impl Display for Core {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Core::*;
        match self {
            Fun(ts) => {
                write!(f, "(->")?;
                for t in &**ts {
                    write!(f, " {}", t)?;
                }
                write!(f, ")")
            }
            PiStar(bindings, rt) => {
                let b: Vec<_> = bindings
                    .iter()
                    .map(|(x, t)| format!("({} {})", x.name(), t))
                    .collect();
                write!(f, "(Π ({}) {})", b.join(" "), rt)
            }
            LambdaStar(params, body) => write!(
                f,
                "(λ ({}) {})",
                params
                    .iter()
                    .map(|x| x.name())
                    .collect::<Vec<_>>()
                    .join(" "),
                body
            ),
            AppStar(func, args) => write!(
                f,
                "({} {})",
                func,
                args.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Object(obj) => write!(f, "{}", obj),
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
                "U" => cores::universe(),
                "Nat" => cores::nat(),
                "zero" => cores::zero(),
                "Atom" => cores::atom(),
                _ if is_var_name(s) => cores::refer(s.clone()),
                name => todo!("{}", name),
            },
            Sexpr::SmallNat(x) => Core::nat(*x),
            Sexpr::List(list) => match &list[..] {
                [Sexpr::Symbol(s), args @ ..] if !is_var_name(s) => match (s.name(), args) {
                    ("the", [t, v]) => Core::the(Core::from(t), Core::from(v)),
                    ("add1", [n]) => cores::add1(Core::from(n)),
                    ("which-Nat", [target, base, step]) => {
                        cores::which_nat(Core::from(target), Core::from(base), Core::from(step))
                    }
                    ("ind-Nat", [target, motive, base, step]) => cores::ind_nat(
                        Core::from(target),
                        Core::from(motive),
                        Core::from(base),
                        Core::from(step),
                    ),
                    ("quote", [Sexpr::Symbol(s)]) => Core::quote(s.clone()),
                    ("->", [ts @ .., rt]) => {
                        Core::fun(ts.iter().map(Core::from).collect(), Core::from(rt))
                    }
                    ("Pi" | "Π", [Sexpr::List(params), rt]) => Core::pi_star(
                        params
                            .iter()
                            .map(|x| match x {
                                Sexpr::List(x) => match &x[..] {
                                    [Sexpr::Symbol(name), typ] => (name.clone(), Core::from(typ)),
                                    _ => unimplemented!(),
                                },
                                _ => unimplemented!(),
                            })
                            .collect(),
                        Core::from(rt),
                    ),
                    ("lambda" | "λ", [Sexpr::List(params), body]) => Core::LambdaStar(
                        params
                            .iter()
                            .map(|x| x.as_symbol().cloned().unwrap())
                            .collect(),
                        Core::from(body).into(),
                    ),
                    (key, _) => todo!("({} ...)", key),
                },
                [f, args @ ..] => {
                    Core::AppStar(R::new(Core::from(f)), args.iter().map(Core::from).collect())
                }
                _ => unimplemented!("{:?}", list),
            },
        }
    }
}

pub trait ValueInterface: Any + Debug + Sync + Send {
    fn as_any(&self) -> &dyn Any;
    fn same(&self, other: &dyn ValueInterface) -> bool;
    fn read_back_type(&self, ctx: &Ctx) -> Result<Core>;

    fn read_back(&self, _ctx: &Ctx, _tv: &Value, _v: &Value) -> Result<Core> {
        unimplemented!("{:?}", self)
    }

    fn apply(
        &self,
        _ctx: &Ctx,
        _r: &Renaming,
        rator_out: &Core,
        _rand: &Core,
    ) -> Result<(Core, Core)> {
        Err(Error::NotAFunctionType((*rator_out).clone()))
    }

    fn now<'a>(&self, v: &'a Value) -> Cow<'a, Value> {
        Cow::Borrowed(v)
    }

    fn as_neutral(&self) -> Option<(&Value, &N)> {
        None
    }
}

impl PartialEq for dyn ValueInterface {
    fn eq(&self, other: &Self) -> bool {
        self.same(other)
    }
}

#[derive(Debug, Clone)]
pub struct Value(R<dyn ValueInterface>);

impl Value {
    pub fn new(obj: impl ValueInterface) -> Self {
        Value(R::new(obj))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        &self.0 == &other.0
    }
}

impl ValueInterface for Value {
    fn as_any(&self) -> &dyn Any {
        self.0.as_any()
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        self.0.same(other)
    }

    fn read_back_type(&self, ctx: &Ctx) -> Result<Core> {
        self.0.read_back_type(ctx)
    }

    fn read_back(&self, ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
        self.0.read_back(ctx, tv, v)
    }

    fn apply(
        &self,
        ctx: &Ctx,
        r: &Renaming,
        rator_out: &Core,
        rand: &Core,
    ) -> Result<(Core, Core)> {
        self.0.apply(ctx, r, rator_out, rand)
    }

    fn now<'a>(&self, v: &'a Value) -> Cow<'a, Value> {
        assert!(std::ptr::eq(self, v));
        self.0.now(v)
    }

    fn as_neutral(&self) -> Option<(&Value, &N)> {
        self.0.as_neutral()
    }
}

#[derive(Clone)]
pub enum Closure {
    FirstOrder { env: Env, var: Symbol, expr: Core },
    HigherOrder(R<dyn Sync + Send + Fn(Value) -> Value>),
}

impl Closure {
    pub fn higher(f: impl 'static + Sync + Send + Fn(Value) -> Value) -> Self {
        Closure::HigherOrder(R::new(f))
    }

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
    App(R<N>, The),
    WhichNat(R<N>, The, The),
}

impl N {
    pub fn app(f: N, typ: Value, val: Value) -> Self {
        N::App(R::new(f), The(typ, val))
    }
    pub fn which_nat(target: impl Into<R<N>>, base: The, step: The) -> Self {
        N::WhichNat(target.into(), base, step)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct The(pub Value, pub Value);

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
                .insert(x.clone(), values::neutral(tv.clone(), N::Var(x.clone())));
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
    use Core::*;
    match expr {
        Fun(types) => {
            let mut names = hashset! {};
            for t in types {
                names = &names | &occurring_names(t)
            }
            names
        }
        PiStar(bindings, t) => bindings
            .iter()
            .map(|(x, t)| occurring_binder_names(x, t))
            .fold(occurring_names(t), |a, b| &a | &b),
        LambdaStar(params, body) => {
            &params.iter().cloned().collect::<HashSet<_>>() | &occurring_names(body)
        }
        AppStar(f, args) => args
            .iter()
            .fold(occurring_names(f), |a, b| &a | &occurring_names(b)),
        Object(obj) => obj.occurring_names(),
    }
}

pub fn occurring_binder_names(name: &Symbol, t: &Core) -> HashSet<Symbol> {
    let mut names = occurring_names(t);
    names.insert(name.clone());
    names
}

pub fn is_var_name(x: &str) -> bool {
    match x {
        "U" | "Nat" | "zero" | "add1" | "which-Nat" | "ind-Nat" | "rec-Nat" | "iter-Nat" | "->"
        | "→" | "Π" | "Pi" | "∏" | "λ" | "lambda" | "quote" | "Atom" | "Σ" | "Sigma" | "Pair"
        | "cons" | "car" | "cdr" | "Trivial" | "sole" | "::" | "nil" | "List" | "rec-List"
        | "ind-List" | "Absurd" | "ind-Absurd" | "=" | "same" | "replace" | "symm" | "trans"
        | "cong" | "ind-=" | "Vec" | "vec::" | "vecnil" | "head" | "tail" | "ind-Vec"
        | "Either" | "left" | "right" | "ind-Either" | "the" | "TODO" => false,
        _ => true,
    }
}
