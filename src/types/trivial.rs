use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::types::{cores, values};
use std::any::Any;

/// The trivial type (also known as unit type)
#[derive(Debug, Copy, Clone)]
pub struct Trivial;

/// The only trivial value
#[derive(Debug, Clone, PartialEq)]
pub struct Sole;

impl CoreInterface for Trivial {
    impl_core_defaults!(
        _,
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        simple_type,
        check_by_synth,
        (resugar: trivial)
    );

    fn val_of(&self, _env: &Env) -> Value {
        values::trivial()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Ok((cores::universe(), cores::trivial()))
    }
}

impl CoreInterface for Sole {
    impl_core_defaults!(
        _,
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        no_type,
        check_by_synth,
        (resugar: sole)
    );

    fn val_of(&self, _env: &Env) -> Value {
        values::sole()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Ok((cores::trivial(), cores::sole()))
    }
}

impl ValueInterface for Trivial {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Ok(cores::trivial())
    }

    fn read_back(&self, _ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
        if v.as_any().downcast_ref::<Sole>().is_some() {
            Ok(cores::sole())
        } else {
            Err(Error::TypeMismatchVar(v.clone(), tv.clone()))
        }
    }
}

impl ValueInterface for Sole {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        unreachable!()
    }
}

impl_sexpr_display!(Trivial, "Trivial");
impl_sexpr_display!(Sole, "sole");
