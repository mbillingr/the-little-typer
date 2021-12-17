use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::Error;
use crate::symbol::Symbol;
use crate::types::values::zero;
use crate::types::{cores, values};
use crate::{alpha, errors};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Formatter;

/// The natural number 0
#[derive(Debug, Copy, Clone)]
pub struct Zero;

impl CoreInterface for Zero {
    impl_core_defaults!(as_any, (same unique));

    fn occurring_names(&self) -> HashSet<Symbol> {
        HashSet::new()
    }

    fn val_of(&self, _env: &Env) -> Value {
        values::zero()
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> errors::Result<Core> {
        Err(Error::NotAType(Core::new(self.clone())))
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> errors::Result<(Core, Core)> {
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

impl std::fmt::Display for Zero {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zero")
    }
}
