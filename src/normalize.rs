use crate::basics::{ctx_to_env, is_var_name, Closure, Core, Ctx, Env, Value, ValueInterface, N};
use crate::types::functions::do_ap;
use crate::types::values;
use std::borrow::Cow;

fn later(env: Env, exp: Core) -> Value {
    values::later(env, exp)
}

pub fn now(v: &Value) -> Cow<Value> {
    v.now(v)
}

pub fn val_of(env: &Env, e: &Core) -> Value {
    match e {
        Core::The(_, expr) => val_of(env, expr),
        Core::U => values::universe(),
        Core::Nat => values::nat(),
        Core::Zero => values::zero(),
        Core::Add1(n) => values::add1(later(env.clone(), (**n).clone())),
        Core::Fun(_) => panic!("Attempt to evaluate -> (should have been converted to Pi)"),
        Core::PiStar(_, _) => panic!("Attempt to evaluate Pi* (should have been converted to Pi)"),
        Core::Pi(x, a, b) => {
            let av = later(env.clone(), (**a).clone());
            values::pi(
                x.clone(),
                av,
                Closure::FirstOrder {
                    env: env.clone(),
                    var: x.clone(),
                    expr: (**b).clone(),
                },
            )
        }
        Core::LambdaStar(_, _) => panic!("Attempt to evaluate sugared lambda"),
        Core::Lambda(x, b) => values::lambda(
            x.clone(),
            Closure::FirstOrder {
                env: env.clone(),
                var: x.clone(),
                expr: (**b).clone(),
            },
        ),
        Core::Atom => values::atom(),
        Core::Quote(a) => values::quote(a.clone()),
        Core::AppStar(_, _) => panic!("Attempt to evaluate n-ary application (should have been converted to sequence of unary applications)"),
        Core::App(rator, rand) => do_ap(
            &later(env.clone(), (**rator).clone()),
            later(env.clone(), (**rand).clone()),
        ),
        Core::Symbol(x) if is_var_name(x) => env.var_val(x).unwrap(),
        Core::Symbol(x) => panic!("No evaluator for {}", x.name()),
    }
}

pub fn read_back_type(ctx: &Ctx, tv: &Value) -> Core {
    now(tv).read_back_type(ctx).unwrap()
}

pub fn read_back(ctx: &Ctx, tv: &Value, v: &Value) -> Core {
    let ntv = now(tv);
    let nv = now(v);

    if let Some((_, ne)) = nv.as_neutral() {
        read_back_neutral(ctx, ne)
    } else {
        ntv.read_back(ctx, tv, &nv).unwrap()
    }
}

pub fn read_back_neutral(_ctx: &Ctx, ne: &N) -> Core {
    match ne {
        N::Var(x) => Core::Symbol(x.clone()),
    }
}

pub fn val_in_ctx(ctx: &Ctx, e: &Core) -> Value {
    val_of(&ctx_to_env(ctx), e)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::values;

    #[test]
    fn test_delayed() {
        let env = Env::new();
        let delayed_value = later(env, Core::U);
        assert_eq!(*now(&delayed_value), values::universe());
    }
}
