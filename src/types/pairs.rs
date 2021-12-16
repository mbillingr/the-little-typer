use crate::alpha;
use crate::basics::{
    fresh, fresh_binder, occurring_names, Closure, Core, CoreInterface, Ctx, Env, Renaming, Value,
    ValueInterface,
};
use crate::errors::{Error, Result};
use crate::normalize::{now, val_in_ctx};
use crate::symbol::Symbol;
use crate::typechecker::{check, is_type};
use crate::types::values::later;
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Sigma<T, C> {
    pub arg_name: Symbol,
    pub car_type: T,
    pub cdr_type: C,
}

/// The type of pairs
#[derive(Debug, Clone, PartialEq)]
pub struct Pair<T>(pub T, pub T);

/// pairs
#[derive(Debug, Clone, PartialEq)]
pub struct Cons<T>(pub T, pub T);

impl CoreInterface for Sigma<Core, Core> {
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
        todo!()
    }

    fn val_of(&self, env: &Env) -> Value {
        let av = later(env.clone(), self.car_type.clone());
        values::sigma(
            self.arg_name.clone(),
            av,
            Closure::FirstOrder {
                env: env.clone(),
                var: self.arg_name.clone(),
                expr: self.cdr_type.clone(),
            },
        )
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        todo!()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        todo!()
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        _lvl: usize,
        _b1: &alpha::Bindings,
        _b2: &alpha::Bindings,
    ) -> bool {
        if let Some(_other) = other.as_any().downcast_ref::<Self>() {
            todo!()
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl CoreInterface for Pair<Core> {
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
        &occurring_names(&self.0) | &occurring_names(&self.1)
    }

    fn val_of(&self, _env: &Env) -> Value {
        todo!()
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        let x = fresh_binder(ctx, &self.1, &Symbol::new("x"));
        let a_out = is_type(ctx, r, &self.0)?;
        let d_out = is_type(
            &ctx.bind_free(x.clone(), val_in_ctx(ctx, &a_out))?,
            r,
            &self.1,
        )?;
        Ok(cores::sigma(x, a_out, d_out))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let a = fresh(ctx, &Symbol::new("a"));
        let a_out = check(ctx, r, &self.0, &values::universe())?;
        let d_out = check(
            &ctx.bind_free(a.clone(), val_in_ctx(ctx, &a_out))?,
            r,
            &self.1,
            &values::universe(),
        )?;
        Ok((cores::universe(), cores::sigma(a, a_out, d_out)))
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        _lvl: usize,
        _b1: &alpha::Bindings,
        _b2: &alpha::Bindings,
    ) -> bool {
        if let Some(_other) = other.as_any().downcast_ref::<Self>() {
            todo!()
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl CoreInterface for Cons<Core> {
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
        &occurring_names(&self.0) | &occurring_names(&self.1)
    }

    fn val_of(&self, _env: &Env) -> Value {
        todo!()
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        Err(Error::NotAType(Core::new(self.clone())))
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Err(Error::CantDetermineType(Core::new(self.clone())))
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        if let Some(sigma) = now(tv).as_any().downcast_ref::<Sigma<Value, Closure>>() {
            let a_out = check(ctx, r, &self.0, &sigma.car_type)?;
            let d_out = check(
                ctx,
                r,
                &self.1,
                &sigma.cdr_type.val_of(val_in_ctx(ctx, &a_out)),
            )?;
            Ok(cores::cons(a_out, d_out))
        } else {
            Err(Error::NotASigmaType(tv.read_back_type(ctx).unwrap()))
        }
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        _lvl: usize,
        _b1: &alpha::Bindings,
        _b2: &alpha::Bindings,
    ) -> bool {
        if let Some(_other) = other.as_any().downcast_ref::<Self>() {
            todo!()
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl ValueInterface for Sigma<Value, Closure> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn ValueInterface) -> bool {
        unimplemented!()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        todo!()
    }

    fn read_back(&self, _ctx: &Ctx, _tv: &Value, _f: &Value) -> Result<Core> {
        todo!()
    }

    fn apply(
        &self,
        _ctx: &Ctx,
        _r: &Renaming,
        _rator_out: &Core,
        _rand: &Core,
    ) -> Result<(Core, Core)> {
        todo!()
    }
}

impl Display for Sigma<Core, Core> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Σ (({} {})) {})",
            self.arg_name.name(),
            self.car_type,
            self.cdr_type
        )
    }
}

impl Display for Pair<Core> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Pair {} {})", self.0, self.1)
    }
}

impl Display for Cons<Core> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} . {})", self.0, self.1)
    }
}
