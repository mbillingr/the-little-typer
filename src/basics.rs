use crate::alpha;
use crate::errors::{Error, Result};
use crate::fresh::freshen;
use crate::normalize::val_in_ctx;
use crate::sexpr::Sexpr;
use crate::symbol::Symbol;
use crate::types::functions::NeutralApp;
use crate::types::natural::NeutralWhichNat;
use crate::types::reference::NeutralVar;
use crate::types::{cores, values};
use sexpr_matcher::match_sexpr;
use sexpr_parser::parse;
use std::any::Any;
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

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core>;

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool;

    fn resugar(&self) -> (HashSet<Symbol>, Core);
}

impl dyn CoreInterface {
    pub fn try_as<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

#[derive(Debug, Clone)]
pub struct Core(R<dyn CoreInterface>);

impl PartialEq for Core {
    fn eq(&self, other: &Self) -> bool {
        R::ptr_eq(&self.0, &other.0) || self.0.same(&*other.0)
    }
}

impl Core {
    pub fn new(obj: impl CoreInterface) -> Self {
        Core(R::new(obj))
    }

    pub fn try_as<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    pub fn the(t: Core, e: Core) -> Self {
        cores::the(t, e)
    }

    pub fn fun(arg_types: Vec<Core>, ret_type: Core) -> Self {
        assert!(arg_types.len() > 0);
        let mut types = arg_types;
        types.push(ret_type);
        cores::fun(types)
    }

    pub fn pi_star(params: Vec<(Symbol, Core)>, rt: impl Into<Core>) -> Self {
        cores::pi_star(params, rt.into())
    }

    pub fn pi(x: impl Into<Symbol>, xt: Core, rt: Core) -> Self {
        cores::pi(x, xt, rt)
    }

    pub fn lambda(x: impl Into<Symbol>, body: Core) -> Self {
        cores::lambda(x, body)
    }

    pub fn lambda_star(params: impl Into<Vec<Symbol>>, body: impl Into<Core>) -> Self {
        cores::lambda_star(params.into(), body.into())
    }

    pub fn app(f: Core, a: Core) -> Self {
        cores::app(f, a)
    }

