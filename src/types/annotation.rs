use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value};
use crate::errors::{Error, Result};
use crate::normalize::val_in_ctx;
use crate::symbol::Symbol;
use crate::types::absurd::Absurd;
use crate::types::{cores, values};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct The {
    pub typ: Core,
    pub exp: Core,
}

impl CoreInterface for The {
    impl_core_defaults!((typ, exp), as_any, same, occurring_names, check_by_synth);

    fn val_of(&self, env: &Env) -> Value {
        let e = &self.exp;
        e.val_of(env)
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        match self.check(ctx, r, &values::universe()) {
            Ok(t_out) => Ok(t_out),
            Err(_) => Err(Error::NotAType(Core::new(self.clone()))),
        }
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let inp = &self.typ;
        let t_out = inp.is_type(ctx, r)?;
        let e = &self.exp;
        let tv = &val_in_ctx(ctx, &t_out);
        let e_out = e.check(ctx, r, tv)?;
        Ok((t_out, e_out))
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        lvl: usize,
        b1: &crate::alpha::Bindings,
        b2: &crate::alpha::Bindings,
    ) -> bool {
        if let Some(other) = other.try_as::<Self>() {
            if self.typ.try_as::<Absurd>().is_some() && other.typ.try_as::<Absurd>().is_some() {
                true
            } else {
                self.typ.alpha_equiv_aux(&other.typ, lvl, b1, b2)
                    && self.exp.alpha_equiv_aux(&other.exp, lvl, b1, b2)
            }
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let term = &self.typ;
        let t = term.resugar();
        let term = &self.exp;
        let e = term.resugar();
        (&t.0 | &e.0, cores::the(t.1, e.1))
    }
}

impl_sexpr_display!(The, ("the", typ, exp));
