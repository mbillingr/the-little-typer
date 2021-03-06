use crate::alpha::alpha_equiv_aux;
use crate::basics::{Closure, Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::normalize::{read_back, val_in_ctx};
use crate::symbol::Symbol;
use crate::types::functions::lambda::Lambda;
use crate::types::reference::NeutralVar;
use crate::types::values::later;
use crate::types::{
    check_with_fresh_binding, cores, functions, is_type_with_fresh_binding, neutral,
    occurring_binder_names, values,
};
use crate::{alpha, errors, resugar, types};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Pi<T, C> {
    pub arg_name: Symbol,
    pub arg_type: T,
    pub res_type: C,
}

#[derive(Debug, PartialEq)]
pub struct PiStar {
    pub binders: Vec<(Symbol, Core)>,
    pub res_type: Core,
}

impl CoreInterface for Pi<Core, Core> {
    impl_core_defaults!((fun, arg), as_any, same, check_by_synth);

    fn occurring_names(&self) -> HashSet<Symbol> {
        &occurring_binder_names(&self.arg_name, &self.arg_type) | &self.res_type.occurring_names()
    }

    fn val_of(&self, env: &Env) -> Value {
        let av = later(env.clone(), self.arg_type.clone());
        values::pi(
            self.arg_name.clone(),
            av,
            Closure::FirstOrder {
                env: env.clone(),
                var: self.arg_name.clone(),
                expr: self.res_type.clone(),
            },
        )
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> errors::Result<Core> {
        let (y, a_out, b_out) =
            is_type_with_fresh_binding(ctx, r, &self.arg_name, &self.arg_type, &self.res_type)?;
        Ok(Core::pi(y, a_out, b_out))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> errors::Result<(Core, Core)> {
        let (x_hat, a_out, b_out) =
            check_with_fresh_binding(ctx, r, &self.arg_name, &self.arg_type, &self.res_type)?;
        Ok((cores::universe(), Core::pi(x_hat, a_out, b_out)))
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            alpha_equiv_aux(lvl, b1, b2, &self.arg_type, &other.arg_type)
                && alpha_equiv_aux(
                    1 + lvl,
                    &b1.bind(&self.arg_name, lvl),
                    &b2.bind(&other.arg_name, lvl),
                    &self.res_type,
                    &other.res_type,
                )
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let arg = self.arg_type.resugar();
        let mut res = self.res_type.resugar();
        if res.0.contains(&self.arg_name) {
            res.0.remove(&self.arg_name);
            (
                &arg.0 | &res.0,
                resugar::add_pi(self.arg_name.clone(), arg.1, res.1),
            )
        } else {
            (&arg.0 | &res.0, resugar::add_fun(arg.1, res.1))
        }
    }
}

impl CoreInterface for PiStar {
    impl_core_defaults!((), as_any, same, check_by_synth, no_alpha_equiv);

    fn occurring_names(&self) -> HashSet<Symbol> {
        self.binders
            .iter()
            .map(|(x, t)| types::occurring_binder_names(x, t))
            .fold(self.res_type.occurring_names(), |a, b| &a | &b)
    }

    fn val_of(&self, _env: &Env) -> Value {
        panic!("Attempt to evaluate Pi* (should have been desugared to `Pi`s)")
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> errors::Result<Core> {
        match &self.binders[..] {
            [] => unimplemented!(),
            [(x, a)] => Pi {
                arg_name: x.clone(),
                arg_type: a.clone(),
                res_type: self.res_type.clone(),
            }
            .is_type(ctx, r),
            [(x, a), more @ ..] => {
                let body = cores::pi_star(more.to_vec(), self.res_type.clone());

                let (z, a_out, b_out) = is_type_with_fresh_binding(ctx, r, x, a, &body)?;

                Ok(cores::pi(z, a_out, b_out))
            }
        }
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> errors::Result<(Core, Core)> {
        match &self.binders[..] {
            [] => unreachable!(),
            [(x, a)] => Pi {
                arg_name: x.clone(),
                arg_type: a.clone(),
                res_type: self.res_type.clone(),
            }
            .synth(ctx, r),
            [(x, a), more @ ..] => {
                let body = PiStar {
                    binders: more.to_vec(),
                    res_type: self.res_type.clone(),
                };

                let (x_hat, a_out, b_out) = check_with_fresh_binding(ctx, r, x, a, &body)?;

                Ok((cores::universe(), Core::pi(x_hat, a_out, b_out)))
            }
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        match &self.binders[..] {
            [(x, arg_type)] => resugar_unary_pi(x, arg_type, &self.res_type),
            _ => todo!(),
        }
    }
}

impl Display for Pi<Core, Core> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(?? (({} {})) {})",
            self.arg_name.name(),
            self.arg_type,
            self.res_type
        )
    }
}

impl Display for PiStar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let b: Vec<_> = self
            .binders
            .iter()
            .map(|(x, t)| format!("({} {})", x.name(), t))
            .collect();
        write!(f, "(?? ({}) {})", b.join(" "), self.res_type)
    }
}

impl ValueInterface for Pi<Value, Closure> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn ValueInterface) -> bool {
        unimplemented!()
    }

    fn read_back_type(&self, ctx: &Ctx) -> errors::Result<Core> {
        let ae = self.arg_type.read_back_type(ctx)?;
        let x_hat = ctx.fresh(&self.arg_name);

        let ctx_hat = ctx.bind_free(x_hat.clone(), self.arg_type.clone()).unwrap();
        let r = self
            .res_type
            .val_of(neutral::neutral(
                self.arg_type.clone(),
                NeutralVar(x_hat.clone()),
            ))
            .read_back_type(&ctx_hat)?;
        Ok(Core::pi(x_hat, ae, r))
    }

    fn read_back(&self, ctx: &Ctx, _tv: &Value, f: &Value) -> errors::Result<Core> {
        let y = match f.as_any().downcast_ref::<Lambda<Closure>>() {
            Some(lam) => &lam.arg_name,
            None => &self.arg_name,
        };

        let x = y;
        let x_hat = ctx.fresh(x);

        let body = read_back(
            &ctx.bind_free(x_hat.clone(), self.arg_type.clone()).unwrap(),
            &self.res_type.val_of(neutral::neutral(
                self.arg_type.clone(),
                NeutralVar(x_hat.clone()),
            )),
            &functions::do_ap(
                f,
                neutral::neutral(self.arg_type.clone(), NeutralVar(x_hat.clone())),
            ),
        )?;

        Ok(Core::lambda(x_hat, body))
    }

    fn apply(
        &self,
        ctx: &Ctx,
        r: &Renaming,
        rator_out: &Core,
        rand: &Core,
    ) -> errors::Result<(Core, Core)> {
        let rand_out = rand.check(ctx, r, &self.arg_type)?;
        Ok((
            self.res_type
                .val_of(val_in_ctx(ctx, &rand_out))
                .read_back_type(ctx)?,
            Core::app((*rator_out).clone(), rand_out),
        ))
    }
}

fn resugar_unary_pi(x: &Symbol, arg_type: &Core, result_type: &Core) -> (HashSet<Symbol>, Core) {
    let term = arg_type;
    let arg = term.resugar();
    let term = result_type;
    let res = term.resugar();
    if res.0.contains(x) {
        todo!()
    } else {
        (&arg.0 | &res.0, resugar::add_fun(arg.1, res.1))
    }
}
