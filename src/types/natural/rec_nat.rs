use crate::basics::{Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, The, Value, N};
use crate::errors;
use crate::normalize::val_in_ctx;
use crate::symbol::Symbol;
use crate::types::functions::do_ap;
use crate::types::natural::{Add1, Zero};
use crate::types::values::later;
use crate::types::{cores, values, MaybeTyped};
use std::collections::HashSet;

ternary_eliminator!(RecNat, do_rec_nat, synth_rec_nat);

#[derive(Debug)]
pub struct NeutralRecNat(pub N, pub The, pub The);

fn synth_rec_nat(this: &RecNat, ctx: &Ctx, r: &Renaming, b: &Core) -> errors::Result<(Core, Core)> {
    let tgt_out = this.target.check(ctx, r, &values::nat())?;
    let (b_t_out, b_out) = b.synth(ctx, r)?;
    let s_out = this.step.check(ctx, r, &{
        let n_minus_one = ctx.fresh(&Symbol::new("n-1"));
        let old = ctx.fresh(&Symbol::new("old"));
        val_in_ctx(
            ctx,
            &cores::pi(
                n_minus_one,
                cores::nat(),
                cores::pi(old, b_t_out.clone(), b_t_out.clone()),
            ),
        )
    })?;
    Ok((
        b_t_out.clone(),
        cores::rec_nat_desugared(tgt_out, b_t_out, b_out, s_out),
    ))
}

fn do_rec_nat(tgt_v: Value, bt_v: Value, b_v: Value, s_v: Value) -> Value {
    _do_rec_nat(&tgt_v, &bt_v, &b_v, &s_v)
}

fn _do_rec_nat(tgt_v: &Value, bt_v: &Value, b_v: &Value, s_v: &Value) -> Value {
    match tgt_v.try_as::<Zero>() {
        Some(_) => return b_v.clone(),
        None => {}
    };

    match tgt_v.try_as::<Add1<Value>>() {
        Some(Add1(n_minus_1v)) => {
            return do_ap(
                &do_ap(s_v, n_minus_1v.clone()),
                _do_rec_nat(n_minus_1v, bt_v, b_v, s_v),
            )
        }
        None => {}
    };

    unreachable!("{:?}", tgt_v)
}

impl NeutralInterface for NeutralRecNat {
    fn read_back_neutral(&self, _ctx: &Ctx) -> errors::Result<Core> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recnat_desugars_to_typed_base() {
        let (typ, exp) = RecNat::untyped(
            cores::zero(),
            cores::quote("base"),
            cores::lambda("x", cores::lambda("y", cores::quote("step"))),
        )
        .synth(&Ctx::new(), &Renaming::new())
        .unwrap();

        assert_eq!(typ, cores::atom());
        assert_eq!(
            exp,
            cores::rec_nat_desugared(
                cores::zero(),
                cores::atom(),
                cores::quote("base"),
                cores::lambda("x", cores::lambda("y", cores::quote("step")))
            )
        );
    }

    #[test]
    fn first_commandment() {
        let exp = RecNat::typed(
            cores::zero(),
            cores::atom(),
            cores::quote("base"),
            cores::lambda("x", cores::lambda("y", cores::quote("step"))),
        );

        assert_eq!(exp.val_of(&Env::new()), values::quote("base"));
    }

    #[test]
    fn second_commandment() {
        let exp = RecNat::typed(
            cores::add1(cores::zero()),
            cores::atom(),
            cores::quote("base"),
            cores::lambda("x", cores::lambda("y", cores::quote("step"))),
        );
        assert_eq!(exp.val_of(&Env::new()), values::quote("step"));

        let exp = RecNat::typed(
            cores::add1(cores::zero()),
            cores::nat(),
            cores::add1(cores::zero()),
            cores::lambda(
                "x",
                cores::lambda("y", cores::cons(cores::refer("x"), cores::refer("y"))),
            ),
        );
        assert_eq!(
            exp.val_of(&Env::new()),
            values::cons(values::zero(), values::add1(values::zero())),
        );
    }
}
