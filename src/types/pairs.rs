use crate::alpha;
use crate::basics::{Closure, Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::normalize::{now, read_back, val_in_ctx};
use crate::symbol::Symbol;
use crate::types::reference::NeutralVar;
use crate::types::values::later;
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Sigma<T, C> {
    pub arg_name: Symbol,
    pub car_type: T,
    pub cdr_type: C,
}

/// The type of pairs
#[derive(Debug, Clone, PartialEq)]
pub struct Pair<T>(pub T, pub T);

/// pairs
#[derive(Debug, Clone, PartialEq)]
pub struct Cons<T>(pub T, pub T);

impl CoreInterface for Sigma<Core, Core> {
    impl_core_defaults!((arg_name, car_type, cdr_type), as_any, same, check_by_synth);

    fn occurring_names(&self) -> HashSet<Symbol> {
        todo!()
    }

    fn val_of(&self, env: &Env) -> Value {
        let av = later(env.clone(), self.car_type.clone());
        values::sigma(
            self.arg_name.clone(),
            av,
            Closure::FirstOrder {
                env: env.clone(),
                var: self.arg_name.clone(),
                expr: self.cdr_type.clone(),
            },
        )
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        todo!()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        todo!()
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        _lvl: usize,
        _b1: &alpha::Bindings,
        _b2: &alpha::Bindings,
    ) -> bool {
        if let Some(_other) = other.as_any().downcast_ref::<Self>() {
            todo!()
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let a_t = self.car_type.resugar();
        let d_t = self.cdr_type.resugar();
        if d_t.0.contains(&self.arg_name) {
            todo!()
        } else {
            (&a_t.0 | &d_t.0, cores::pair(a_t.1, d_t.1))
        }
    }
}

impl CoreInterface for Pair<Core> {
    impl_core_defaults!(
        (0, 1),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        check_by_synth
    );

    fn val_of(&self, _env: &Env) -> Value {
        todo!()
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        let x = ctx.fresh_binder(&self.1, &Symbol::new("x"));
        let a_out = self.0.is_type(ctx, r)?;
        let d_out = self
            .1
            .is_type(&ctx.bind_free(x.clone(), val_in_ctx(ctx, &a_out))?, r)?;
        Ok(cores::sigma(x, a_out, d_out))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let a = ctx.fresh(&Symbol::new("a"));
        let a_out = self.0.check(ctx, r, &values::universe())?;
        let d_out = self.1.check(
            &ctx.bind_free(a.clone(), val_in_ctx(ctx, &a_out))?,
            r,
            &values::universe(),
        )?;
        Ok((cores::universe(), cores::sigma(a, a_out, d_out)))
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        unimplemented!()
    }
}

impl CoreInterface for Cons<Core> {
    impl_core_defaults!((0, 1), as_any, same, occurring_names, alpha_equiv, no_type);

    fn val_of(&self, env: &Env) -> Value {
        values::cons(
            later(env.clone(), self.0.clone()),
            later(env.clone(), self.1.clone()),
        )
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Err(Error::CantDetermineType(Core::new(self.clone())))
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        if let Some(sigma) = now(tv).as_any().downcast_ref::<Sigma<Value, Closure>>() {
            let a_out = self.0.check(ctx, r, &sigma.car_type)?;
            let d_out = self
                .1
                .check(ctx, r, &sigma.cdr_type.val_of(val_in_ctx(ctx, &a_out)))?;
            Ok(cores::cons(a_out, d_out))
        } else {
            Err(Error::NotASigmaType(tv.read_back_type(ctx).unwrap()))
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let a = self.0.resugar();
        let d = self.1.resugar();
        (&a.0 | &d.0, cores::cons(a.1, d.1))
    }
}

impl ValueInterface for Sigma<Value, Closure> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn ValueInterface) -> bool {
        unimplemented!()
    }

    fn read_back_type(&self, ctx: &Ctx) -> Result<Core> {
        let a_e = self.car_type.read_back_type(ctx)?;
        let x_hat = ctx.fresh(&self.arg_name);
        let ctx_hat = ctx.bind_free(x_hat.clone(), self.car_type.clone())?;
        Ok(cores::sigma(
            x_hat.clone(),
            a_e,
            self.cdr_type
                .val_of(values::neutral(self.car_type.clone(), NeutralVar(x_hat)))
                .read_back_type(&ctx_hat)?,
        ))
    }

    fn read_back(&self, ctx: &Ctx, _tv: &Value, pv: &Value) -> Result<Core> {
        let the_car = do_car(pv);
        Ok(cores::cons(
            read_back(ctx, &self.car_type, &the_car)?,
            read_back(ctx, &self.cdr_type.val_of(the_car), &do_cdr(pv))?,
        ))
    }

    fn apply(
        &self,
        _ctx: &Ctx,
        _r: &Renaming,
        _rator_out: &Core,
        _rand: &Core,
    ) -> Result<(Core, Core)> {
        todo!()
    }
}

impl ValueInterface for Cons<Value> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Err(Error::NotATypeVar(values::cons(
            self.0.clone(),
            self.1.clone(),
        )))
    }
}

impl Display for Sigma<Core, Core> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Σ (({} {})) {})",
            self.arg_name.name(),
            self.car_type,
            self.cdr_type
        )
    }
}

impl Display for Pair<Core> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Pair {} {})", self.0, self.1)
    }
}

impl Display for Cons<Core> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(cons {} {})", self.0, self.1)
    }
}

fn do_car(pv: &Value) -> Value {
    match now(pv).as_any().downcast_ref::<Cons<Value>>() {
        Some(Cons(a, _)) => return a.clone(),
        None => {}
    }

    todo!()
}

fn do_cdr(pv: &Value) -> Value {
    match now(pv).as_any().downcast_ref::<Cons<Value>>() {
        Some(Cons(_, d)) => return d.clone(),
        None => {}
    }

    todo!()
}
