use crate::basics::{
    Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, The, Value, ValueInterface, N,
};
use crate::errors;
use crate::errors::Error;
use crate::normalize::{read_back, val_in_ctx};
use crate::symbol::Symbol;
use crate::types::values::later;
use crate::types::{cores, functions, values};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

/// A function application
#[derive(Debug, Clone, PartialEq)]
pub struct App {
    pub fun: Core,
    pub arg: Core,
}

#[derive(Debug)]
pub struct NeutralApp(pub N, pub The);

/// N-ary function application; desugars to series of `App`s
#[derive(Debug, Clone, PartialEq)]
pub struct AppStar {
    pub fun: Core,
    pub args: Vec<Core>,
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
        let f = self.fun.resugar();
        let a = self.arg.resugar();
        (&f.0 | &a.0, cores::app(f.1, a.1))
    }
}

impl CoreInterface for AppStar {
    impl_core_defaults!((fun, arg), as_any, same, no_alpha_equiv, check_by_synth);

    fn occurring_names(&self) -> HashSet<Symbol> {
        let mut names = self.fun.occurring_names();
        for arg in &self.args {
            names = &names | &arg.occurring_names();
        }
        names
    }

    fn val_of(&self, _env: &Env) -> Value {
        panic!("Attempt to evaluate n-ary application (should have been desugared to `App`)")
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> errors::Result<Core> {
        match self.check(ctx, r, &values::universe()) {
            Ok(t_out) => Ok(t_out),
            Err(_) => Err(Error::NotAType(Core::new(self.clone()))),
        }
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> errors::Result<(Core, Core)> {
        let inp = &self.fun;
        let (rator_t, rator_out) = inp.synth(ctx, r)?;
        match &self.args[..] {
            [] => Err(Error::WrongArity(Core::app_star(rator_out, vec![]))),
            [rand] => val_in_ctx(ctx, &rator_t).apply(ctx, r, &rator_out, rand),
            [_rand0, _rands @ ..] => todo!(),
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.fun, self.arg)
    }
}

impl Display for AppStar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}", self.fun)?;
        for arg in &self.args {
            write!(f, " {}", arg)?;
        }
        write!(f, ")")
    }
}

impl NeutralInterface for NeutralApp {
    fn read_back_neutral(&self, ctx: &Ctx) -> errors::Result<Core> {
        let fun = &self.0;
        let The(typ, val) = &self.1;
        Ok(Core::app(
            fun.read_back_neutral(ctx)?,
            read_back(ctx, typ, val)?,
        ))
    }
}
