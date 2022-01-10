use crate::basics::{Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, The, Value, N};
use crate::errors;
use crate::normalize::{read_back, val_in_ctx};
use crate::symbol::Symbol;
use crate::types::functions::do_ap;
use crate::types::natural::{Add1, Zero};
use crate::types::values::later;
use crate::types::{cores, values, MaybeTyped};
use std::collections::HashSet;

ternary_eliminator!(IterNat, do_iter_nat, synth_iter_nat);

#[derive(Debug)]
pub struct NeutralIterNat(pub N, pub The, pub The);

fn synth_iter_nat(
    this: &IterNat,
    ctx: &Ctx,
    r: &Renaming,
    b: &Core,
) -> errors::Result<(Core, Core)> {
    let tgt_out = this.target.check(ctx, r, &values::nat())?;
    let (b_t_out, b_out) = b.synth(ctx, r)?;
    let s_out = this.step.check(ctx, r, &{
        let old = ctx.fresh(&Symbol::new("old"));
        val_in_ctx(ctx, &cores::pi(old, b_t_out.clone(), b_t_out.clone()))
    })?;
    Ok((
        b_t_out.clone(),
        cores::iter_nat_desugared(tgt_out, b_t_out, b_out, s_out),
    ))
}

fn do_iter_nat(tgt_v: Value, bt_v: Value, b_v: Value, s_v: Value) -> Value {
    _do_iter_nat(&tgt_v, bt_v, b_v, &s_v)
}

fn _do_iter_nat(tgt_v: &Value, bt_v: Value, b_v: Value, s_v: &Value) -> Value {
    match tgt_v.try_as::<Zero>() {
        Some(_) => return b_v,
        None => {}
    };

    match tgt_v.try_as::<Add1<Value>>() {
        Some(Add1(n_minus_1v)) => return do_ap(s_v, _do_iter_nat(n_minus_1v, bt_v, b_v, s_v)),
        None => {}
    };

    match tgt_v.as_neutral() {
        Some((_, ne)) => {
            return values::neutral(
                bt_v.clone(),
                NeutralIterNat(
                    ne.clone(),
                    The(bt_v.clone(), b_v),
                    The(
                        pi_type!(((_n, values::nat())), { bt_v.clone() }),
                        s_v.clone(),
                    ),
                ),
            )
        }
        None => {}
    };

    unreachable!("{:?}", tgt_v)
}

impl NeutralInterface for NeutralIterNat {
    fn read_back_neutral(&self, ctx: &Ctx) -> errors::Result<Core> {
        let NeutralIterNat(tgt, The(b_tv, b_v), The(s_tv, s_v)) = self;
        Ok(cores::iter_nat_desugared(
            tgt.read_back_neutral(ctx)?,
            b_tv.read_back_type(ctx)?,
            read_back(ctx, b_tv, b_v)?,
            read_back(ctx, s_tv, s_v)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iternat_desugars_to_typed_base() {
        let (typ, exp) = IterNat::untyped(
            cores::zero(),
            cores::quote("base"),
            cores::lambda("x", cores::quote("step")),
        )
        .synth(&Ctx::new(), &Renaming::new())
        .unwrap();

        assert_eq!(typ, cores::atom());
        assert_eq!(
            exp,
            cores::iter_nat_desugared(
                cores::zero(),
                cores::atom(),
                cores::quote("base"),
                cores::lambda("x", cores::quote("step"))
            )
        );
    }

    #[test]
    fn first_commandment() {
        let exp = IterNat::typed(
            cores::zero(),
            cores::atom(),
            cores::quote("base"),
            cores::lambda("x", cores::quote("step")),
        );

        assert_eq!(exp.val_of(&Env::new()), values::quote("base"));
    }

    #[test]
    fn second_commandment() {
        let exp = IterNat::typed(
            cores::add1(cores::zero()),
            cores::atom(),
            cores::quote("base"),
            cores::lambda("x", cores::quote("step")),
        );
        assert_eq!(exp.val_of(&Env::new()), values::quote("step"));

        let exp = IterNat::typed(
            cores::add1(cores::zero()),
            cores::atom(),
            cores::add1(cores::zero()),
            cores::lambda("x", cores::add1(cores::refer("x"))),
        );
        assert_eq!(
            exp.val_of(&Env::new()),
            values::add1(values::add1(values::zero()))
        );
    }
}
