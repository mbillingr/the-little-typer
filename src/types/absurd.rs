use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::types::{cores, values};
use std::any::Any;

/// The absurd type (also known as empty type)
#[derive(Debug, Copy, Clone)]
pub struct Absurd;

/// The only trivial value
#[derive(Debug, Clone, PartialEq)]
pub struct Sole;

impl CoreInterface for Absurd {
    impl_core_defaults!(
        _,
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        simple_type,
        check_by_synth,
        (resugar: absurd)
    );

    fn val_of(&self, _env: &Env) -> Value {
        values::absurd()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Ok((cores::universe(), cores::absurd()))
    }
}

impl_sexpr_display!(Absurd, "Absurd");

impl ValueInterface for Absurd {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Ok(cores::absurd())
    }

    fn read_back(&self, ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
        if let Some((_, ne)) = v.as_neutral() {
            Ok(cores::the(cores::absurd(), ne.read_back_neutral(ctx)?))
        } else {
            Err(Error::TypeMismatchVar(v.clone(), tv.clone()))
        }
    }
}
