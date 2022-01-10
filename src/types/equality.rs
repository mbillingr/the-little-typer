use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::normalize::{read_back, val_in_ctx};
use crate::symbol::Symbol;
use crate::typechecker::convert;
use crate::types::values::later;
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Equal<T> {
    pub typ: T,
    pub from: T,
    pub to: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Same<T>(pub T);

impl CoreInterface for Equal<Core> {
    impl_core_defaults!((typ, from, to), as_any, same, occurring_names, alpha_equiv);

    fn val_of(&self, env: &Env) -> Value {
        values::equal(
            later(env.clone(), self.typ.clone()),
            later(env.clone(), self.from.clone()),
            later(env.clone(), self.to.clone()),
        )
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        let a_out = self.typ.is_type(ctx, r)?;
        let av = val_in_ctx(ctx, &a_out);
        let from_out = self.from.check(ctx, r, &av)?;
        let to_out = self.to.check(ctx, r, &av)?;
        Ok(cores::equal(a_out, from_out, to_out))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let a_out = self.typ.check(ctx, r, &values::universe())?;
        let av = val_in_ctx(ctx, &a_out);
        let from_out = self.from.check(ctx, r, &av)?;
        let to_out = self.to.check(ctx, r, &av)?;
        Ok((cores::universe(), cores::equal(a_out, from_out, to_out)))
    }

    fn check(&self, _ctx: &Ctx, _r: &Renaming, _tv: &Value) -> Result<Core> {
        todo!()
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl CoreInterface for Same<Core> {
    impl_core_defaults!((0), as_any, same, occurring_names, alpha_equiv, no_type);

    fn val_of(&self, env: &Env) -> Value {
        values::same(later(env.clone(), self.0.clone()))
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        todo!()
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        if let Some(Equal {
            typ: av,
            from: fromv,
            to: tov,
        }) = tv.try_as::<Equal<Value>>()
        {
            let c_out = self.0.check(ctx, r, av)?;
            let v = val_in_ctx(ctx, &c_out);
            convert(ctx, av, fromv, &v)?;
            convert(ctx, av, tov, &v)?;
            Ok(cores::same(c_out))
        } else {
            Err(Error::NotAnEqualType(tv.read_back_type(ctx)?))
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl<T: Display> Display for Equal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(= {} {} {})", self.typ, self.from, self.to)
    }
}

impl<T: Display> Display for Same<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(same {})", self.0)
    }
}

impl ValueInterface for Equal<Value> {
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
        Ok(cores::equal(
            self.typ.read_back_type(ctx)?,
            read_back(ctx, &self.typ, &self.from)?,
            read_back(ctx, &self.typ, &self.to)?,
        ))
    }

    fn read_back(&self, _ctx: &Ctx, _tv: &Value, _pv: &Value) -> Result<Core> {
        todo!()
    }
}

impl ValueInterface for Same<Value> {
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
        todo!()
    }

    fn read_back(&self, _ctx: &Ctx, _tv: &Value, _pv: &Value) -> Result<Core> {
        todo!()
    }
}
