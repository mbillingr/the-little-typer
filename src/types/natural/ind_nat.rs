use crate::alpha::alpha_equiv_aux;
use crate::basics::{
    occurring_names, Closure, Core, CoreInterface, Ctx, Env, Renaming, Value, ValueInterface,
};
use crate::errors::Error;
use crate::normalize::{now, val_in_ctx};
use crate::resugar::resugar_;
use crate::symbol::Symbol;
use crate::typechecker::check;
use crate::types::functions::do_ap;
use crate::types::natural::{Add1, Zero};
use crate::types::values::later;
use crate::types::{cores, values};
use crate::{alpha, errors};
use std::any::Any;
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
    impl_core_defaults!(as_any, same);

    fn occurring_names(&self) -> HashSet<Symbol> {
        &(&occurring_names(&self.target) | &occurring_names(&self.motive))
            | &(&occurring_names(&self.base) | &occurring_names(&self.step))
    }

    fn val_of(&self, env: &Env) -> Value {
        do_ind_nat(
            later(env.clone(), self.target.clone()),
            later(env.clone(), self.motive.clone()),
            later(env.clone(), self.base.clone()),
            later(env.clone(), self.step.clone()),
        )
    }

    fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> errors::Result<Core> {
        Err(Error::NotAType(Core::new(self.clone())))
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> errors::Result<(Core, Core)> {
        let tgt_out = check(ctx, r, &self.target, &values::nat())?;
        let mot_out = check(
            ctx,
            r,
            &self.motive,
            &values::pi("n", values::nat(), Closure::higher(|_| values::universe())),
        )?;
        let mot_val = val_in_ctx(ctx, &mot_out);
        let b_out = check(ctx, r, &self.base, &do_ap(&mot_val, values::zero()))?;
        let s_out = check(
            ctx,
            r,
            &self.step,
            &pi_type!(((n_minus_1, values::nat())), {
                let mot_val = mot_val.clone();
                pi_type!(
                    ((_ih, do_ap(&mot_val, n_minus_1.clone()))),
                    do_ap(&mot_val, values::add1(n_minus_1.clone()))
                )
            }),
        )?;
        Ok((
            cores::app(mot_out.clone(), tgt_out.clone()),
            cores::ind_nat(tgt_out, mot_out, b_out, s_out),
        ))
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            alpha_equiv_aux(lvl, b1, b2, &self.target, &other.target)
                && alpha_equiv_aux(lvl, b1, b2, &self.motive, &other.motive)
                && alpha_equiv_aux(lvl, b1, b2, &self.base, &other.base)
                && alpha_equiv_aux(lvl, b1, b2, &self.step, &other.step)
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let tgt = resugar_(&self.target);
        let mot = resugar_(&self.motive);
        let bse = resugar_(&self.base);
        let stp = resugar_(&self.step);

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