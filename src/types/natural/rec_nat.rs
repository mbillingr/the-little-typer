use crate::basics::{Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, The, Value, N};
use crate::errors;
use crate::normalize::val_in_ctx;
use crate::symbol::Symbol;
use crate::types::functions::do_ap;
use crate::types::natural::{Add1, MaybeTyped, Zero};
use crate::types::values::later;
use crate::types::{cores, values};
use std::collections::HashSet;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub struct RecNat {
    target: Core,
    base: MaybeTyped,
    step: Core,
}

#[derive(Debug)]
pub struct NeutralRecNat(pub N, pub The, pub The);

impl RecNat {
    pub fn typed(target: Core, base_t: Core, base: Core, step: Core) -> Self {
        RecNat {
            target,
            step,
            base: MaybeTyped::The(base_t, base),
        }
    }

    pub fn untyped(target: Core, base: Core, step: Core) -> Self {
        RecNat {
            target,
            step,
            base: MaybeTyped::Plain(base),
        }
    }
}

impl CoreInterface for RecNat {
    impl_core_defaults!(
        (target, base, step),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        no_type,
        check_by_synth
    );

    fn val_of(&self, env: &Env) -> Value {
        match &self.base {
            MaybeTyped::Plain(_) => unimplemented!("evaluate a desugared rec-Nat instead"),
            MaybeTyped::The(bt, b) => do_rec_nat(
                &later(env.clone(), self.target.clone()),
                later(env.clone(), bt.clone()),
                later(env.clone(), b.clone()),
                &later(env.clone(), self.step.clone()),
            ),
        }
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> errors::Result<(Core, Core)> {
        match &self.base {
            MaybeTyped::The(_, _) => unimplemented!("already synth'ed"),
            MaybeTyped::Plain(b) => {
                let tgt_out = self.target.check(ctx, r, &values::nat())?;
                let (b_t_out, b_out) = b.synth(ctx, r)?;
                let s_out = self.step.check(ctx, r, &{
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
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let tgt = self.target.resugar();
        let bas = self.base.resugar();
        let stp = self.step.resugar();
        (
            &tgt.0 | &(&bas.0 | &stp.0),
            Core::new(RecNat {
                target: tgt.1,
                base: bas.1,
                step: stp.1,
            }),
        )
    }
}

impl std::fmt::Display for RecNat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.base {
            MaybeTyped::Plain(base) => {
                write!(f, "(rec-Nat {} {} {})", self.target, base, self.step)
            }
            MaybeTyped::The(base_type, base) => write!(
                f,
                "(rec-Nat {} (the {} {}) {})",
                self.target, base_type, base, self.step
            ),
        }
    }
}

fn do_rec_nat(tgt_v: &Value, bt_v: Value, b_v: Value, s_v: &Value) -> Value {
    match tgt_v.try_as::<Zero>() {
        Some(_) => return b_v,
        None => {}
    };

    match tgt_v.try_as::<Add1<Value>>() {
        Some(Add1(n_minus_1v)) => {
            return do_ap(
                &do_ap(s_v, n_minus_1v.clone()),
                do_rec_nat(n_minus_1v, bt_v, b_v, s_v),
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
