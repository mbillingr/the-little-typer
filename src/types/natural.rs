use crate::alpha;
use crate::alpha::alpha_equiv_aux;
use crate::basics::{
    ctx_to_env, fresh, occurring_names, Closure, Core, CoreInterface, Ctx, Env, Renaming, The,
    Value, ValueInterface, N,
};
use crate::errors::{Error, Result};
use crate::normalize::{now, read_back, val_in_ctx};
use crate::resugar::resugar_;
use crate::symbol::Symbol;
use crate::typechecker::{check, synth};
use crate::types::cores::{which_nat, which_nat_desugared};
use crate::types::functions::do_ap;
use crate::types::neutral::Neutral;
use crate::types::values::{add1, later, zero};
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Formatter;

macro_rules! pi_type {
    ((), $ret:expr) => {$ret};

    ((($x:ident, $arg_t:expr) $($b:tt)*), $ret:expr) => {
        values::pi(stringify!($x), $arg_t, Closure::higher(move |$x| pi_type!(($($b)*), $ret)))
    };
}

/// The type of all natural numbers
#[derive(Debug, Copy, Clone)]
pub struct Nat;

/// The natural number 0
#[derive(Debug, Copy, Clone)]
pub struct Zero;

/// One more than another natural number
#[derive(Debug, Clone, PartialEq)]
pub struct Add1<T>(pub T);

#[derive(Debug, Clone, PartialEq)]
pub struct WhichNat {
    target: Core,
    base: MaybeTyped,
    step: Core,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndNat {
    target: Core,
    motive: Core,
    base: Core,
    step: Core,
}

#[derive(Debug, Clone, PartialEq)]
enum MaybeTyped {
    Plain(Core),
    The(Core, Core),
}

impl WhichNat {
    pub fn typed(target: Core, base_t: Core, base: Core, step: Core) -> Self {
        WhichNat {
            target,
            step,
            base: MaybeTyped::The(base_t, base),
        }
    }

    pub fn untyped(target: Core, base: Core, step: Core) -> Self {
        WhichNat {
            target,
            step,
            base: MaybeTyped::Plain(base),
        }
    }
}

impl IndNat {
    pub fn new(target: Core, motive: Core, base: Core, step: Core) -> Self {
        IndNat {
            target,
            motive,
            base,
            step,
        }
    }
}

impl CoreInterface for Nat {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn CoreInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn occurring_names(&self) -> HashSet<Symbol> {
        HashSet::new()
    }

    fn val_of(&self, _env: &Env) -> Value {
        values::nat()
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        Ok(cores::nat())
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Ok((cores::universe(), cores::nat()))
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        _lvl: usize,
        _b1: &alpha::Bindings,
        _b2: &alpha::Bindings,
    ) -> bool {
        CoreInterface::same(self, other)
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        (HashSet::new(), cores::nat())
    }
}

impl CoreInterface for Zero {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn CoreInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn occurring_names(&self) -> HashSet<Symbol> {
        HashSet::new()
    }

    fn val_of(&self, _env: &Env) -> Value {
        values::zero()
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        Err(Error::NotAType(Core::new(self.clone())))
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Ok((cores::nat(), cores::zero()))
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        _lvl: usize,
        _b1: &alpha::Bindings,
        _b2: &alpha::Bindings,
    ) -> bool {
        CoreInterface::same(self, other)
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        (HashSet::new(), cores::zero())
    }
}

impl CoreInterface for Add1<Core> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn CoreInterface) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map(|o| self == o)
            .unwrap_or(false)
    }

    fn occurring_names(&self) -> HashSet<Symbol> {
        occurring_names(&self.0)
    }

    fn val_of(&self, env: &Env) -> Value {
        values::add1(later(env.clone(), self.0.clone()))
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        Err(Error::NotAType(Core::new(self.clone())))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        check(ctx, r, &self.0, &values::nat()).map(|n_out| (cores::nat(), Core::add1(n_out)))
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            alpha_equiv_aux(lvl, b1, b2, &self.0, &other.0)
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let (names, n) = resugar_(&self.0);
        (names, cores::add1(n))
    }
}

