use crate::alpha::alpha_equiv_aux;
use crate::basics::{Closure, Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::Error;
use crate::symbol::Symbol;
use crate::types::functions::Pi;
use crate::types::reference::NeutralVar;
use crate::types::values::lambda;
use crate::types::{cores, values};
use crate::{alpha, errors, resugar};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

/// An actual Function
#[derive(Debug, Clone, PartialEq)]
pub struct Lambda<B> {
    pub arg_name: Symbol,
    pub body: B,
}

/// Function with multiple arguments; desugars to nested `Lambda`s
#[derive(Debug, Clone, PartialEq)]
pub struct LambdaStar {
    pub params: Vec<Symbol>,
    pub body: Core,
}

impl<B> Lambda<B> {
    pub fn new(arg_name: Symbol, body: B) -> Self {
        Lambda { arg_name, body }
    }
}

impl CoreInterface for Lambda<Core> {
    impl_core_defaults!((arg_name, body), as_any, same, no_type, no_synth);

    fn occurring_names(&self) -> HashSet<Symbol> {
        let expr = &self.body;
        let mut names = expr.occurring_names();
        names.insert(self.arg_name.clone());
        names
    }

    fn val_of(&self, env: &Env) -> Value {
        values::lambda(
            self.arg_name.clone(),
            Closure::FirstOrder {
                env: env.clone(),
                var: self.arg_name.clone(),
                expr: self.body.clone(),
            },
        )
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> errors::Result<Core> {
        if let Some(pi) = tv.try_as::<Pi<Value, Closure>>() {
            let x_hat = ctx.fresh(&self.arg_name);
            let b_out = self.body.check(
                &ctx.bind_free(x_hat.clone(), pi.arg_type.clone())?,
                &r.extend(self.arg_name.clone(), x_hat.clone()),
                &pi.res_type.val_of(values::neutral(
                    pi.arg_type.clone(),
                    NeutralVar(x_hat.clone()),
                )),
            )?;
            Ok(Core::lambda(x_hat, b_out))
        } else {
            Err(Error::NotAFunctionType(tv.read_back_type(ctx)?))
        }
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            alpha_equiv_aux(
                1 + lvl,
                &b1.bind(&self.arg_name, lvl),
                &b2.bind(&other.arg_name, lvl),
                &self.body,
                &other.body,
            )
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let (mut names, r) = self.body.resugar();
        names.remove(&self.arg_name);
        (names, resugar::add_lambda(self.arg_name.clone(), r))
    }
}

impl CoreInterface for LambdaStar {
    impl_core_defaults!((), as_any, same, no_type, no_synth, no_alpha_equiv);

    fn occurring_names(&self) -> HashSet<Symbol> {
        let expr = &self.body;
        let mut names = expr.occurring_names();
        for p in &self.params {
            names.insert(p.clone());
        }
        names
    }

    fn val_of(&self, _env: &Env) -> Value {
        panic!("Attempt to evaluate lambda* (should have been desugared to `Lambda`s)")
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> errors::Result<Core> {
        match &self.params[..] {
            [] => panic!("nullary lambda"),
            [x] => Lambda::new(x.clone(), self.body.clone()).check(ctx, r, tv),
            [x, xs @ ..] => Lambda::new(
                x.clone(),
                cores::lambda_star(xs.to_vec(), self.body.clone()),
            )
            .check(ctx, r, tv),
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl Display for Lambda<Core> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(?? ({}) {})", self.arg_name.name(), self.body)
    }
}

impl Display for LambdaStar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(?? ({}) {})",
            self.params
                .iter()
                .map(|x| x.name())
                .collect::<Vec<_>>()
                .join(" "),
            self.body
        )
    }
}

impl ValueInterface for Lambda<Closure> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn ValueInterface) -> bool {
        unimplemented!()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> errors::Result<Core> {
        Err(Error::NotATypeVar(lambda(
            self.arg_name.clone(),
            self.body.clone(),
        )))
    }
}
