use crate::basics::{
    Closure, Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, The, Value, N,
};
use crate::errors;
use crate::normalize::read_back;
use crate::symbol::Symbol;
use crate::types::functions::do_ap;
use crate::types::natural::{Add1, MaybeTyped, Zero};
use crate::types::values::later;
use crate::types::{cores, values};
use std::collections::HashSet;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub struct WhichNat {
    target: Core,
    base: MaybeTyped,
    step: Core,
}

#[derive(Debug)]
pub struct NeutralWhichNat(pub N, pub The, pub The);

impl WhichNat {
    pub fn typed(target: Core, base_t: Core, base: Core, step: Core) -> Self {
        WhichNat {
            target,
            step,
            base: MaybeTyped::The(base_t, base),
        }
    }

    pub fn untyped(target: Core, base: Core, step: Core) -> Self {
        WhichNat {
            target,
            step,
            base: MaybeTyped::Plain(base),
        }
    }
}

impl CoreInterface for WhichNat {
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
            MaybeTyped::Plain(_) => unimplemented!("evaluate a desugared which-Nat instead"),
            MaybeTyped::The(bt, b) => do_which_nat(
                later(env.clone(), self.target.clone()),
                later(env.clone(), bt.clone()),
                later(env.clone(), b.clone()),
                later(env.clone(), self.step.clone()),
            ),
        }
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> errors::Result<(Core, Core)> {
        match &self.base {
            MaybeTyped::The(_, _) => unimplemented!("already synth'ed"),
            MaybeTyped::Plain(b) => {
                let tgt_out = self.target.check(ctx, r, &values::nat())?;
                let (b_t_out, b_out) = b.synth(ctx, r)?;
                let n_minus_one = ctx.fresh(&Symbol::new("n-1"));
                let s_out = self.step.check(
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
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let tgt = self.target.resugar();
        let bas = self.base.resugar();
        let stp = self.step.resugar();
        (
            &tgt.0 | &(&bas.0 | &stp.0),
            Core::new(WhichNat {
                target: tgt.1,
                base: bas.1,
                step: stp.1,
            }),
        )
    }
}

impl std::fmt::Display for WhichNat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.base {
            MaybeTyped::Plain(base) => {
                write!(f, "(which-Nat {} {} {})", self.target, base, self.step)
            }
            MaybeTyped::The(base_type, base) => write!(
                f,
                "(which-Nat {} (the {} {}) {})",
                self.target, base_type, base, self.step
            ),
        }
    }
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
