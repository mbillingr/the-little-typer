use crate::alpha;
use crate::alpha::alpha_equiv_aux;
use crate::basics::{
    occurring_names, Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface,
};
use crate::errors::{Error, Result};
use crate::resugar::resugar_;
use crate::symbol::Symbol;
use crate::typechecker::check;
use crate::types::values::{add1, later};
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Formatter;
use std::option::Option::Some;
use std::result::Result::Err;

/// One more than another natural number
#[derive(Debug, Clone, PartialEq)]
pub struct Add1<T>(pub T);

impl CoreInterface for Add1<Core> {
    impl_core_defaults!(as_any, same);

    fn occurring_names(&self) -> HashSet<Symbol> {
        occurring_names(&self.0)
    }

    fn val_of(&self, env: &Env) -> Value {
        values::add1(later(env.clone(), self.0.clone()))
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        Err(Error::NotAType(Core::new(self.clone())))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        check(ctx, r, &self.0, &values::nat()).map(|n_out| (cores::nat(), Core::add1(n_out)))
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            alpha_equiv_aux(lvl, b1, b2, &self.0, &other.0)
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let (names, n) = resugar_(&self.0);
        (names, cores::add1(n))
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

impl std::fmt::Display for Add1<Core> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(add1 {})", self.0)
    }
}