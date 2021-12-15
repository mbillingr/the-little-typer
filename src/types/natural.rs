use crate::alpha;
use crate::alpha::alpha_equiv_aux;
use crate::basics::{
    ctx_to_env, fresh, occurring_names, Closure, Core, CoreInterface, Ctx, Env, Renaming, Value,
    ValueInterface,
};
use crate::errors::{Error, Result};
use crate::normalize::{now, read_back};
use crate::resugar::resugar_;
use crate::symbol::Symbol;
use crate::typechecker::{check, synth};
use crate::types::cores::{which_nat, which_nat_desugared};
use crate::types::functions::do_ap;
use crate::types::values::{add1, later, zero};
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Formatter;

/// The type of all natural numbers
#[derive(Debug, Copy, Clone)]
pub struct Nat;

/// The natural number 0
#[derive(Debug, Copy, Clone)]
pub struct Zero;

/// One more than another natural number
#[derive(Debug, Clone, PartialEq)]
pub struct Add1<T>(pub T);

#[derive(Debug, Clone)]
pub enum WhichNat {
    Plain(Core, Core, Core),
    Verbose(Core, Core, Core, Core),
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

    fn same(&self, _other: &dyn CoreInterface) -> bool {
        unimplemented!()
    }

    fn occurring_names(&self) -> HashSet<Symbol> {
        match self {
            WhichNat::Plain(tgt, b, s) => {
                &(&occurring_names(tgt) | &occurring_names(b)) | &occurring_names(s)
            }
            WhichNat::Verbose(tgt, bt, b, s) => {
                &(&occurring_names(tgt) | &occurring_names(bt))
                    | &(&occurring_names(b) | &occurring_names(s))
            }
        }
    }

    fn val_of(&self, env: &Env) -> Value {
        match self {
            WhichNat::Plain(_, _, _) => unimplemented!("evaluate WhichNatUnsugared instead"),
            WhichNat::Verbose(tgt, bt, b, s) => do_which_nat(
                later(env.clone(), tgt.clone()),
                later(env.clone(), bt.clone()),
                later(env.clone(), b.clone()),
                later(env.clone(), s.clone()),
            ),
        }
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        Err(Error::NotAType(Core::new(self.clone())))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        match self {
            WhichNat::Verbose(_, _, _, _) => unimplemented!("already synth'ed"),
            WhichNat::Plain(tgt, b, s) => {
                let tgt_out = check(ctx, r, tgt, &values::nat())?;
                let (b_t_out, b_out) = synth(ctx, r, b)?;
                let n_minus_one = fresh(ctx, &Symbol::new("n-1"));
                let s_out = check(
                    ctx,
                    r,
                    s,
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
            match (self, other) {
                (WhichNat::Plain(tgt1, bs1, s1), WhichNat::Plain(tgt2, bs2, s2)) => {
                    alpha_equiv_aux(lvl, b1, b2, tgt1, tgt2)
                        && alpha_equiv_aux(lvl, b1, b2, bs1, bs2)
                        && alpha_equiv_aux(lvl, b1, b2, s1, s2)
                }
                (WhichNat::Verbose(tgt1, bt1, bs1, s1), WhichNat::Verbose(tgt2, bt2, bs2, s2)) => {
                    alpha_equiv_aux(lvl, b1, b2, tgt1, tgt2)
                        && alpha_equiv_aux(lvl, b1, b2, bt1, bt2)
                        && alpha_equiv_aux(lvl, b1, b2, bs1, bs2)
                        && alpha_equiv_aux(lvl, b1, b2, s1, s2)
                }
                _ => false,
            }
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        match self {
            WhichNat::Plain(tgt, b, s) => {
                let t = resugar_(tgt);
                let b = resugar_(b);
                let s = resugar_(s);
                (&(&t.0 | &b.0) | &s.0, which_nat(t.1, b.1, s.1))
            }
            WhichNat::Verbose(tgt, bt, b, s) => {
                let t = resugar_(tgt);
                let bt = resugar_(bt);
                let b = resugar_(b);
                let s = resugar_(s);
                (
                    &(&t.0 | &bt.0) | &(&b.0 | &s.0),
                    which_nat_desugared(t.1, bt.1, b.1, s.1),
                )
            }
        }
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
        match self {
            WhichNat::Plain(target, base, step) => {
                write!(f, "(which-Nat {} {} {})", target, base, step)
            }
            WhichNat::Verbose(target, base_type, base, step) => write!(
                f,
                "(which-Nat {} (the {} {}) {})",
                target, base_type, base, step
            ),
        }
    }
}

fn do_which_nat(tgt_v: Value, _bt_v: Value, b_v: Value, s_v: Value) -> Value {
    match now(&tgt_v).as_any().downcast_ref::<Zero>() {
        Some(_) => return b_v,
        None => {}
    };

    match now(&tgt_v).as_any().downcast_ref::<Add1<Value>>() {
        Some(Add1(n_minus_1v)) => return do_ap(&s_v, n_minus_1v.clone()),
        None => {}
    };

    todo!()
}
