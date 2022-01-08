use crate::basics::{Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::normalize::{read_back, val_in_ctx};
use crate::symbol::Symbol;
use crate::types::natural::{Add1, Zero};
use crate::types::values::later;
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

/// The type of lists with length
#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T>(pub T, pub T);

/// The empty vector
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VecNil;

/// The vector constructor
#[derive(Debug, Clone, PartialEq)]
pub struct VectorCons<T>(pub T, pub T);

#[derive(Debug, Clone, PartialEq)]
pub struct Head(pub Core);

#[derive(Debug, Clone, PartialEq)]
pub struct Tail(pub Core);

//ternary_eliminator!(RecVec, do_rec_list, synth_rec_list);

impl CoreInterface for Vector<Core> {
    impl_core_defaults!(
        (0, 1),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        check_by_synth
    );

    fn val_of(&self, env: &Env) -> Value {
        values::vec(
            later(env.clone(), self.0.clone()),
            later(env.clone(), self.1.clone()),
        )
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        let e_out = self.0.is_type(ctx, r)?;
        let len_out = self.1.check(ctx, r, &values::nat())?;
        Ok(cores::vec(e_out, len_out))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let e_out = self.0.check(ctx, r, &values::universe())?;
        let len_out = self.1.check(ctx, r, &values::nat())?;
        Ok((cores::universe(), cores::vec(e_out, len_out)))
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let t = self.0.resugar();
        let n = self.1.resugar();
        (&t.0 | &n.0, cores::vec(t.1, n.1))
    }
}

impl CoreInterface for VecNil {
    impl_core_defaults!(_, as_any, same, occurring_names, alpha_equiv, no_type);

    fn val_of(&self, _env: &Env) -> Value {
        values::vecnil()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Err(Error::CantDetermineType(Core::new(self.clone())))
    }

    fn check(&self, ctx: &Ctx, _r: &Renaming, tv: &Value) -> Result<Core> {
        if let Some(Vector(_, n)) = tv.try_as::<Vector<Value>>() {
            if n.try_as::<Zero>().is_some() {
                Ok(cores::vecnil())
            } else {
                Err(Error::LengthNotZero(values::nat().read_back(ctx, n)?))
            }
        } else {
            Err(Error::NotAVecType(tv.read_back_type(ctx).unwrap()))
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        (HashSet::new(), cores::vecnil())
    }
}

impl CoreInterface for VectorCons<Core> {
    impl_core_defaults!((0, 1), as_any, same, occurring_names, alpha_equiv, no_type);

    fn val_of(&self, env: &Env) -> Value {
        values::vec_cons(
            later(env.clone(), self.0.clone()),
            later(env.clone(), self.1.clone()),
        )
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Err(Error::CantDetermineType(Core::new(self.clone())))
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        let (etv, len_minus_one) = expect_non_empty_vec(ctx, tv)?;
        let h_out = self.0.check(ctx, r, etv)?;
        let t_out = self
            .1
            .check(ctx, r, &values::vec(etv.clone(), len_minus_one.clone()))?;
        Ok(cores::vec_cons(h_out, t_out))
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let h = self.0.resugar();
        let t = self.1.resugar();
        (&h.0 | &t.0, cores::vec_cons(h.1, t.1))
    }
}

impl CoreInterface for Head {
    impl_core_defaults!(
        (0),
        as_any,
        same,
        occurring_names,
        check_by_synth,
        alpha_equiv
    );

    fn val_of(&self, env: &Env) -> Value {
        do_head(&later(env.clone(), self.0.clone())).clone()
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        unimplemented!()
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let (es_type_out, es_out) = self.0.synth(ctx, r)?;
        let es_type_out_val = val_in_ctx(ctx, &es_type_out);
        let (etv, _) = expect_non_empty_vec(ctx, &es_type_out_val)?;
        Ok((etv.read_back_type(ctx)?, cores::head(es_out)))
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        unimplemented!()
    }
}

impl CoreInterface for Tail {
    impl_core_defaults!(
        (0),
        as_any,
        same,
        occurring_names,
        check_by_synth,
        alpha_equiv
    );

    fn val_of(&self, env: &Env) -> Value {
        do_tail(&later(env.clone(), self.0.clone())).clone()
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        unimplemented!()
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let (es_type_out, es_out) = self.0.synth(ctx, r)?;
        let es_type_out_val = val_in_ctx(ctx, &es_type_out);
        let (etv, len_minus_1) = expect_non_empty_vec(ctx, &es_type_out_val)?;
        Ok((
            cores::vec(
                etv.read_back_type(ctx)?,
                read_back(ctx, &values::nat(), len_minus_1)?,
            ),
            cores::tail(es_out),
        ))
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        unimplemented!()
    }
}

fn expect_non_empty_vec<'a>(ctx: &Ctx, tv: &'a Value) -> Result<(&'a Value, &'a Value)> {
    if let Some(Vector(etv, len)) = tv.try_as::<Vector<Value>>() {
        if let Some(Add1(len_minus_one)) = len.try_as::<Add1<Value>>() {
            Ok((etv, len_minus_one))
        } else {
            Err(Error::LengthZero(values::nat().read_back(ctx, len)?))
        }
    } else {
        Err(Error::NotAVecType(tv.read_back_type(ctx).unwrap()))
    }
}

/*fn synth_rec_vec(this: &RecList, ctx: &Ctx, r: &Renaming, b: &Core) -> Result<(Core, Core)> {
    let (tgt_t, tgt_out) = this.target.synth(ctx, r)?;
    let tgt_tv = val_in_ctx(ctx, &tgt_t);
    if let Some(Vector(e_tv)) = tgt_tv.try_as::<Vector<Value>>() {
        let (b_t_out, b_out) = b.synth(ctx, r)?;
        let b_t_val = val_in_ctx(ctx, &b_t_out);
        let s_out = this.step.check(
            ctx,
            r,
            &pi_type!(((_e, e_tv.clone())), {
                let b_t_val = b_t_val.clone();
                pi_type!(((_es, tgt_tv.clone())), {
                    let b_t_val = b_t_val.clone();
                    pi_type!(((_ih, b_t_val.clone())), b_t_val.clone())
                })
            }),
        )?;
        Ok((
            b_t_out.clone(),
            cores::rec_list_desugared(tgt_out, b_t_out, b_out, s_out),
        ))
    } else {
        Err(Error::NotAListType(tgt_tv.read_back_type(ctx)?))
    }
}*/

impl<T: Display> Display for Vector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Vec {} {})", self.0, self.1)
    }
}

