use crate::alpha::alpha_equiv_aux;
use crate::basics::{
    fresh, occurring_binder_names, occurring_names, Closure, Core, CoreInterface, Ctx, Env,
    Renaming, Value, ValueInterface, N,
};
use crate::normalize::{read_back, read_back_type, val_in_ctx};
use crate::symbol::Symbol;
use crate::typechecker::{check, is_type};
use crate::types::functions::lambda::Lambda;
use crate::types::values::later;
use crate::types::{cores, functions, neutral, values};
use crate::{alpha, errors, resugar};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Pi<T, C> {
    pub arg_name: Symbol,
    pub arg_type: T,
    pub res_type: C,
}

impl CoreInterface for Pi<Core, Core> {
    impl_core_defaults!((fun, arg), as_any, same, check_by_synth);

    fn occurring_names(&self) -> HashSet<Symbol> {
        &occurring_binder_names(&self.arg_name, &self.arg_type) | &occurring_names(&self.res_type)
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
        let y = fresh(ctx, &self.arg_name);
        let a_out = is_type(ctx, r, &self.arg_type)?;
        let a_outv = val_in_ctx(ctx, &a_out);
        let b_out = is_type(
            &ctx.bind_free(y.clone(), a_outv)?,
            &r.extend(self.arg_name.clone(), y.clone()),
            &self.res_type,
        )?;
        Ok(Core::pi(y, a_out, b_out))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> errors::Result<(Core, Core)> {
        let x_hat = fresh(ctx, &self.arg_name);
        let a_out = check(ctx, r, &self.arg_type, &values::universe())?;
        let b_out = check(
            &ctx.bind_free(x_hat.clone(), val_in_ctx(ctx, &a_out))?,
            &r.extend(self.arg_name.clone(), x_hat.clone()),
            &self.res_type,
            &values::universe(),
        )?;
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
        let arg = resugar::resugar_(&self.arg_type);
        let res = resugar::resugar_(&self.res_type);
        if res.0.contains(&self.arg_name) {
            todo!()
        } else {
            (&arg.0 | &res.0, resugar::add_fun(arg.1, res.1))
        }
    }
}

impl Display for Pi<Core, Core> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Π (({} {})) {})",
            self.arg_name.name(),
            self.arg_type,
            self.res_type
        )
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
        let ae = read_back_type(ctx, &self.arg_type);
        let x_hat = fresh(ctx, &self.arg_name);

        let ctx_hat = ctx.bind_free(x_hat.clone(), self.arg_type.clone()).unwrap();
        let r = read_back_type(
            &ctx_hat,
            &self.res_type.val_of(neutral::neutral(
                self.arg_type.clone(),
                N::Var(x_hat.clone()),
            )),
        );
        Ok(Core::pi(x_hat, ae, r))
    }

    fn read_back(&self, ctx: &Ctx, _tv: &Value, f: &Value) -> errors::Result<Core> {
        let y = match f.as_any().downcast_ref::<Lambda<Closure>>() {
            Some(lam) => &lam.arg_name,
            None => &self.arg_name,
        };

        let x_hat = fresh(ctx, y);
        Ok(Core::lambda(
            x_hat.clone(),
            read_back(
                &ctx.bind_free(x_hat.clone(), self.arg_type.clone()).unwrap(),
                &self.res_type.val_of(neutral::neutral(
                    self.arg_type.clone(),
                    N::Var(x_hat.clone()),
                )),
                &functions::do_ap(f, neutral::neutral(self.arg_type.clone(), N::Var(x_hat))),
            ),
        ))
    }

    fn apply(
        &self,
        _ctx: &Ctx,
        _r: &Renaming,
        rator_out: &Core,
        _rand: &Core,
    ) -> errors::Result<(Core, Core)> {
        let rand_out = check(_ctx, _r, _rand, &self.arg_type)?;
        Ok((
            read_back_type(_ctx, &self.res_type.val_of(val_in_ctx(_ctx, &rand_out))),
            Core::app((*rator_out).clone(), rand_out),
        ))
    }
}
