use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, R};
use crate::errors::{Error, Result};
use crate::symbol::Symbol;
use std::collections::HashSet;
use std::fmt::Formatter;
use crate::alpha;

/// The type of all types
#[derive(Debug, Clone)]
pub struct Invalid(pub R<str>);

impl CoreInterface for Invalid {
    impl_core_defaults!(_, as_any, occurring_names);

    fn same(&self, _: &dyn CoreInterface) -> bool {
        false
    }

    fn val_of(&self, _env: &Env) -> Value {
        unimplemented!()
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        Err(Error::InvalidSyntax(self.0.clone()))
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Err(Error::InvalidSyntax(self.0.clone()))
    }

    fn check(&self, _ctx: &Ctx, _r: &Renaming, _tv: &Value) -> Result<Core> {
        Err(Error::InvalidSyntax(self.0.clone()))
    }

    fn alpha_equiv_aux(
        &self,
        _other: &dyn CoreInterface,
        _lvl: usize,
        _b1: &alpha::Bindings,
        _b2: &alpha::Bindings,
    ) -> bool {
        false
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        unimplemented!()
    }
}

impl std::fmt::Display for Invalid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "U")
    }
}
