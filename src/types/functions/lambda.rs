use crate::alpha::alpha_equiv_aux;
use crate::basics::{
    fresh, occurring_binder_names, Closure, Core, CoreInterface, Ctx, Env, Renaming, Value,
    ValueInterface, N,
};
use crate::errors::Error;
use crate::normalize::now;
use crate::symbol::Symbol;
use crate::typechecker::check;
use crate::types::functions::Pi;
use crate::types::values;
use crate::types::values::lambda;
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

impl CoreInterface for Lambda<Core> {
    impl_core_defaults!((arg_name, body), as_any, same, no_type);

    fn occurring_names(&self) -> HashSet<Symbol> {
        occurring_binder_names(&self.arg_name, &self.body)
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

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> errors::Result<(Core, Core)> {
        Err(Error::CantDetermineType(Core::new(self.clone())))
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> errors::Result<Core> {
        if let Some(pi) = now(tv).as_any().downcast_ref::<Pi<Value, Closure>>() {
            let x_hat = fresh(ctx, &self.arg_name);
            let b_out = check(
                &ctx.bind_free(x_hat.clone(), pi.arg_type.clone())?,
                &r.extend(self.arg_name.clone(), x_hat.clone()),
                &self.body,
                &pi.res_type
                    .val_of(values::neutral(pi.arg_type.clone(), N::Var(x_hat.clone()))),
            )?;
            Ok(Core::lambda(x_hat, b_out))
        } else {
            Err(Error::NotAFunctionType(tv.read_back_type(ctx).unwrap()))
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
        let (mut names, r) = resugar::resugar_(&self.body);
        names.remove(&self.arg_name);
        (names, resugar::add_lambda(self.arg_name.clone(), r))
    }
}

impl Display for Lambda<Core> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(λ ({}) {})", self.arg_name.name(), self.body)
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
