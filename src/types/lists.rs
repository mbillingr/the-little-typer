use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::normalize::{read_back, val_in_ctx};
use crate::symbol::Symbol;
use crate::types::values::later;
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

/// The type of lists
#[derive(Debug, Clone, PartialEq)]
pub struct List<T>(pub T);

/// The empty list
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Nil;

/// The type of lists
#[derive(Debug, Clone, PartialEq)]
pub struct ListCons<T>(pub T, pub T);

impl CoreInterface for List<Core> {
    impl_core_defaults!(
        (0),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        check_by_synth
    );

    fn val_of(&self, env: &Env) -> Value {
        values::list(later(env.clone(), self.0.clone()))
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        let e_out = self.0.is_type(ctx, r)?;
        Ok(cores::list(e_out))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let e_out = self.0.check(ctx, r, &values::universe())?;
        Ok((cores::universe(), cores::list(e_out)))
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let t = self.0.resugar();
        (t.0, cores::list(t.1))
    }
}

impl CoreInterface for Nil {
    impl_core_defaults!(_, as_any, same, occurring_names, alpha_equiv, no_type);

    fn val_of(&self, _env: &Env) -> Value {
        values::nil()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Err(Error::CantDetermineType(Core::new(self.clone())))
    }

    fn check(&self, ctx: &Ctx, _r: &Renaming, tv: &Value) -> Result<Core> {
        if tv.try_as::<List<Value>>().is_some() {
            Ok(cores::nil())
        } else {
            Err(Error::NotAListType(tv.read_back_type(ctx).unwrap()))
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        (HashSet::new(), cores::nil())
    }
}

impl CoreInterface for ListCons<Core> {
    impl_core_defaults!(
        (0, 1),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        no_type,
        check_by_synth
    );

    fn val_of(&self, env: &Env) -> Value {
        values::list_cons(
            later(env.clone(), self.0.clone()),
            later(env.clone(), self.1.clone()),
        )
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let (e, e_out) = self.0.synth(ctx, r)?;
        let lt = cores::list(e);
        let es_out = self.1.check(ctx, r, &val_in_ctx(ctx, &lt))?;
        Ok((lt, cores::list_cons(e_out, es_out)))
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let h = self.0.resugar();
        let t = self.1.resugar();
        (&h.0 | &t.0, cores::list_cons(h.1, t.1))
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(List {})", self.0)
    }
}

impl std::fmt::Display for Nil {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "nil")
    }
}

impl<T: Display> Display for ListCons<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(:: {} {})", self.0, self.1)
    }
}

impl ValueInterface for List<Value> {
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
        Ok(cores::list(self.0.read_back_type(ctx)?))
    }

    fn read_back(&self, ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
        if v.as_any().downcast_ref::<Nil>().is_some() {
            Ok(cores::nil())
        } else if let Some(ListCons(h, t)) = v.as_any().downcast_ref::<ListCons<Value>>() {
            Ok(cores::list_cons(
                read_back(ctx, &self.0, h)?,
                self.read_back(ctx, tv, t)?,
            ))
        } else {
            Err(Error::TypeMismatchVar(v.clone(), tv.clone()))
        }
    }
}

impl ValueInterface for Nil {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Err(Error::NotATypeVar(values::nil()))
    }
}

impl ValueInterface for ListCons<Value> {
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
        Err(Error::NotATypeVar(values::list_cons(
            self.0.clone(),
            self.1.clone(),
        )))
    }
}
