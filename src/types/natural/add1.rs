use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::symbol::Symbol;
use crate::types::values::{add1, later};
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;
use std::option::Option::Some;
use std::result::Result::Err;

/// One more than another natural number
#[derive(Debug, Clone, PartialEq)]
pub struct Add1<T>(pub T);

impl CoreInterface for Add1<Core> {
    impl_core_defaults!(
        (0),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        no_type,
        check_by_synth,
        (resugar: add1)
    );

    fn val_of(&self, env: &Env) -> Value {
        values::add1(later(env.clone(), self.0.clone()))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let e = &self.0;
        let tv = &values::nat();
        e.check(ctx, r, tv)
            .map(|n_out| (cores::nat(), Core::add1(n_out)))
    }
}

impl ValueInterface for Add1<Value> {
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

impl_sexpr_display!(T: Add1<T>, ("add1", 0));
