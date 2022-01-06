use crate::basics::{
    Closure, Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, The, Value, N,
};
use crate::errors;
use crate::normalize::read_back;
use crate::symbol::Symbol;
use crate::types::functions::do_ap;
use crate::types::natural::{Add1, Zero};
use crate::types::values::later;
use crate::types::{cores, values, MaybeTyped};
use std::collections::HashSet;

ternary_eliminator!(WhichNat, do_which_nat, synth_which_nat);

#[derive(Debug)]
pub struct NeutralWhichNat(pub N, pub The, pub The);

fn synth_which_nat(
    this: &WhichNat,
    ctx: &Ctx,
    r: &Renaming,
    b: &Core,
) -> errors::Result<(Core, Core)> {
    let tgt_out = this.target.check(ctx, r, &values::nat())?;
    let (b_t_out, b_out) = b.synth(ctx, r)?;
    let n_minus_one = ctx.fresh(&Symbol::new("n-1"));
    let s_out = this.step.check(
        ctx,
        r,
        &values::pi(
            n_minus_one.clone(),
            values::nat(),
            Closure::FirstOrder {
                env: ctx.to_env(),
                var: n_minus_one,
                expr: b_t_out.clone(),
            },
        ),
    )?;
    Ok((
        b_t_out.clone(),
        cores::which_nat_desugared(tgt_out, b_t_out, b_out, s_out),
    ))
}

fn do_which_nat(tgt_v: Value, bt_v: Value, b_v: Value, s_v: Value) -> Value {
    match tgt_v.try_as::<Zero>() {
        Some(_) => return b_v,
        None => {}
    };

    match tgt_v.try_as::<Add1<Value>>() {
        Some(Add1(n_minus_1v)) => return do_ap(&s_v, n_minus_1v.clone()),
        None => {}
    };

    match tgt_v.as_neutral() {
        Some((_, ne)) => {
            return values::neutral(
                bt_v.clone(),
                NeutralWhichNat(
                    ne.clone(),
                    The(bt_v.clone(), b_v),
                    The(pi_type!(((_n, values::nat())), { bt_v.clone() }), s_v),
                ),
            )
        }
        None => {}
    };

    unreachable!("{:?}", tgt_v)
}

impl NeutralInterface for NeutralWhichNat {
    fn read_back_neutral(&self, ctx: &Ctx) -> errors::Result<Core> {
        let NeutralWhichNat(tgt, The(b_tv, b_v), The(s_tv, s_v)) = self;
        Ok(cores::which_nat(
            tgt.read_back_neutral(ctx)?,
            cores::the(b_tv.read_back_type(ctx)?, read_back(ctx, b_tv, b_v)?),
            read_back(ctx, s_tv, s_v)?,
        ))
    }
}