impl CoreInterface for WhichNat {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn CoreInterface) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map(|o| self == o)
            .unwrap_or(false)
    }

    fn occurring_names(&self) -> HashSet<Symbol> {
        let names = &occurring_names(&self.target) | &occurring_names(&self.step);

        let base_names = match &self.base {
            MaybeTyped::Plain(b) => occurring_names(b),
            MaybeTyped::The(bt, b) => &occurring_names(bt) | &occurring_names(b),
        };

        &names | &base_names
    }

    fn val_of(&self, env: &Env) -> Value {
        match &self.base {
            MaybeTyped::Plain(_) => unimplemented!("evaluate a desugared which-Nat instead"),
            MaybeTyped::The(bt, b) => do_which_nat(
                later(env.clone(), self.target.clone()),
                later(env.clone(), bt.clone()),
                later(env.clone(), b.clone()),
                later(env.clone(), self.step.clone()),
            ),
        }
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        Err(Error::NotAType(Core::new(self.clone())))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        match &self.base {
            MaybeTyped::The(_, _) => unimplemented!("already synth'ed"),
            MaybeTyped::Plain(b) => {
                let tgt_out = check(ctx, r, &self.target, &values::nat())?;
                let (b_t_out, b_out) = synth(ctx, r, b)?;
                let n_minus_one = fresh(ctx, &Symbol::new("n-1"));
                let s_out = check(
                    ctx,
                    r,
                    &self.step,
                    &values::pi(
                        n_minus_one.clone(),
                        values::nat(),
                        Closure::FirstOrder {
                            env: ctx_to_env(ctx),
                            var: n_minus_one,
                            expr: b_t_out.clone(),
                        },
                    ),
                )?;
                Ok((
                    b_t_out.clone(),
                    cores::which_nat_desugared(tgt_out, b_t_out, b_out, s_out),
                ))
            }
        }
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            alpha_equiv_aux(lvl, b1, b2, &self.target, &other.target)
                && alpha_equiv_aux(lvl, b1, b2, &self.step, &other.step)
                && match (&self.base, &other.base) {
                    (MaybeTyped::Plain(bs1), MaybeTyped::Plain(bs2)) => {
                        alpha_equiv_aux(lvl, b1, b2, bs1, bs2)
                    }
                    (MaybeTyped::The(bt1, bs1), MaybeTyped::The(bt2, bs2)) => {
                        alpha_equiv_aux(lvl, b1, b2, bt1, bt2)
                            && alpha_equiv_aux(lvl, b1, b2, bs1, bs2)
                    }
                    _ => false,
                }
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let tgt = resugar_(&self.target);
        let stp = resugar_(&self.step);
        match &self.base {
            MaybeTyped::Plain(b) => {
                let b = resugar_(b);
                (&(&tgt.0 | &b.0) | &stp.0, which_nat(tgt.1, b.1, stp.1))
            }
            MaybeTyped::The(bt, b) => {
                let bt = resugar_(bt);
                let b = resugar_(b);
                (
                    &(&tgt.0 | &bt.0) | &(&b.0 | &stp.0),
                    which_nat_desugared(tgt.1, bt.1, b.1, stp.1),
                )
            }
        }
    }
}

impl CoreInterface for IndNat {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn CoreInterface) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map(|o| self == o)
            .unwrap_or(false)
    }

    fn occurring_names(&self) -> HashSet<Symbol> {
        &(&occurring_names(&self.target) | &occurring_names(&self.motive))
            | &(&occurring_names(&self.base) | &occurring_names(&self.step))
    }

    fn val_of(&self, env: &Env) -> Value {
        do_ind_nat(
            later(env.clone(), self.target.clone()),
            later(env.clone(), self.motive.clone()),
            later(env.clone(), self.base.clone()),
            later(env.clone(), self.step.clone()),
        )
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        Err(Error::NotAType(Core::new(self.clone())))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let tgt_out = check(ctx, r, &self.target, &values::nat())?;
        let mot_out = check(
            ctx,
            r,
            &self.motive,
            &values::pi("n", values::nat(), Closure::higher(|_| values::universe())),
        )?;
        let mot_val = val_in_ctx(ctx, &mot_out);
        let b_out = check(ctx, r, &self.base, &do_ap(&mot_val, values::zero()))?;
        let s_out = check(
            ctx,
            r,
            &self.step,
            &pi_type!(((n_minus_1, values::nat())), {
                let mot_val = mot_val.clone();
                pi_type!(
                    ((_ih, do_ap(&mot_val, n_minus_1.clone()))),
                    do_ap(&mot_val, values::add1(n_minus_1.clone()))
                )
            }),
        )?;
        Ok((
            cores::app(mot_out.clone(), tgt_out.clone()),
            cores::ind_nat(tgt_out, mot_out, b_out, s_out),
        ))
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            alpha_equiv_aux(lvl, b1, b2, &self.target, &other.target)
                && alpha_equiv_aux(lvl, b1, b2, &self.motive, &other.motive)
                && alpha_equiv_aux(lvl, b1, b2, &self.base, &other.base)
                && alpha_equiv_aux(lvl, b1, b2, &self.step, &other.step)
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let tgt = resugar_(&self.target);
        let mot = resugar_(&self.motive);
        let bse = resugar_(&self.base);
        let stp = resugar_(&self.step);

        (
            &(&tgt.0 | &mot.0) | &(&bse.0 | &stp.0),
            cores::ind_nat(tgt.1, mot.1, bse.1, stp.1),
        )
    }
}

