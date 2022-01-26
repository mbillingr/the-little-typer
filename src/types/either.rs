use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::normalize::read_back;
use crate::symbol::Symbol;
use crate::types::values::later;
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Either<T>(pub T, pub T);

#[derive(Debug, Clone, PartialEq)]
pub struct Left<T>(pub T);

#[derive(Debug, Clone, PartialEq)]
pub struct Right<T>(pub T);

impl CoreInterface for Either<Core> {
    impl_core_defaults!(
        (0, 1),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        check_by_synth,
        (resugar: either)
    );

    fn val_of(&self, env: &Env) -> Value {
        values::either(
            later(env.clone(), self.0.clone()),
            later(env.clone(), self.1.clone()),
        )
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        let l_out = self.0.is_type(ctx, r)?;
        let r_out = self.1.is_type(ctx, r)?;
        Ok(cores::either(l_out, r_out))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let u = values::universe();
        let l_out = self.0.check(ctx, r, &u)?;
        let r_out = self.1.check(ctx, r, &u)?;
        Ok((cores::universe(), cores::either(l_out, r_out)))
    }
}

impl CoreInterface for Left<Core> {
    impl_core_defaults!((0), as_any, same, occurring_names, no_synth, alpha_equiv);

    fn val_of(&self, env: &Env) -> Value {
        values::left(later(env.clone(), self.0.clone()))
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        unimplemented!()
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        if let Some(Either(ltv, _)) = tv.try_as::<Either<Value>>() {
            let l_out = self.0.check(ctx, r, ltv)?;
            Ok(cores::left(l_out))
        } else {
            Err(Error::NotAnEitherType(tv.read_back_type(ctx).unwrap()))
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        unimplemented!()
    }
}

impl CoreInterface for Right<Core> {
    impl_core_defaults!((0), as_any, same, occurring_names, no_synth, alpha_equiv);

    fn val_of(&self, env: &Env) -> Value {
        values::right(later(env.clone(), self.0.clone()))
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        unimplemented!()
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        if let Some(Either(_, rtv)) = tv.try_as::<Either<Value>>() {
            let r_out = self.0.check(ctx, r, rtv)?;
            Ok(cores::right(r_out))
        } else {
            Err(Error::NotAnEitherType(tv.read_back_type(ctx).unwrap()))
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        unimplemented!()
    }
}

impl_sexpr_display!(T: Either<T>, ("Either", 0, 1));
impl_sexpr_display!(T: Left<T>, ("left", 0));
impl_sexpr_display!(T: Right<T>, ("right", 0));

impl ValueInterface for Either<Value> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        if let Some(other) = other.try_as::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn read_back_type(&self, ctx: &Ctx) -> Result<Core> {
        Ok(cores::either(
            self.0.read_back_type(ctx)?,
            self.1.read_back_type(ctx)?,
        ))
    }

    fn read_back(&self, ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
        if let Some(Left(lv)) = v.try_as::<Left<Value>>() {
            return Ok(cores::left(read_back(ctx, &self.0, lv)?));
        }

        if let Some(Right(rv)) = v.try_as::<Right<Value>>() {
            return Ok(cores::right(read_back(ctx, &self.1, rv)?));
        }

        Err(Error::TypeMismatchVar(v.clone(), tv.clone()))
    }
}

impl ValueInterface for Left<Value> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        if let Some(other) = other.try_as::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        unimplemented!()
    }
}

impl ValueInterface for Right<Value> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        if let Some(other) = other.try_as::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        unimplemented!()
    }
}
