use crate::alpha;
use crate::alpha::alpha_equiv_aux;
use crate::basics::{
    ctx_to_env, fresh, occurring_names, Closure, Core, CoreInterface, Ctx, Env, Renaming, Value,
    ValueInterface,
};
use crate::errors::{Error, Result};
use crate::normalize::{read_back, val_in_ctx};
use crate::resugar::resugar_;
use crate::symbol::Symbol;
use crate::typechecker::{check, same_type, synth};
use crate::types::values::{add1, later, zero};
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Formatter;
use crate::types::cores::which_nat;

/// The type of all natural numbers
#[derive(Debug, Copy, Clone)]
pub struct Nat;

/// The natural number 0
#[derive(Debug, Copy, Clone)]
pub struct Zero;

/// One more than another natural number
#[derive(Debug, Clone)]
pub struct Add1<T>(pub T);

#[derive(Debug, Clone)]
pub struct WhichNat {
    pub target: Core,
    pub base: Core,
    pub step: Core,
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

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        let (t_out, e_out) = self.synth(ctx, r)?;
        same_type(ctx, &val_in_ctx(ctx, &t_out), tv)?;
        Ok(e_out)
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
        (HashSet::new(), Core::new(*self))
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

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        let (t_out, e_out) = self.synth(ctx, r)?;
        same_type(ctx, &val_in_ctx(ctx, &t_out), tv)?;
        Ok(e_out)
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
        (HashSet::new(), Core::new(*self))
    }
}

impl CoreInterface for Add1<Core> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn CoreInterface) -> bool {
        unimplemented!()
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
    ) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            alpha_equiv_aux(lvl, b1, b2, &self.0, &other.0)
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        resugar_(&self.0)
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
        &(&occurring_names(&self.target) | &occurring_names(&self.base))
            | &occurring_names(&self.step)
    }

    fn val_of(&self, env: &Env) -> Value {
        unimplemented!("evaluate WhichNatUnsugared instead")
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        Err(Error::NotAType(Core::new(self.clone())))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let tgt_out = check(ctx, r, &self.target, &values::nat())?;
        let (b_t_out, b_out) = synth(ctx, r, &self.base)?;
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
    ) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            alpha_equiv_aux(lvl, b1, b2, &self.target, &other.target)
                && alpha_equiv_aux(lvl, b1, b2, &self.base, &other.base)
                && alpha_equiv_aux(lvl, b1, b2, &self.step, &other.step)
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let t = resugar_(&self.target);
        let b = resugar_(&self.base);
        let s = resugar_(&self.step);
        (&(&t.0 | &b.0) | &s.0, which_nat(t.1, b.1, s.1))
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
        write!(f, "(which-Nat {} {} {})", self.target, self.base, self.step)
    }
}