    pub fn app_star(f: impl Into<Core>, args: impl Into<Vec<Core>>) -> Self {
        cores::app_star(f.into(), args.into())
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

impl CoreInterface for Core {
    fn as_any(&self) -> &dyn Any {
        self.0.as_any()
    }

    fn same(&self, other: &dyn CoreInterface) -> bool {
        self.0.same(other)
    }

    fn occurring_names(&self) -> HashSet<Symbol> {
        self.0.occurring_names()
    }

    fn val_of(&self, env: &Env) -> Value {
        self.0.val_of(env)
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        self.0.is_type(ctx, r)
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        self.0.synth(ctx, r)
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        self.0.check(ctx, r, tv)
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool {
        self.0.alpha_equiv_aux(other, lvl, b1, b2)
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        self.0.resugar()
    }
}

impl Display for Core {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
        match_sexpr! {
            sexpr,
            case [Sexpr::Invalid(s)] => cores::invalid_syntax(s),
            case [Sexpr::SmallNat(x)] => cores::the_nat(*x),
            case "U" => cores::universe(),
            case "Nat" => cores::nat(),
            case "zero" => cores::zero(),
            case "Atom" => cores::atom(),
            case "nil" => cores::nil(),
            case [Sexpr::Symbol(s)] => if is_var_name(s) {
                    cores::refer(s.clone())
                } else {
                    todo!("{:?}", s)
                },
            case ("the", t, v) => Core::the(Core::from(t), Core::from(v)),
            case ("add1", n) => cores::add1(Core::from(n)),
            case ("which-Nat", target, base, step) => cores::which_nat(Core::from(target), Core::from(base), Core::from(step)),
            case ("iter-Nat", target, base, step) => cores::iter_nat(Core::from(target), Core::from(base), Core::from(step)),
            case ("rec-Nat", target, base, step) => cores::rec_nat(Core::from(target), Core::from(base), Core::from(step)),
            case ("ind-Nat", target, motive, base, step) => cores::ind_nat(Core::from(target), Core::from(motive), Core::from(base), Core::from(step)),
            case ("quote", [Sexpr::Symbol(s)]) => Core::quote(s.clone()),
            case ("->" :: [[ts@.., rt]]) => Core::fun(ts.iter().map(Core::from).collect(), Core::from(rt)),
            case ("Pi", [Sexpr::List(params)], rt) => Core::pi_star(parse_binders(params), Core::from(rt)),
            case ("Π", [Sexpr::List(params)], rt) => Core::pi_star(parse_binders(params), Core::from(rt)),
            case ("∏", [Sexpr::List(params)], rt) => Core::pi_star(parse_binders(params), Core::from(rt)),
            case ("lambda", [Sexpr::List(params)], body) => cores::lambda_star(
                        params.iter().map(|x| x.as_symbol().cloned().unwrap()).collect(),
                        Core::from(body).into(),
                    ),
            case ("λ", [Sexpr::List(params)], body) => cores::lambda_star(
                        params.iter().map(|x| x.as_symbol().cloned().unwrap()).collect(),
                        Core::from(body).into(),
                    ),
            case ("Sigma", [Sexpr::List(params)], rt) => cores::sigma_star(parse_binders(params), Core::from(rt)),
            case ("Σ", [Sexpr::List(params)], rt) => cores::sigma_star(parse_binders(params), Core::from(rt)),
            case ("Pair", a, d) => cores::pair(Core::from(a), Core::from(d)),
            case ("cons", car, cdr) => cores::cons(Core::from(car), Core::from(cdr)),
            case ("car", cons) => cores::car(Core::from(cons)),
            case ("cdr", cons) => cores::cdr(Core::from(cons)),
            //
            case ("List", t) => cores::list(t.into()),
            case ("::", h, r) => cores::list_cons(h.into(), r.into()),
            case (op :: args) => cores::app_star(Core::from(op), args.iter().map(Core::from).collect()),
            case _ => todo!("{:?}", sexpr),
        }
    }
}

impl<'a, A: From<&'a Sexpr>, B: From<&'a Sexpr>> From<&'a Sexpr> for (A, B) {
    fn from(sexpr: &'a Sexpr) -> Self {
        match sexpr {
            Sexpr::List(x) => match &x[..] {
                [a, b] => return (A::from(a), B::from(b)),
                _ => {}
            },
            _ => {}
        }
        panic!("expected list of length 2, got {}", sexpr)
    }
}

impl From<&Sexpr> for Symbol {
    fn from(sexpr: &Sexpr) -> Self {
        match sexpr {
            Sexpr::Symbol(name) => name.clone(),
            _ => panic!("expected symbol, got {}", sexpr),
        }
    }
}

fn parse_binders(exprs: &[Sexpr]) -> Vec<(Symbol, Core)> {
    parse_sexpr_list(exprs)
}

fn parse_sexpr_list<'a, T: From<&'a Sexpr>>(exprs: &'a [Sexpr]) -> Vec<T> {
    exprs.iter().map(Into::into).collect()
}

pub trait ValueInterface: Any + Debug + Sync + Send {
    fn as_any(&self) -> &dyn Any;
    fn same(&self, other: &dyn ValueInterface) -> bool;
    fn read_back_type(&self, ctx: &Ctx) -> Result<Core>;

    fn read_back(&self, _ctx: &Ctx, _tv: &Value, _v: &Value) -> Result<Core> {
        unimplemented!("read_back {:?}", self)
    }

    fn apply(
        &self,
        ctx: &Ctx,
        r: &Renaming,
        rator_out: &Core,
        _rand: &Core,
    ) -> Result<(Core, Core)> {
        let (t_out, _) = rator_out.synth(ctx, r)?;
        Err(Error::NotAFunctionType(t_out))
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

impl dyn ValueInterface {
    pub fn try_as<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

#[derive(Debug, Clone)]
pub struct Value(R<dyn ValueInterface>);

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        &self.0 == &other.0
    }
}

impl Value {
    pub fn new(obj: impl ValueInterface) -> Self {
        Value(R::new(obj))
    }

    pub fn try_as<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    pub fn as_any(&self) -> &dyn Any {
        self.0.as_any()
    }

    pub fn same(&self, other: &dyn ValueInterface) -> bool {
        self.0.same(other)
    }

    pub fn read_back_type(&self, ctx: &Ctx) -> Result<Core> {
        self.0.read_back_type(ctx)
    }

    pub fn read_back(&self, ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
        self.0.read_back(ctx, tv, v)
    }

    pub fn apply(
        &self,
        ctx: &Ctx,
        r: &Renaming,
        rator_out: &Core,
        rand: &Core,
    ) -> Result<(Core, Core)> {
        self.0.apply(ctx, r, rator_out, rand)
    }

    pub fn as_neutral(&self) -> Option<(&Value, &N)> {
        self.0.as_neutral()
    }
}

impl FromStr for Value {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let sexpr = parse::<Sexpr>(s).map_err(|e| format!("{:?}", e))?;
        Ok((&sexpr).into())
    }
}

impl From<&Sexpr> for Value {
    fn from(sexpr: &Sexpr) -> Self {
        match_sexpr! {
            sexpr,
            case [Sexpr::Invalid(s)] => panic!("invalid value: {}", s),
            case [Sexpr::SmallNat(x)] => values::the_nat(*x),
            case "U" => values::universe(),
            case "Nat" => values::nat(),
            case "zero" => values::zero(),
            case "Atom" => values::atom(),
            case [Sexpr::Symbol(s)] => panic!("invalid value: {:?}", s),
            case ("add1", n) => values::add1(Value::from(n)),
            case ("quote", [Sexpr::Symbol(s)]) => values::quote(s.clone()),
            case ("cons", car, cdr) => values::cons(car.into(), cdr.into()),
            case _ => todo!("{:?}", sexpr),
        }
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
            Closure::FirstOrder { env, var, expr } => expr.val_of(&env.extend(var.clone(), v)),
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

pub trait NeutralInterface: Debug + Sync + Send {
    fn read_back_neutral(&self, ctx: &Ctx) -> Result<Core>;
}

#[derive(Debug, Clone)]
pub struct N(R<dyn NeutralInterface>);

impl N {
    pub fn app(f: impl NeutralInterface + 'static, typ: Value, val: Value) -> Self {
        N(R::new(NeutralApp(N(R::new(f)), The(typ, val))))
    }
    pub fn which_nat(target: impl NeutralInterface + 'static, base: The, step: The) -> Self {
        N(R::new(NeutralWhichNat(N(R::new(target)), base, step)))
    }

    pub fn read_back_neutral(&self, ctx: &Ctx) -> Result<Core> {
        self.0.read_back_neutral(ctx)
    }
}

impl<T: NeutralInterface + 'static> From<T> for N {
    fn from(n: T) -> Self {
        N(R::new(n))
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

    pub fn claim(&self, name: impl Into<Symbol>, t: Core) -> Result<Self> {
        let name = name.into();
        match self.0.assv(&name) {
            Some((_, Binder::Claim(_))) => return Err(Error::ClaimedName(name)),
            Some((_, Binder::Def(_, _))) => return Err(Error::DefinedName(name)),
            Some((_, Binder::Free(_))) => unreachable!("claims are only allowed in the global context, and there should never be free variables"),
            None => {},
        };

        let t_out = t.is_type(self, &Renaming::new())?;
        let tv = val_in_ctx(self, &t_out);
        Ok(self.extend(name, Binder::Claim(tv)))
    }

    pub fn define(&self, name: impl Into<Symbol>, v: Core) -> Result<Self> {
        let name = name.into();
        let tv = match self.0.assv(&name) {
            Some((_, Binder::Claim(tv))) => tv,
            Some((_, Binder::Def(_, _))) => return Err(Error::DefinedName(name)),
            Some((_, Binder::Free(_))) => unreachable!("definitions are only allowed in the global context, and there should never be free variables"),
            None => return Err(Error::UnclaimedName(name)),
        };

        let v_out = v.check(self, &Renaming::new(), &tv)?;
        let vv = val_in_ctx(self, &v_out);
        Ok(self.extend(name, Binder::Def(tv.clone(), vv)))
    }

    pub fn fresh(&self, x: &Symbol) -> Symbol {
        freshen(&self.names_only(), x)
    }

    pub fn fresh_binder(&self, expr: &Core, x: &Symbol) -> Symbol {
        freshen(&(&self.names_only() | &expr.occurring_names()), x)
    }

    pub fn bind_free(&self, x: Symbol, tv: Value) -> Result<Self> {
        if self.0.assv(&x).is_some() {
            Err(Error::AlreadyBound(x.clone(), self.clone()))
        } else {
            Ok(self.extend(x, Binder::Free(tv)))
        }
    }

    fn extend(&self, name: impl Into<Symbol>, binder: Binder) -> Self {
        Ctx(R::new(CtxImpl::Entry(name.into(), binder, self.clone())))
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

    pub fn to_env(&self) -> Env {
        match &*self.0 {
            CtxImpl::Nil => Env::new(),
            CtxImpl::Entry(x, Binder::Def(_, v), next) => {
                let ctx = next;
                let mut env = ctx.to_env();
                env.0.insert(x.clone(), v.clone());
                env
            }
            CtxImpl::Entry(x, Binder::Free(tv), next) => {
                let ctx = next;
                let mut env = ctx.to_env();
                env.0.insert(
                    x.clone(),
                    values::neutral(tv.clone(), NeutralVar(x.clone())),
                );
                env
            }
            CtxImpl::Entry(_, Binder::Claim(_), next) => next.to_env(),
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
