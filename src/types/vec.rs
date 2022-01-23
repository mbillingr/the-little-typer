use crate::basics::{
    Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, The, Value, ValueInterface, N,
};
use crate::errors::{Error, Result};
use crate::normalize::{read_back, val_in_ctx};
use crate::symbol::Symbol;
use crate::typechecker::convert;
use crate::types::functions::do_ap;
use crate::types::natural::{Add1, Nat, Zero};
use crate::types::values::later;
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;

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

#[derive(Debug, Clone, PartialEq)]
pub struct IndVec {
    pub len: Core,
    pub target: Core,
    pub motive: Core,
    pub base: Core,
    pub step: Core,
}

#[derive(Debug)]
pub struct NeutralHead(pub N);

#[derive(Debug)]
pub struct NeutralTail(pub N);

#[derive(Debug)]
pub struct NeutralIndVec1(pub N, pub The, pub The, pub The, pub The);

#[derive(Debug)]
pub struct NeutralIndVec2(pub The, pub N, pub The, pub The, pub The);

#[derive(Debug)]
pub struct NeutralIndVec12(pub N, pub N, pub The, pub The, pub The);

impl CoreInterface for Vector<Core> {
    impl_core_defaults!(
        (0, 1),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        check_by_synth,
        (resugar: vec)
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
}

impl CoreInterface for VecNil {
    impl_core_defaults!(
        _,
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        no_type,
        (resugar: vecnil)
    );

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
}

impl CoreInterface for VectorCons<Core> {
    impl_core_defaults!(
        (0, 1),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        no_type,
        (resugar: vec_cons)
    );

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
        do_tail(&later(env.clone(), self.0.clone()))
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

impl CoreInterface for IndVec {
    impl_core_defaults!(
        (len, target, motive, base, step),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        no_type,
        check_by_synth,
        (resugar: ind_vec)
    );

