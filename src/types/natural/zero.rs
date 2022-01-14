use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors;
use crate::errors::Error;
use crate::symbol::Symbol;
use crate::types::values::zero;
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;

/// The natural number 0
#[derive(Debug, Copy, Clone)]
pub struct Zero;

impl CoreInterface for Zero {
    impl_core_defaults!(
        _,
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        no_type,
        check_by_synth
    );

    fn val_of(&self, _env: &Env) -> Value {
        values::zero()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> errors::Result<(Core, Core)> {
        Ok((cores::nat(), cores::zero()))
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        (HashSet::new(), cores::zero())
    }
}

impl ValueInterface for Zero {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> errors::Result<Core> {
        Err(Error::NotATypeVar(zero()))
    }
}

impl_sexpr_display!(Zero, "zero");
