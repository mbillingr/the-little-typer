use crate::basics::{
    Closure, Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, The, Value, N,
};
use crate::errors;
use crate::errors::Error;
use crate::normalize::{read_back, val_in_ctx};
use crate::symbol::Symbol;
use crate::types::functions::do_ap;
use crate::types::natural::{Add1, Zero};
use crate::types::values::{add1, later};
use crate::types::{cores, values};
use std::collections::HashSet;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub struct IndNat {
    target: Core,
    motive: Core,
    base: Core,
    step: Core,
}

#[derive(Debug)]
pub struct NeutralIndNat {
    target: N,
    motive: The,
    base: The,
    step: The,
}

impl IndNat {
    pub fn new(target: Core, motive: Core, base: Core, step: Core) -> Self {
        IndNat {
            target,
            motive,
            base,
            step,
        }
    }
}

impl CoreInterface for IndNat {
    impl_core_defaults!(
        (target, motive, base, step),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        no_type,
        check_by_synth
    );

    fn val_of(&self, env: &Env) -> Value {
        do_ind_nat(
            later(env.clone(), self.target.clone()),
            later(env.clone(), self.motive.clone()),
            later(env.clone(), self.base.clone()),
            later(env.clone(), self.step.clone()),
        )
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> errors::Result<(Core, Core)> {
        let tgt_out = self.target.check(ctx, r, &values::nat())?;
        let mot_out = self.motive.check(
            ctx,
            r,
            &values::pi("n", values::nat(), Closure::higher(|_| values::universe())),
        )?;
        let mot_val = val_in_ctx(ctx, &mot_out);
        let b_out = self.base.check(ctx, r, &do_ap(&mot_val, values::zero()))?;
        let tv = pi_type!(((n_minus_1, values::nat())), {
            let mot_val = mot_val.clone();
            pi_type!(
                ((_ih, do_ap(&mot_val, n_minus_1.clone()))),
                do_ap(&mot_val, values::add1(n_minus_1.clone()))
            )
        });
        let s_out = self.step.check(ctx, r, &tv)?;
        Ok((
            cores::app(mot_out.clone(), tgt_out.clone()),
            cores::ind_nat(tgt_out, mot_out, b_out, s_out),
        ))
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let tgt = self.target.resugar();
        let mot = self.motive.resugar();
        let bse = self.base.resugar();
        let stp = self.step.resugar();

        (
            &(&tgt.0 | &mot.0) | &(&bse.0 | &stp.0),
            cores::ind_nat(tgt.1, mot.1, bse.1, stp.1),
        )
    }
}

impl std::fmt::Display for IndNat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(ind-Nat {} {} {} {})",
            self.target, self.motive, self.base, self.step
        )
    }
}

fn do_ind_nat(tgt_v: Value, mot_v: Value, b_v: Value, s_v: Value) -> Value {
    match tgt_v.try_as::<Zero>() {
        Some(_) => return b_v,
        None => {}
    };

    match tgt_v.try_as::<Add1<Value>>() {
        Some(Add1(n_minus_1v)) => {
            return do_ap(
                &do_ap(&s_v, n_minus_1v.clone()),
                do_ind_nat(n_minus_1v.clone(), mot_v, b_v, s_v),
            )
        }
        None => {}
    };

    match tgt_v.as_neutral() {
        Some((_, ne)) => {
            return values::neutral(
                do_ap(&mot_v, tgt_v.clone()),
                NeutralIndNat {
                    target: ne.clone(),
                    motive: The(
                        pi_type!(((_x, values::nat())), values::universe()),
                        mot_v.clone(),
                    ),
                    base: The(do_ap(&mot_v, values::zero()), b_v),
                    step: The(
                        values::pi(
                            "n_minus_one",
                            values::nat(),
                            Closure::higher(move |n_minus_one| {
                                let mot_v = mot_v.clone();
                                values::pi(
                                    "ih",
                                    do_ap(&mot_v, n_minus_one.clone()),
                                    Closure::higher(move |_ih| {
                                        do_ap(&mot_v, add1(n_minus_one.clone()))
                                    }),
                                )
                            }),
                        ),
                        s_v,
                    ),
                },
            );
        }
        None => {}
    };

    unreachable!("{:?}", tgt_v)
}

impl NeutralInterface for NeutralIndNat {
    fn read_back_neutral(&self, ctx: &Ctx) -> errors::Result<Core> {
        let NeutralIndNat {
            target: tgt,
            motive: The(mot_tv, mot_v),
            base: The(b_tv, b_v),
            step: The(s_tv, s_v),
        } = self;
        Ok(cores::ind_nat(
            tgt.read_back_neutral(ctx)?,
            read_back(ctx, mot_tv, mot_v)?,
            read_back(ctx, b_tv, b_v)?,
            read_back(ctx, s_tv, s_v)?,
        ))
    }
}
