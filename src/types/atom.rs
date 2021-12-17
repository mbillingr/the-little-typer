use crate::alpha;
use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::symbol::Symbol;
use crate::typechecker::atom_is_ok;
use crate::types::values::quote;
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Formatter;

/// The type of atoms
#[derive(Debug, Copy, Clone)]
pub struct Atom;

/// Quotations are atoms
#[derive(Debug, Clone, PartialEq)]
pub struct Quote(pub Symbol);

impl CoreInterface for Atom {
    impl_core_defaults!(_, as_any, same, occurring_names, alpha_equiv, simple_type);

    fn val_of(&self, _env: &Env) -> Value {
        values::atom()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Ok((cores::universe(), cores::atom()))
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        (HashSet::new(), Core::new(*self))
    }
}

impl CoreInterface for Quote {
    impl_core_defaults!((), as_any, same, occurring_names, no_type);

    fn val_of(&self, _env: &Env) -> Value {
        values::quote(self.0.clone())
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        if atom_is_ok(&self.0) {
            Ok((cores::atom(), cores::quote(self.0.clone())))
        } else {
            Err(Error::InvalidAtom(self.0.clone()))
        }
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        _lvl: usize,
        _b1: &alpha::Bindings,
        _b2: &alpha::Bindings,
    ) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.0 == other.0
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        (HashSet::new(), Core::new(self.clone()))
    }
}

impl ValueInterface for Atom {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Ok(cores::atom())
    }

    fn read_back(&self, _ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
        if let Some(Quote(s)) = v.as_any().downcast_ref::<Quote>() {
            Ok(cores::quote(s.clone()))
        } else {
            Err(Error::TypeMismatchVar(v.clone(), tv.clone()))
        }
    }
}

impl ValueInterface for Quote {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        if let Some(Quote(s)) = other.as_any().downcast_ref::<Self>() {
            &self.0 == s
        } else {
            false
        }
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Err(Error::NotATypeVar(quote(self.0.clone())))
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Atom")
    }
}

impl std::fmt::Display for Quote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}", self.0.name())
    }
}
