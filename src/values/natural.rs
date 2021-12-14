use crate::basics::{Core, Ctx, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::normalize::read_back;
use crate::values::{add1, zero};
use std::any::Any;

/// The type of all natural numbers
#[derive(Debug)]
pub struct Nat;

/// The natural number 0
#[derive(Debug)]
pub struct Zero;

/// One more than another natural number
#[derive(Debug)]
pub struct Add1(pub Value);

impl ValueInterface for Nat {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Ok(Core::Nat)
    }

    fn read_back(&self, ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
        if v.as_any().downcast_ref::<Zero>().is_some() {
            Ok(Core::Zero)
        } else if let Some(Add1(n)) = v.as_any().downcast_ref::<Add1>() {
            Ok(Core::add1(read_back(ctx, tv, n)))
        } else {
            Err(Error::TypeMismatchVar(v.clone(), tv.clone()))
        }
    }
}

impl ValueInterface for Zero {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Err(Error::NotATypeVar(zero()))
    }
}

impl ValueInterface for Add1 {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        if let Some(Add1(n)) = other.as_any().downcast_ref::<Self>() {
            &self.0 == n
        } else {
            false
        }
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Err(Error::NotATypeVar(add1(self.0.clone())))
    }
}
