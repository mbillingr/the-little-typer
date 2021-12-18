use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value};
use crate::errors::{Error, Result};
use crate::normalize::{val_in_ctx, val_of};
use crate::resugar::resugar_;
use crate::symbol::Symbol;
use crate::typechecker::{check, is_type};
use crate::types::{cores, values};
use std::collections::HashSet;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub struct The {
    pub typ: Core,
    pub exp: Core,
}

impl CoreInterface for The {
    impl_core_defaults!(
        (typ, exp),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        check_by_synth
    );

    fn val_of(&self, env: &Env) -> Value {
        val_of(env, &self.exp)
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        match self.check(ctx, r, &values::universe()) {
            Ok(t_out) => Ok(t_out),
            Err(_) => Err(Error::NotAType(Core::new(self.clone()))),
        }
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let t_out = is_type(ctx, r, &self.typ)?;
        let e_out = check(ctx, r, &self.exp, &val_in_ctx(ctx, &t_out))?;
        Ok((t_out, e_out))
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let t = resugar_(&self.typ);
        let e = resugar_(&self.exp);
        (&t.0 | &e.0, cores::the(t.1, e.1))
    }
}

impl std::fmt::Display for The {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(the {} {})", self.typ, self.exp)
    }
}
