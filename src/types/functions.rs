use crate::alpha;
use crate::alpha::alpha_equiv_aux;
use crate::basics::{
    fresh, occurring_binder_names, occurring_names, Closure, Core, CoreInterface, Ctx, Env,
    Renaming, Value, ValueInterface, N,
};
use crate::errors::{Error, Result};
use crate::normalize::{now, read_back, read_back_type, val_in_ctx};
use crate::resugar;
use crate::symbol::Symbol;
use crate::typechecker::{check, is_type, same_type};
use crate::types::neutral::Neutral;
use crate::types::values::{lambda, later, neutral};
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Pi<T, C> {
    pub arg_name: Symbol,
    pub arg_type: T,
    pub res_type: C,
}

/// An actual Function
#[derive(Debug, Clone)]
pub struct Lambda<B> {
    pub arg_name: Symbol,
    pub body: B,
}

/// A function application
#[derive(Debug, Clone)]
pub struct App {
    pub fun: Core,
    pub arg: Core,
}

impl CoreInterface for Pi<Core, Core> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn CoreInterface) -> bool {
        unimplemented!()
    }

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

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
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

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
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

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        let (t_out, e_out) = self.synth(ctx, r)?;
        same_type(ctx, &val_in_ctx(ctx, &t_out), tv)?;
        Ok(e_out)
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

impl CoreInterface for Lambda<Core> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn CoreInterface) -> bool {
        unimplemented!()
    }

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

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        Err(Error::NotAType(Core::new(self.clone())))
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Err(Error::CantDetermineType(Core::new(self.clone())))
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
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

impl CoreInterface for App {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn CoreInterface) -> bool {
        unimplemented!()
    }

    fn occurring_names(&self) -> HashSet<Symbol> {
        &occurring_names(&self.fun) | &occurring_names(&self.arg)
    }

    fn val_of(&self, env: &Env) -> Value {
        do_ap(
            &later(env.clone(), self.fun.clone()),
            later(env.clone(), self.arg.clone()),
        )
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        match self.check(ctx, r, &values::universe()) {
            Ok(t_out) => Ok(t_out),
            Err(_) => Err(Error::NotAType(Core::new(self.clone()))),
        }
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        panic!("use AppStar for synthesis")
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        let (t_out, e_out) = self.synth(ctx, r)?;
        same_type(ctx, &val_in_ctx(ctx, &t_out), tv)?;
        Ok(e_out)
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            alpha_equiv_aux(lvl, b1, b2, &self.fun, &other.fun)
                && alpha_equiv_aux(lvl, b1, b2, &self.arg, &other.arg)
        } else {
            false
        }
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

impl ValueInterface for Pi<Value, Closure> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn ValueInterface) -> bool {
        unimplemented!()
    }

    fn read_back_type(&self, ctx: &Ctx) -> Result<Core> {
        let ae = read_back_type(ctx, &self.arg_type);
        let x_hat = fresh(ctx, &self.arg_name);

        let ctx_hat = ctx.bind_free(x_hat.clone(), self.arg_type.clone()).unwrap();
        let r = read_back_type(
            &ctx_hat,
            &self.res_type.val_of(values::neutral(
                self.arg_type.clone(),
                N::Var(x_hat.clone()),
            )),
        );
        Ok(Core::pi(x_hat, ae, r))
    }

    fn read_back(&self, ctx: &Ctx, _tv: &Value, f: &Value) -> Result<Core> {
        let y = match f.as_any().downcast_ref::<Lambda<Closure>>() {
            Some(lam) => &lam.arg_name,
            None => &self.arg_name,
        };

        let x_hat = fresh(ctx, y);
        Ok(Core::lambda(
            x_hat.clone(),
            read_back(
                &ctx.bind_free(x_hat.clone(), self.arg_type.clone()).unwrap(),
                &self.res_type.val_of(values::neutral(
                    self.arg_type.clone(),
                    N::Var(x_hat.clone()),
                )),
                &do_ap(f, values::neutral(self.arg_type.clone(), N::Var(x_hat))),
            ),
        ))
    }

    fn raw_apply(
        &self,
        _ctx: &Ctx,
        _r: &Renaming,
        rator_out: &Core,
        _rand: &Core,
    ) -> Result<(Core, Core)> {
        let rand_out = check(_ctx, _r, _rand, &self.arg_type)?;
        Ok((
            read_back_type(_ctx, &self.res_type.val_of(val_in_ctx(_ctx, &rand_out))),
            Core::app((*rator_out).clone(), rand_out),
        ))
    }
}

impl ValueInterface for Lambda<Closure> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn ValueInterface) -> bool {
        unimplemented!()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Err(Error::NotATypeVar(lambda(
            self.arg_name.clone(),
            self.body.clone(),
        )))
    }
}

pub fn do_ap(rator: &Value, rand: Value) -> Value {
    match now(rator).as_any().downcast_ref::<Lambda<Closure>>() {
        Some(Lambda { body, .. }) => return body.val_of(rand),
        None => {}
    }

    match now(rator).as_any().downcast_ref::<Neutral>() {
        Some(neu) => {
            if let Some(pi) = now(&neu.type_value)
                .as_any()
                .downcast_ref::<Pi<Value, Closure>>()
            {
                neutral(
                    pi.res_type.val_of(rand.clone()),
                    N::app(neu.kind.clone(), pi.arg_type.clone(), rand),
                )
            } else {
                todo!()
            }
        }
        None => todo!("{:?}", now(rator)),
    }
}
