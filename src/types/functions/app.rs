use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value};
use crate::errors::Error;
use crate::symbol::Symbol;
use crate::types::values::later;
use crate::types::{cores, functions, values};
use crate::{errors, resugar};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

/// A function application
#[derive(Debug, Clone, PartialEq)]
pub struct App {
    pub fun: Core,
    pub arg: Core,
}

impl CoreInterface for App {
    impl_core_defaults!(
        (fun, arg),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        check_by_synth
    );

    fn val_of(&self, env: &Env) -> Value {
        functions::do_ap(
            &later(env.clone(), self.fun.clone()),
            later(env.clone(), self.arg.clone()),
        )
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> errors::Result<Core> {
        match self.check(ctx, r, &values::universe()) {
            Ok(t_out) => Ok(t_out),
            Err(_) => Err(Error::NotAType(Core::new(self.clone()))),
        }
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> errors::Result<(Core, Core)> {
        panic!("use AppStar for synthesis")
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let f = resugar::resugar_(&self.fun);
        let a = resugar::resugar_(&self.arg);
        (&f.0 | &a.0, cores::app(f.1, a.1))
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.fun, self.arg)
    }
}