    fn val_of(&self, env: &Env) -> Value {
        do_ind_vec(
            later(env.clone(), self.len.clone()),
            later(env.clone(), self.target.clone()),
            later(env.clone(), self.motive.clone()),
            later(env.clone(), self.base.clone()),
            later(env.clone(), self.step.clone()),
        )
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let nat = values::nat();
        let len_out = self.len.check(ctx, r, &nat)?;
        let len_v = val_in_ctx(ctx, &len_out);
        let (vec_t, vec_out) = self.target.synth(ctx, r)?;
        let vec_tv = val_in_ctx(ctx, &vec_t);
        if let Some(Vector(ev, len2_v)) = vec_tv.try_as::<Vector<Value>>() {
            convert(ctx, &nat, &len_v, len2_v)?;
            let mot_out = {
                let ev = ev.clone();
                self.motive.check(
                    ctx,
                    r,
                    &pi_type!(
                        ((k, nat)),
                        pi_type!(((_es as "es", values::vec(ev.clone(), k))), values::universe())
                    ),
                )?
            };
            let mot_val = val_in_ctx(ctx, &mot_out);
            let b_out = self.base.check(
                ctx,
                r,
                &do_ap(&do_ap(&mot_val, values::zero()), values::vecnil()),
            )?;
            let s_out = self
                .step
                .check(ctx, r, &ind_vec_step_type(ev.clone(), mot_val))?;
            Ok((
                cores::app(
                    cores::app(mot_out.clone(), len_out.clone()),
                    vec_out.clone(),
                ),
                cores::ind_vec(len_out, vec_out, mot_out, b_out, s_out),
            ))
        } else {
            Err(Error::NotAVecType(vec_tv.read_back_type(ctx)?))
        }
    }
}

fn ind_vec_step_type(ev: Value, mot_v: Value) -> Value {
    pi_type!(((k, values::nat())), {
        let mot_v = mot_v.clone();
        let ev = ev.clone();
        pi_type!(((e, ev.clone())), {
            let mot_v = mot_v.clone();
            let k = k.clone();
            pi_type!(((es, values::vec(ev.clone(), k.clone()))), {
                let mot_v = mot_v.clone();
                let k = k.clone();
                let e = e.clone();
                pi_type!(((_ih as "ih", do_ap(&do_ap(&mot_v, k.clone()), es.clone()))), {
                    do_ap(
                        &do_ap(&mot_v, values::add1(k.clone())),
                        values::vec_cons(e.clone(), es.clone()),
                    )
                })
            })
        })
    })
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

impl_sexpr_display!(T: Vector<T>, ("Vec", 0, 1));
impl_sexpr_display!(VecNil, "vecnil");
impl_sexpr_display!(T: VectorCons<T>, ("vec::", 0, 1));
impl_sexpr_display!(Head, ("head", 0));
impl_sexpr_display!(Tail, ("tail", 0));
impl_sexpr_display!(IndVec, ("ind-Vec", len, target, motive, base, step));

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

fn do_head(tgt_v: &Value) -> Value {
    if let Some(VectorCons(hv, _)) = tgt_v.try_as::<VectorCons<Value>>() {
        return hv.clone();
    }

    match tgt_v.as_neutral() {
        Some((vec, ne)) => match vec.try_as::<Vector<Value>>() {
            Some(Vector(ev, lenv)) => match lenv.try_as::<Add1<Value>>() {
                Some(Add1(_)) => return values::neutral(ev.clone(), NeutralHead(ne.clone())),
                None => {}
            },
            None => {}
        },
        None => {}
    }

    unreachable!()
}

fn do_tail(tgt_v: &Value) -> Value {
    if let Some(VectorCons(_, tv)) = tgt_v.try_as::<VectorCons<Value>>() {
        return tv.clone();
    }

    match tgt_v.as_neutral() {
        Some((vec, ne)) => match vec.try_as::<Vector<Value>>() {
            Some(Vector(ev, lenv)) => match lenv.try_as::<Add1<Value>>() {
                Some(Add1(len_minus_1v)) => {
                    return values::neutral(
                        values::vec(ev.clone(), len_minus_1v.clone()),
                        NeutralTail(ne.clone()),
                    )
                }
                None => {}
            },
            None => {}
        },
        None => {}
    }

    unreachable!()
}

impl NeutralInterface for NeutralHead {
    fn read_back_neutral(&self, ctx: &Ctx) -> Result<Core> {
        Ok(cores::head(self.0.read_back_neutral(ctx)?))
    }
}

impl NeutralInterface for NeutralTail {
    fn read_back_neutral(&self, ctx: &Ctx) -> Result<Core> {
        Ok(cores::tail(self.0.read_back_neutral(ctx)?))
    }
}

fn do_ind_vec(len_v: Value, vec_v: Value, mot_v: Value, b_v: Value, s_v: Value) -> Value {
    _do_ind_vec(&len_v, &vec_v, mot_v, b_v, &s_v)
}

fn _do_ind_vec(len_v: &Value, vec_v: &Value, mot_v: Value, b_v: Value, s_v: &Value) -> Value {
    if let (Some(_), Some(_)) = (len_v.try_as::<Zero>(), vec_v.try_as::<VecNil>()) {
        return b_v;
    }

    if let (Some(Add1(len_m1_v)), Some(VectorCons(h, t))) = (
        len_v.try_as::<Add1<Value>>(),
        vec_v.try_as::<VectorCons<Value>>(),
    ) {
        return do_ap(
            &do_ap(
                &do_ap(&do_ap(s_v, len_m1_v.clone()), h.clone()),
                do_tail(vec_v),
            ),
            _do_ind_vec(len_m1_v, t, mot_v, b_v, s_v),
        );
    }

    if let Some((vec_tv, ne)) = vec_v.as_neutral() {
        if let Some(Vector(etv, _)) = vec_tv.try_as::<Vector<Value>>() {
            let t_out = do_ap(&do_ap(&mot_v, len_v.clone()), vec_v.clone());
            let m_out = The(
                {
                    let etv = etv.clone();
                    pi_type!(((k, values::nat())), {
                        let etv = etv.clone();
                        pi_type!(((_es as "es", values::vec(etv, k))), values::universe())
                    })
                },
                mot_v.clone(),
            );
            let b_out = The(do_ap(&do_ap(&mot_v, values::zero()), values::vecnil()), b_v);
            let s_out = The(ind_vec_step_type(etv.clone(), mot_v), s_v.clone());

            if let Some((len_tv, len)) = len_v.as_neutral() {
                if len_tv.try_as::<Nat>().is_some() {
                    return values::neutral(
                        t_out,
                        NeutralIndVec12(len.clone(), ne.clone(), m_out, b_out, s_out),
                    );
                }
            }

            return values::neutral(
                t_out,
                NeutralIndVec2(
                    The(values::nat(), len_v.clone()),
                    ne.clone(),
                    m_out,
                    b_out,
                    s_out,
                ),
            );
        }
    }

    unreachable!()
}

impl NeutralInterface for NeutralIndVec1 {
    fn read_back_neutral(&self, _ctx: &Ctx) -> Result<Core> {
        todo!()
    }
}

impl NeutralInterface for NeutralIndVec2 {
    fn read_back_neutral(&self, _ctx: &Ctx) -> Result<Core> {
        todo!()
    }
}

impl NeutralInterface for NeutralIndVec12 {
    fn read_back_neutral(&self, ctx: &Ctx) -> Result<Core> {
        let NeutralIndVec12(len, es, The(mot_t, mot), The(b_t, b), The(s_t, s)) = self;
        Ok(cores::ind_vec(
            len.read_back_neutral(ctx)?,
            es.read_back_neutral(ctx)?,
            read_back(ctx, mot_t, mot)?,
            read_back(ctx, b_t, b)?,
            read_back(ctx, s_t, s)?,
        ))
    }
}
