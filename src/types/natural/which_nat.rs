use crate::basics::{
    Closure, Core, CoreInterface, Ctx, Env, Renaming, The, Value, ValueInterface, N,
};
use crate::errors;
use crate::errors::Error;
use crate::normalize::now;
use crate::symbol::Symbol;
use crate::types::functions::do_ap;
use crate::types::natural::{Add1, MaybeTyped, Zero};
use crate::types::neutral::Neutral;
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
                let e = &self.target;
                let tv = &values::nat();
                let tgt_out = e.check(ctx, r, tv)?;
                let inp = b;
                let (b_t_out, b_out) = inp.synth(ctx, r)?;
                let x = &Symbol::new("n-1");
                let n_minus_one = ctx.fresh(x);
                let e = &self.step;
                let tv = &values::pi(
                    n_minus_one.clone(),
                    values::nat(),
                    Closure::FirstOrder {
                        env: ctx.to_env(),
                        var: n_minus_one,
                        expr: b_t_out.clone(),
                    },
                );
                let s_out = e.check(ctx, r, tv)?;
                Ok((
                    b_t_out.clone(),
                    cores::which_nat_desugared(tgt_out, b_t_out, b_out, s_out),
                ))
            }
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        let term = &self.target;
        let tgt = term.resugar();
        let bas = self.base.resugar();
        let term = &self.step;
        let stp = term.resugar();
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
    match now(&tgt_v).as_any().downcast_ref::<Zero>() {
        Some(_) => return b_v,
        None => {}
    };

    match now(&tgt_v).as_any().downcast_ref::<Add1<Value>>() {
        Some(Add1(n_minus_1v)) => return do_ap(&s_v, n_minus_1v.clone()),
        None => {}
    };

    match now(&tgt_v).as_any().downcast_ref::<Neutral>() {
        Some(Neutral { kind: ne, .. }) => {
            return values::neutral(
                bt_v.clone(),
                N::which_nat(
                    ne.clone(),
                    The(bt_v.clone(), b_v),
                    The(pi_type!(((_n, values::nat())), { bt_v.clone() }), s_v),
                ),
            )
        }
        None => {}
    };

    unreachable!("{:?}", now(&tgt_v))
}
