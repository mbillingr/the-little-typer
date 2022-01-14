use crate::alpha;
use crate::alpha::alpha_equiv_aux;
use crate::basics::{
    Closure, Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, Value, ValueInterface, N,
};
use crate::errors::{Error, Result};
use crate::normalize::{read_back, val_in_ctx};
use crate::symbol::Symbol;
use crate::types::reference::NeutralVar;
use crate::types::values::later;
use crate::types::{check_with_fresh_binding, cores, is_type_with_fresh_binding, values};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Sigma<T, C> {
    pub arg_name: Symbol,
    pub car_type: T,
    pub cdr_type: C,
}

#[derive(Debug, PartialEq)]
pub struct SigmaStar {
    pub binders: Vec<(Symbol, Core)>,
    pub cdr_type: Core,
}

/// The type of pairs
#[derive(Debug, Clone, PartialEq)]
pub struct Pair<T>(pub T, pub T);

#[derive(Debug, Clone, PartialEq)]
pub struct Cons<T>(pub T, pub T);

#[derive(Debug, Clone, PartialEq)]
pub struct Car<T>(pub T);

#[derive(Debug, Clone, PartialEq)]
pub struct Cdr<T>(pub T);

#[derive(Debug)]
pub struct NeutralCar(pub N);

#[derive(Debug)]
pub struct NeutralCdr(pub N);

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

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        let (y, a_out, b_out) =
            is_type_with_fresh_binding(ctx, r, &self.arg_name, &self.car_type, &self.cdr_type)?;
        Ok(cores::sigma(y, a_out, b_out))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let (x_hat, a_out, d_out) =
            check_with_fresh_binding(ctx, r, &self.arg_name, &self.car_type, &self.cdr_type)?;
        Ok((cores::universe(), cores::sigma(x_hat, a_out, d_out)))
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool {
        if let Some(other) = other.try_as::<Self>() {
            alpha_equiv_aux(lvl, b1, b2, &self.car_type, &other.car_type)
                && alpha_equiv_aux(
                    1 + lvl,
                    &b1.bind(&self.arg_name, lvl),
                    &b2.bind(&other.arg_name, lvl),
                    &self.cdr_type,
                    &other.cdr_type,
                )
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

impl CoreInterface for SigmaStar {
    impl_core_defaults!((), as_any, same, check_by_synth, no_alpha_equiv);

    fn occurring_names(&self) -> HashSet<Symbol> {
        todo!()
    }

    fn val_of(&self, _env: &Env) -> Value {
        panic!("Attempt to evaluate Sigma* (should have been desugared to `Sigma`s)")
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        match &self.binders[..] {
            [] => unimplemented!(),
            [(x, a)] => Sigma {
                arg_name: x.clone(),
                car_type: a.clone(),
                cdr_type: self.cdr_type.clone(),
            }
            .is_type(ctx, r),
            [(x, a), more @ ..] => {
                let body = SigmaStar {
                    binders: more.to_vec(),
                    cdr_type: self.cdr_type.clone(),
                };

                let (z, a_out, d_out) = check_with_fresh_binding(ctx, r, x, a, &body)?;

                Ok(cores::sigma(z, a_out, d_out))
            }
        }
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        match &self.binders[..] {
            [] => unreachable!(),
            [(x, a)] => Sigma {
                arg_name: x.clone(),
                car_type: a.clone(),
                cdr_type: self.cdr_type.clone(),
            }
            .synth(ctx, r),
            [(x, a), more @ ..] => {
                let body = SigmaStar {
                    binders: more.to_vec(),
                    cdr_type: self.cdr_type.clone(),
                };

                let (x_hat, a_out, d_out) = check_with_fresh_binding(ctx, r, x, a, &body)?;

                Ok((cores::universe(), cores::sigma(x_hat, a_out, d_out)))
            }
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
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
    impl_core_defaults!(
        (0, 1),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        no_type,
        (resugar: cons)
    );

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
        if let Some(sigma) = tv.try_as::<Sigma<Value, Closure>>() {
            let a_out = self.0.check(ctx, r, &sigma.car_type)?;
            let d_out = self
                .1
                .check(ctx, r, &sigma.cdr_type.val_of(val_in_ctx(ctx, &a_out)))?;
            Ok(cores::cons(a_out, d_out))
        } else {
            Err(Error::NotASigmaType(tv.read_back_type(ctx).unwrap()))
        }
    }
}

impl CoreInterface for Car<Core> {
    impl_core_defaults!(
        (0),
        as_any,
        same,
        occurring_names,
        check_by_synth,
        alpha_equiv
    );

    fn val_of(&self, env: &Env) -> Value {
        do_car(&later(env.clone(), self.0.clone()))
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        todo!()
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        match self.0.synth(ctx, r)? {
            (p_t, p_out) => {
                let val = val_in_ctx(ctx, &p_t);
                match val.try_as::<Sigma<Value, Closure>>() {
                    Some(Sigma { car_type: a, .. }) => {
                        Ok((a.read_back_type(ctx)?, cores::car(p_out)))
                    }
                    _ => Err(Error::NotASigmaType(val.read_back_type(ctx).unwrap())),
                }
            }
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl CoreInterface for Cdr<Core> {
    impl_core_defaults!(
        (0),
        as_any,
        same,
        occurring_names,
        check_by_synth,
        alpha_equiv
    );

    fn val_of(&self, env: &Env) -> Value {
        do_cdr(&later(env.clone(), self.0.clone()))
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        todo!()
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        match self.0.synth(ctx, r)? {
            (p_t, p_out) => {
                let val = val_in_ctx(ctx, &p_t);
                match val.try_as::<Sigma<Value, Closure>>() {
                    Some(Sigma { cdr_type: c, .. }) => Ok((
                        c.val_of(do_car(&val_in_ctx(ctx, &p_out)))
                            .read_back_type(ctx)?,
                        cores::cdr(p_out),
                    )),
                    _ => Err(Error::NotASigmaType(val.read_back_type(ctx).unwrap())),
                }
            }
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
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
        if let Some(other) = other.try_as::<Self>() {
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

impl Display for SigmaStar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let b: Vec<_> = self
            .binders
            .iter()
            .map(|(x, t)| format!("({} {})", x.name(), t))
            .collect();
        write!(f, "(Σ ({}) {})", b.join(" "), self.cdr_type)
    }
}

impl_sexpr_display!(T: Pair<T>, ("Pair", 0, 1));
impl_sexpr_display!(T: Cons<T>, ("cons", 0, 1));
impl_sexpr_display!(T: Car<T>, ("car", 0));
impl_sexpr_display!(T: Cdr<T>, ("cdr", 0));

fn do_car(pv: &Value) -> Value {
    match pv.try_as::<Cons<Value>>() {
        Some(Cons(a, _)) => return a.clone(),
        None => {}
    }

    match pv.as_neutral() {
        Some((p, ne)) => match p.try_as::<Sigma<Value, Closure>>() {
            Some(s) => return values::neutral(s.car_type.clone(), NeutralCar(ne.clone())),
            None => {}
        },
        None => {}
    }

    unreachable!("{:?}", pv)
}

fn do_cdr(pv: &Value) -> Value {
    match pv.try_as::<Cons<Value>>() {
        Some(Cons(_, d)) => return d.clone(),
        None => {}
    }

    match pv.as_neutral() {
        Some((p, ne)) => match p.try_as::<Sigma<Value, Closure>>() {
            Some(s) => {
                return values::neutral(s.cdr_type.val_of(do_car(pv)), NeutralCdr(ne.clone()))
            }
            None => {}
        },
        None => {}
    }

    unreachable!("{:?}", pv)
}

impl NeutralInterface for NeutralCar {
    fn read_back_neutral(&self, ctx: &Ctx) -> Result<Core> {
        Ok(cores::car(self.0.read_back_neutral(ctx)?))
    }
}

impl NeutralInterface for NeutralCdr {
    fn read_back_neutral(&self, ctx: &Ctx) -> Result<Core> {
        Ok(cores::cdr(self.0.read_back_neutral(ctx)?))
    }
}