impl std::fmt::Display for VecNil {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "vecnil")
    }
}

impl<T: Display> Display for VectorCons<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(vec:: {} {})", self.0, self.1)
    }
}

impl Display for Head {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(head {})", self.0)
    }
}

impl Display for Tail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(tail {})", self.0)
    }
}

impl ValueInterface for Vector<Value> {
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
        Ok(cores::vec(
            self.0.read_back_type(ctx)?,
            read_back(ctx, &values::nat(), &self.1)?,
        ))
    }

    fn read_back(&self, ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
        if self.1.try_as::<Zero>().is_some() && v.try_as::<VecNil>().is_some() {
            return Ok(cores::vecnil());
        }

        if let Some(Add1(len_minus_one_v)) = self.1.try_as::<Add1<Value>>() {
            if let Some(VectorCons(h, t)) = v.try_as::<VectorCons<Value>>() {
                return Ok(cores::vec_cons(
                    read_back(ctx, &self.0, h)?,
                    read_back(
                        ctx,
                        &values::vec(self.0.clone(), len_minus_one_v.clone()),
                        t,
                    )?,
                ));
            }
        }

        Err(Error::TypeMismatchVar(v.clone(), tv.clone()))
    }
}

impl ValueInterface for VecNil {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Err(Error::NotATypeVar(values::vecnil()))
    }
}

impl ValueInterface for VectorCons<Value> {
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
        Err(Error::NotATypeVar(values::list_cons(
            self.0.clone(),
            self.1.clone(),
        )))
    }
}

fn do_head(tgt_v: &Value) -> &Value {
    if let Some(VectorCons(hv, _)) = tgt_v.try_as::<VectorCons<Value>>() {
        return hv;
    }

    todo!()
}

fn do_tail(tgt_v: &Value) -> &Value {
    if let Some(VectorCons(_, tv)) = tgt_v.try_as::<VectorCons<Value>>() {
        return tv;
    }

    todo!()
}

/*fn do_rec_vec(tgt_v: Value, bt_v: Value, b_v: Value, s_v: Value) -> Value {
    _do_rec_list(&tgt_v, bt_v, b_v, &s_v)
}*/

/*fn _do_rec_list(tgt_v: &Value, bt_v: Value, b_v: Value, s_v: &Value) -> Value {
    match tgt_v.try_as::<VecNil>() {
        Some(_) => return b_v,
        None => {}
    };

    match tgt_v.try_as::<VectorCons<Value>>() {
        Some(VectorCons(h, t)) => {
            return do_ap(
                &do_ap(&do_ap(s_v, h.clone()), t.clone()),
                _do_rec_list(t, bt_v, b_v, s_v),
            )
        }
        None => {}
    };

    todo!()
}*/
