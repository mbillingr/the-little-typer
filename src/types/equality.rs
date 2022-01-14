use crate::basics::{Closure, Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::normalize::{read_back, val_in_ctx};
use crate::symbol::Symbol;
use crate::typechecker::{convert, same_type};
use crate::types::functions::{do_ap, Pi};
use crate::types::values::later;
use crate::types::{cores, values};
use std::any::Any;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Equal<T> {
    pub typ: T,
    pub from: T,
    pub to: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Same<T>(pub T);

#[derive(Debug, Clone, PartialEq)]
pub struct Replace {
    pub target: Core,
    pub motive: Core,
    pub base: Core,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cong(pub Core, pub Core);

#[derive(Debug, Clone, PartialEq)]
pub struct Cong2(pub Core, pub Core, pub Core);

#[derive(Debug, Clone, PartialEq)]
pub struct Symm(pub Core);

impl CoreInterface for Equal<Core> {
    impl_core_defaults!(
        (typ, from, to),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        check_by_synth
    );

    fn val_of(&self, env: &Env) -> Value {
        values::equal(
            later(env.clone(), self.typ.clone()),
            later(env.clone(), self.from.clone()),
            later(env.clone(), self.to.clone()),
        )
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        let a_out = self.typ.is_type(ctx, r)?;
        let av = val_in_ctx(ctx, &a_out);
        let from_out = self.from.check(ctx, r, &av)?;
        let to_out = self.to.check(ctx, r, &av)?;
        Ok(cores::equal(a_out, from_out, to_out))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let a_out = self.typ.check(ctx, r, &values::universe())?;
        let av = val_in_ctx(ctx, &a_out);
        let from_out = self.from.check(ctx, r, &av)?;
        let to_out = self.to.check(ctx, r, &av)?;
        Ok((cores::universe(), cores::equal(a_out, from_out, to_out)))
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let t = self.typ.resugar();
        let a = self.from.resugar();
        let b = self.to.resugar();
        (&t.0 | &(&a.0 | &b.0), cores::equal(t.1, a.1, b.1))
    }
}

impl CoreInterface for Same<Core> {
    impl_core_defaults!((0), as_any, same, occurring_names, alpha_equiv, no_type);

    fn val_of(&self, env: &Env) -> Value {
        values::same(later(env.clone(), self.0.clone()))
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        todo!()
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> Result<Core> {
        if let Some(Equal {
            typ: av,
            from: fromv,
            to: tov,
        }) = tv.try_as::<Equal<Value>>()
        {
            let c_out = self.0.check(ctx, r, av)?;
            let v = val_in_ctx(ctx, &c_out);
            convert(ctx, av, fromv, &v)?;
            convert(ctx, av, tov, &v)?;
            Ok(cores::same(c_out))
        } else {
            Err(Error::NotAnEqualType(tv.read_back_type(ctx)?))
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl CoreInterface for Replace {
    impl_core_defaults!(
        (target, motive, base),
        as_any,
        same,
        occurring_names,
        no_type,
        check_by_synth,
        alpha_equiv
    );

    fn val_of(&self, env: &Env) -> Value {
        do_replace(
            later(env.clone(), self.target.clone()),
            later(env.clone(), self.motive.clone()),
            later(env.clone(), self.base.clone()),
        )
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let Replace {
            target: tgt,
            motive: mot,
            base: b,
        } = self;
        let (tgt_t_out, tgt_out) = tgt.synth(ctx, r)?;
        let tgt_t_outv = val_in_ctx(ctx, &tgt_t_out);
        if let Some(Equal {
            typ: av,
            from: fromv,
            to: tov,
        }) = tgt_t_outv.try_as::<Equal<Value>>()
        {
            let mot_out = mot.check(ctx, r, &pi_type!(((_x, av.clone())), values::universe()))?;
            let b_out = b.check(ctx, r, &do_ap(&val_in_ctx(ctx, &mot_out), fromv.clone()))?;
            Ok((
                do_ap(&val_in_ctx(ctx, &mot_out), tov.clone()).read_back_type(ctx)?,
                cores::replace(tgt_out, mot_out, b_out),
            ))
        } else {
            Err(Error::NotAnEqualType(tgt_t_out))
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl CoreInterface for Cong {
    impl_core_defaults!(
        (0, 1),
        as_any,
        same,
        occurring_names,
        check_by_synth,
        alpha_equiv
    );

    fn val_of(&self, _env: &Env) -> Value {
        todo!()
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        todo!()
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let (p_t_out, p_out) = self.0.synth(ctx, r)?;
        let (f_t_out, f_out) = self.1.synth(ctx, r)?;
        let p_t_outv = val_in_ctx(ctx, &p_t_out);
        let f_t_outv = val_in_ctx(ctx, &f_t_out);
        if let Some(Equal {
            typ: av,
            from: from_v,
            to: to_v,
        }) = p_t_outv.try_as::<Equal<Value>>()
        {
            if let Some(Pi {
                arg_name: _x,
                arg_type: bv,
                res_type: c,
            }) = f_t_outv.try_as::<Pi<Value, Closure>>()
            {
                same_type(ctx, av, bv)?;
                let cv = c.val_of(from_v.clone());
                let f_v = val_in_ctx(ctx, &f_out);
                Ok((
                    cores::equal(
                        cv.read_back_type(ctx)?,
                        read_back(ctx, &cv, &do_ap(&f_v, from_v.clone()))?,
                        read_back(ctx, &cv, &do_ap(&f_v, to_v.clone()))?,
                    ),
                    cores::cong_desugared(p_out, cv.read_back_type(ctx)?, f_out),
                ))
            } else {
                Err(Error::NotAFunctionType(f_t_outv.read_back_type(ctx)?))
            }
        } else {
            Err(Error::NotAnEqualType(p_t_outv.read_back_type(ctx)?))
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl CoreInterface for Cong2 {
    impl_core_defaults!(
        (0, 1),
        as_any,
        same,
        occurring_names,
        check_by_synth,
        alpha_equiv
    );

    fn val_of(&self, env: &Env) -> Value {
        do_cong(
            later(env.clone(), self.0.clone()),
            later(env.clone(), self.1.clone()),
            later(env.clone(), self.2.clone()),
        )
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        todo!()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        unreachable!()
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl CoreInterface for Symm {
    impl_core_defaults!(
        (0),
        as_any,
        same,
        occurring_names,
        check_by_synth,
        alpha_equiv
    );

    fn val_of(&self, env: &Env) -> Value {
        todo!()
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> Result<Core> {
        todo!()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        todo!()
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl_sexpr_display!(T: Equal<T>, ("=", typ, from, to));
impl_sexpr_display!(T: Same<T>, ("same", 0));
impl_sexpr_display!(Replace, ("replace", target, motive, base));
impl_sexpr_display!(Cong, ("cong", 0, 1));
impl_sexpr_display!(Cong2, ("cong2", 0, 1, 2));
impl_sexpr_display!(Symm, ("symm", 0));

impl ValueInterface for Equal<Value> {
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
        Ok(cores::equal(
            self.typ.read_back_type(ctx)?,
            read_back(ctx, &self.typ, &self.from)?,
            read_back(ctx, &self.typ, &self.to)?,
        ))
    }

    fn read_back(&self, ctx: &Ctx, _tv: &Value, pv: &Value) -> Result<Core> {
        if let Some(Same(v)) = pv.try_as::<Same<Value>>() {
            Ok(cores::same(read_back(ctx, &self.typ, v)?))
        } else {
            unimplemented!()
        }
    }
}

impl ValueInterface for Same<Value> {
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
        todo!()
    }

    fn read_back(&self, _ctx: &Ctx, _tv: &Value, _pv: &Value) -> Result<Core> {
        todo!()
    }
}

fn do_replace(tgt_v: Value, _mot_v: Value, b_v: Value) -> Value {
    if let Some(Same(_)) = tgt_v.try_as::<Same<Value>>() {
        return b_v;
    }

    todo!()
}

fn do_cong(tgt_v: Value, _b_v: Value, fun_v: Value) -> Value {
    if let Some(Same(v)) = tgt_v.try_as::<Same<Value>>() {
        return values::same(do_ap(&fun_v, v.clone()));
    }

    todo!()
}
