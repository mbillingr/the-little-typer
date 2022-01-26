use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors;
use crate::errors::Error;
use crate::normalize::read_back;
use crate::types::natural::zero::Zero;
use crate::types::natural::Add1;
use crate::types::{cores, values};
use std::any::Any;

/// The type of all natural numbers
#[derive(Debug, Copy, Clone)]
pub struct Nat;

impl CoreInterface for Nat {
    impl_core_defaults!(
        _,
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        simple_type,
        check_by_synth,
        (resugar: nat)
    );

    fn val_of(&self, _env: &Env) -> Value {
        values::nat()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> errors::Result<(Core, Core)> {
        Ok((cores::universe(), cores::nat()))
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
            read_back(ctx, tv, n).map(cores::add1)
        } else {
            Err(Error::TypeMismatchVar(v.clone(), tv.clone()))
        }
    }
}

impl_sexpr_display!(Nat, "Nat");
