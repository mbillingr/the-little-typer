use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::symbol::Symbol;
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Formatter;

/// The type of all types
#[derive(Debug, Copy, Clone)]
pub struct Universe;

impl ValueInterface for Universe {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Ok(cores::universe())
    }

    fn read_back(&self, ctx: &Ctx, _tv: &Value, v: &Value) -> Result<Core> {
        Ok(v.read_back_type(ctx)?)
    }
}

impl CoreInterface for Universe {
    impl_core_defaults!(
        _,
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        simple_type,
        check_by_synth
    );

    fn val_of(&self, _env: &Env) -> Value {
        values::universe()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Err(Error::UhasNoType)
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        (HashSet::new(), Core::new(*self))
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "U")
    }
}
