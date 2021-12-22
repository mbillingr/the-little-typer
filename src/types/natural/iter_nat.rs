use crate::basics::{Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, The, Value, N};
use crate::errors;
use crate::errors::Error;
use crate::normalize::val_in_ctx;
use crate::symbol::Symbol;
use crate::types::functions::do_ap;
use crate::types::natural::{Add1, MaybeTyped, Zero};
use crate::types::values::later;
use crate::types::{cores, values};
use std::collections::HashSet;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub struct IterNat {
    target: Core,
    base: MaybeTyped,
    step: Core,
}

#[derive(Debug)]
pub struct NeutralIterNat(pub N, pub The, pub The);

impl IterNat {
    pub fn typed(target: Core, base_t: Core, base: Core, step: Core) -> Self {
        IterNat {
            target,
            step,
            base: MaybeTyped::The(base_t, base),
        }
    }

    pub fn untyped(target: Core, base: Core, step: Core) -> Self {
        IterNat {
            target,
            step,
            base: MaybeTyped::Plain(base),
        }
    }
}

impl CoreInterface for IterNat {
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
            MaybeTyped::Plain(_) => unimplemented!("evaluate a desugared iter-Nat instead"),
            MaybeTyped::The(bt, b) => do_iter_nat(
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
                    let old = ctx.fresh(&Symbol::new("old"));
                    val_in_ctx(ctx, &cores::pi(old, b_t_out.clone(), b_t_out.clone()))
                })?;
                Ok((
                    b_t_out.clone(),
                    cores::iter_nat_desugared(tgt_out, b_t_out, b_out, s_out),
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
            Core::new(IterNat {
                target: tgt.1,
                base: bas.1,
                step: stp.1,
            }),
        )
    }
}

impl std::fmt::Display for IterNat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.base {
            MaybeTyped::Plain(base) => {
                write!(f, "(iter-Nat {} {} {})", self.target, base, self.step)
            }
            MaybeTyped::The(base_type, base) => write!(
                f,
                "(iter-Nat {} (the {} {}) {})",
                self.target, base_type, base, self.step
            ),
        }
    }
}

fn do_iter_nat(tgt_v: &Value, bt_v: Value, b_v: Value, s_v: &Value) -> Value {
    match tgt_v.try_as::<Zero>() {
        Some(_) => return b_v,
        None => {}
    };

    match tgt_v.try_as::<Add1<Value>>() {
        Some(Add1(n_minus_1v)) => return do_ap(s_v, do_iter_nat(n_minus_1v, bt_v, b_v, s_v)),
        None => {}
    };

    /*match tgt_v.as_neutral() {
        Some((_, ne)) => {
            return values::neutral(
                bt_v.clone(),
                NeutralIterNat(
                    ne.clone(),
                    The(bt_v.clone(), b_v),
                    The(pi_type!(((_n, values::nat())), { bt_v.clone() }), s_v),
                ),
            )
        }
        None => {}
    };*/

    unreachable!("{:?}", tgt_v)
}

impl NeutralInterface for NeutralIterNat {
    fn read_back_neutral(&self, _ctx: &Ctx) -> errors::Result<Core> {
        todo!()
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
