use crate::basics::{Core, Ctx, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::symbol::Symbol;
use crate::types::values::quote;
use std::any::Any;

/// The type of atoms
#[derive(Debug)]
pub struct Atom;

/// Quotations are atoms
#[derive(Debug)]
pub struct Quote(pub Symbol);

impl ValueInterface for Atom {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Ok(Core::Atom)
    }

    fn read_back(&self, _ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
        if let Some(Quote(s)) = v.as_any().downcast_ref::<Quote>() {
            Ok(Core::Quote(s.clone()))
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
