use crate::basics::{Closure, Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface};
use crate::errors;
use crate::errors::Error;
use crate::normalize::{now, val_in_ctx};
use crate::symbol::Symbol;
use crate::types::functions::do_ap;
use crate::types::natural::{Add1, Zero};
use crate::types::values::later;
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
    match now(&tgt_v).as_any().downcast_ref::<Zero>() {
        Some(_) => return b_v,
        None => {}
    };

    match now(&tgt_v).as_any().downcast_ref::<Add1<Value>>() {
        Some(Add1(n_minus_1v)) => {
            return do_ap(
                &do_ap(&s_v, n_minus_1v.clone()),
                do_ind_nat(n_minus_1v.clone(), mot_v, b_v, s_v),
            )
        }
        None => {}
    };

    todo!()

    //unreachable!("{:?}", now(&tgt_v))
}
