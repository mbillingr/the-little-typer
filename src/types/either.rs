use crate::basics::{
    Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, The, Value, ValueInterface, N,
};
use crate::errors::{Error, Result};
use crate::normalize::{read_back, val_in_ctx};
use crate::symbol::Symbol;
use crate::types::functions::do_ap;
use crate::types::values::later;
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Either<T>(pub T, pub T);

#[derive(Debug, Clone, PartialEq)]
pub struct Left<T>(pub T);

#[derive(Debug, Clone, PartialEq)]
pub struct Right<T>(pub T);

#[derive(Debug, Clone, PartialEq)]
pub struct IndEither {
    target: Core,
    motive: Core,
    left: Core,
    right: Core,
}
#[derive(Debug)]
pub struct NeutralIndEither(pub N, pub The, pub The, pub The);

impl IndEither {
    pub fn new(target: Core, motive: Core, left: Core, right: Core) -> Self {
        IndEither {
            target,
            motive,
            left,
            right,
        }
    }
}

impl CoreInterface for Either<Core> {
    impl_core_defaults!(
        (0, 1),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        check_by_synth,
        (resugar: either)
    );

    fn val_of(&self, env: &Env) -> Value {
        values::either(
            later(env.clone(), self.0.clone()),
            later(env.clone(), self.1.clone()),
        )
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        let l_out = self.0.is_type(ctx, r)?;
        let r_out = self.1.is_type(ctx, r)?;
        Ok(cores::either(l_out, r_out))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let u = values::universe();
        let l_out = self.0.check(ctx, r, &u)?;
        let r_out = self.1.check(ctx, r, &u)?;
        Ok((cores::universe(), cores::either(l_out, r_out)))
    }
}

impl CoreInterface for Left<Core> {
    impl_core_defaults!((0), as_any, same, occurring_names, no_synth, alpha_equiv);

    fn val_of(&self, env: &Env) -> Value {
        values::left(later(env.clone(), self.0.clone()))
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        unimplemented!()
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        if let Some(Either(ltv, _)) = tv.try_as::<Either<Value>>() {
            let l_out = self.0.check(ctx, r, ltv)?;
            Ok(cores::left(l_out))
        } else {
            Err(Error::NotAnEitherType(tv.read_back_type(ctx).unwrap()))
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        unimplemented!()
    }
}

impl CoreInterface for Right<Core> {
    impl_core_defaults!((0), as_any, same, occurring_names, no_synth, alpha_equiv);

    fn val_of(&self, env: &Env) -> Value {
        values::right(later(env.clone(), self.0.clone()))
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        unimplemented!()
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        if let Some(Either(_, rtv)) = tv.try_as::<Either<Value>>() {
            let r_out = self.0.check(ctx, r, rtv)?;
            Ok(cores::right(r_out))
        } else {
            Err(Error::NotAnEitherType(tv.read_back_type(ctx).unwrap()))
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        unimplemented!()
    }
}

impl CoreInterface for IndEither {
    impl_core_defaults!(
        (target, motive, left, right),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        no_type,
        check_by_synth,
        (resugar: ind_either)
    );

    fn val_of(&self, env: &Env) -> Value {
        do_ind_either(
            later(env.clone(), self.target.clone()),
            later(env.clone(), self.motive.clone()),
            later(env.clone(), self.left.clone()),
            later(env.clone(), self.right.clone()),
        )
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let (tgt_t, tgt_out) = self.target.synth(ctx, r)?;
        let tgt_tv = val_in_ctx(ctx, &tgt_t);
        if let Some(Either(ltv, rtv)) = tgt_tv.try_as::<Either<Value>>() {
            let mot_out = self.motive.check(
                ctx,
                r,
                &pi_type!(((_x as "x", tgt_tv.clone())), values::universe()),
            )?;
            let mot_val = val_in_ctx(ctx, &mot_out);
            let l_out = self.left.check(ctx, r, {
                let mot_val = mot_val.clone();
                &pi_type!(((x, ltv.clone())), do_ap(&mot_val, values::left(x)))
            })?;
            let r_out = self.right.check(ctx, r, {
                &pi_type!(((x, rtv.clone())), do_ap(&mot_val, values::right(x)))
            })?;
            Ok((
                cores::app(mot_out.clone(), tgt_out.clone()),
                cores::ind_either(tgt_out, mot_out, l_out, r_out),
            ))
        } else {
            Err(Error::NotAnEitherType(tgt_tv.read_back_type(ctx)?))
        }
    }
}

impl_sexpr_display!(T: Either<T>, ("Either", 0, 1));
impl_sexpr_display!(T: Left<T>, ("left", 0));
impl_sexpr_display!(T: Right<T>, ("right", 0));
impl_sexpr_display!(IndEither, ("ind-Either", target, motive, left, right));

impl ValueInterface for Either<Value> {
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

    fn read_back_type(&self, ctx: &Ctx) -> Result<Core> {
        Ok(cores::either(
            self.0.read_back_type(ctx)?,
            self.1.read_back_type(ctx)?,
        ))
    }

    fn read_back(&self, ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
        if let Some(Left(lv)) = v.try_as::<Left<Value>>() {
            return Ok(cores::left(read_back(ctx, &self.0, lv)?));
        }

        if let Some(Right(rv)) = v.try_as::<Right<Value>>() {
            return Ok(cores::right(read_back(ctx, &self.1, rv)?));
        }

        Err(Error::TypeMismatchVar(v.clone(), tv.clone()))
    }
}

impl ValueInterface for Left<Value> {
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
        unimplemented!()
    }
}

impl ValueInterface for Right<Value> {
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
        unimplemented!()
    }
}

fn do_ind_either(tgt_v: Value, mot_v: Value, l_v: Value, r_v: Value) -> Value {
    _do_ind_either(&tgt_v, &mot_v, l_v, r_v)
}

fn _do_ind_either(tgt: &Value, mot: &Value, l: Value, r: Value) -> Value {
    if let Some(Left(x)) = tgt.try_as::<Left<Value>>() {
        return do_ap(&l, x.clone());
    }

    if let Some(Right(x)) = tgt.try_as::<Right<Value>>() {
        return do_ap(&r, x.clone());
    }

    if let Some((either_tv, ne)) = tgt.as_neutral() {
        if let Some(Either(ltv, rtv)) = either_tv.try_as::<Either<Value>>() {
            let mot_tv = pi_type!(((_x as "xs", either_tv.clone())), values::universe());
            return values::neutral(
                do_ap(mot, tgt.clone()),
                NeutralIndEither(
                    ne.clone(),
                    The(mot_tv, mot.clone()),
                    {
                        let mot = mot.clone();
                        The(
                            pi_type!(((x, ltv.clone())), do_ap(&mot, values::left(x))),
                            l,
                        )
                    },
                    {
                        let mot = mot.clone();
                        The(
                            pi_type!(((x, rtv.clone())), do_ap(&mot, values::right(x))),
                            r,
                        )
                    },
                ),
            );
        }
    }

    unreachable!()
}

impl NeutralInterface for NeutralIndEither {
    fn read_back_neutral(&self, ctx: &Ctx) -> Result<Core> {
        let NeutralIndEither(tgt, The(mot_t, mot), The(l_t, l), The(r_t, r)) = self;
        Ok(cores::ind_list(
            tgt.read_back_neutral(ctx)?,
            read_back(ctx, mot_t, mot)?,
            read_back(ctx, l_t, l)?,
            read_back(ctx, r_t, r)?,
        ))
    }
}
