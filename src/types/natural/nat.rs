use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::Error;
use crate::normalize::read_back;
use crate::symbol::Symbol;
use crate::types::natural::zero::Zero;
use crate::types::natural::Add1;
use crate::types::{cores, values};
use crate::{alpha, errors};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Formatter;

/// The type of all natural numbers
#[derive(Debug, Copy, Clone)]
pub struct Nat;

impl CoreInterface for Nat {
    impl_core_defaults!(_, as_any, same, occurring_names, alpha_equiv, simple_type);

    fn val_of(&self, _env: &Env) -> Value {
        values::nat()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> errors::Result<(Core, Core)> {
        Ok((cores::universe(), cores::nat()))
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        (HashSet::new(), cores::nat())
    }
}

impl ValueInterface for Nat {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> errors::Result<Core> {
        Ok(cores::nat())
    }

    fn read_back(&self, ctx: &Ctx, tv: &Value, v: &Value) -> errors::Result<Core> {
        if v.as_any().downcast_ref::<Zero>().is_some() {
            Ok(cores::zero())
        } else if let Some(Add1(n)) = v.as_any().downcast_ref::<Add1<Value>>() {
            Ok(cores::add1(read_back(ctx, tv, n)))
        } else {
            Err(Error::TypeMismatchVar(v.clone(), tv.clone()))
        }
    }
}

impl std::fmt::Display for Nat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nat")
    }
}
