use crate::basics::{ctx_to_env, is_var_name, Closure, Core, Ctx, Env, Value, N};
use crate::values;
use crate::values::functions::do_ap;
use std::borrow::Cow;

fn later(env: Env, exp: Core) -> Value {
    values::later(env, exp)
}

pub fn now(v: &Value) -> Cow<Value> {
    match v {
        Value::Obj(obj) => obj.now(v),
    }
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
    match &*now(tv) {
        Value::Obj(obj) => obj.read_back_type(ctx).unwrap(),
    }
}

pub fn read_back(ctx: &Ctx, tv: &Value, v: &Value) -> Core {
    use Value::*;
    let ntv = now(tv);
    let nv = now(v);

    if let Some((_, ne)) = nv.as_neutral() {
        return read_back_neutral(ctx, ne);
    }

    // first try combinations where we don't need to own v
    match (&*ntv, &*nv) {
        (Obj(obj), v) => return obj.read_back(ctx, tv, v).unwrap(),
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

    #[test]
    fn test_delayed() {
        let env = Env::new();
        let delayed_value = later(env, Core::U);
        assert_eq!(*now(&delayed_value), values::universe());
    }
}