impl ValueInterface for Nat {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Ok(cores::nat())
    }

    fn read_back(&self, ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
        if v.as_any().downcast_ref::<Zero>().is_some() {
            Ok(cores::zero())
        } else if let Some(Add1(n)) = v.as_any().downcast_ref::<Add1<Value>>() {
            Ok(cores::add1(read_back(ctx, tv, n)))
        } else {
            Err(Error::TypeMismatchVar(v.clone(), tv.clone()))
        }
    }
}

impl ValueInterface for Zero {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Err(Error::NotATypeVar(zero()))
    }
}

impl ValueInterface for Add1<Value> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        if let Some(Add1(n)) = other.as_any().downcast_ref::<Self>() {
            &self.0 == n
        } else {
            false
        }
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Err(Error::NotATypeVar(add1(self.0.clone())))
    }
}

impl std::fmt::Display for Nat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nat")
    }
}

impl std::fmt::Display for Zero {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zero")
    }
}

impl std::fmt::Display for Add1<Core> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(add1 {})", self.0)
    }
}

impl std::fmt::Display for WhichNat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.base {
            MaybeTyped::Plain(base) => {
                write!(f, "(which-Nat {} {} {})", self.target, base, self.step)
            }
            MaybeTyped::The(base_type, base) => write!(
                f,
                "(which-Nat {} (the {} {}) {})",
                self.target, base_type, base, self.step
            ),
        }
    }
}

impl std::fmt::Display for IndNat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(ind-Nat {} {} {} {})",
            self.target, self.motive, self.base, self.step
        )
    }
}

fn do_which_nat(tgt_v: Value, bt_v: Value, b_v: Value, s_v: Value) -> Value {
    match now(&tgt_v).as_any().downcast_ref::<Zero>() {
        Some(_) => return b_v,
        None => {}
    };

    match now(&tgt_v).as_any().downcast_ref::<Add1<Value>>() {
        Some(Add1(n_minus_1v)) => return do_ap(&s_v, n_minus_1v.clone()),
        None => {}
    };

    match now(&tgt_v).as_any().downcast_ref::<Neutral>() {
        Some(Neutral { kind: ne, .. }) => {
            return values::neutral(
                bt_v.clone(),
                N::which_nat(
                    ne.clone(),
                    The(bt_v.clone(), b_v),
                    The(pi_type!(((_n, values::nat())), { bt_v.clone() }), s_v),
                ),
            )
        }
        None => {}
    };

    unreachable!("{:?}", now(&tgt_v))
}

fn do_ind_nat(tgt_v: Value, mot_v: Value, b_v: Value, s_v: Value) -> Value {
    match now(&tgt_v).as_any().downcast_ref::<Zero>() {
        Some(_) => return b_v,
        None => {}
    };

    match now(&tgt_v).as_any().downcast_ref::<Add1<Value>>() {
        Some(Add1(n_minus_1v)) => {
            return do_ap(
                &do_ap(&s_v, n_minus_1v.clone()),
                do_ind_nat(n_minus_1v.clone(), mot_v, b_v, s_v),
            )
        }
        None => {}
    };

    todo!()

    //unreachable!("{:?}", now(&tgt_v))
}
