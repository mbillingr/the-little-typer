use crate::basics::{
    ctx_to_env, fresh, is_var_name, Closure, Core, Ctx, Delayed, Env, SharedBox, SharedBoxGuard,
    Value, N, R,
};
use std::borrow::Cow;

fn later(env: Env, exp: Core) -> Value {
    Value::Delay(SharedBox::new(Delayed::Later(env, exp)))
}

fn undelay(c: &SharedBoxGuard<Delayed>) -> Value {
    match &**c {
        Delayed::Later(env, exp) => now(&val_of(env, exp)).into_owned(),
        _ => unreachable!(),
    }
}

pub fn now(v: &Value) -> Cow<Value> {
    match v {
        Value::Delay(delayed) => {
            let mut dv = delayed.write_lock();
            if let Delayed::Value(x) = &*dv {
                return Cow::Owned(x.clone());
            }
            let the_value = undelay(&dv);
            dv.replace(Delayed::Value(the_value.clone()));
            Cow::Owned(the_value)
        }
        other => Cow::Borrowed(other),
    }
}

pub fn val_of(env: &Env, e: &Core) -> Value {
    match e {
        Core::The(_, expr) => val_of(env, expr),
        Core::U => Value::Universe,
        Core::Nat => Value::Nat,
        Core::Zero => Value::Zero,
        Core::Add1(n) => Value::add1(later(env.clone(), (**n).clone())),
        Core::Fun(_) => panic!("Attempt to evaluate -> (should have been converted to Pi)"),
        Core::PiStar(_, _) => panic!("Attempt to evaluate Pi* (should have been converted to Pi)"),
        Core::Pi(x, a, b) => {
            let av = later(env.clone(), (**a).clone());
            Value::pi(
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
        Core::Lambda(x, b) => Value::lam(
            x.clone(),
            Closure::FirstOrder {
                env: env.clone(),
                var: x.clone(),
                expr: (**b).clone(),
            },
        ),
        Core::Atom => Value::Atom,
        Core::Quote(a) => Value::Quote(a.clone()),
        Core::AppStar(_, _) => panic!("Attempt to evaluate n-ary application (should have been converted to sequence of unary applications)"),
        Core::App(rator, rand) => do_ap(
            later(env.clone(), (**rator).clone()),
            later(env.clone(), (**rand).clone()),
        ),
        Core::Symbol(x) if is_var_name(x) => env.var_val(x).unwrap(),
        Core::Symbol(x) => panic!("No evaluator for {}", x.name()),
    }
}

fn do_ap(rator: Value, rand: Value) -> Value {
    match &*now(&rator) {
        Value::Lam { body, .. } => body.val_of(rand),
        _ => todo!("{:?}", rator),
    }
}

pub fn read_back_type(ctx: &Ctx, tv: &Value) -> Core {
    match &*now(tv) {
        Value::Universe => Core::U,
        Value::Nat => Core::Nat,
        Value::Pi {
            arg_name: x,
            arg_type: a,
            result_type: c,
        } => {
            let ae = read_back_type(ctx, a);
            let x_hat = fresh(ctx, x);

            let ctx_hat = ctx.bind_free(x_hat.clone(), (**a).clone()).unwrap();
            let r = read_back_type(
                &ctx_hat,
                &c.val_of(Value::Neu(a.clone(), N::Var(x_hat.clone()))),
            );
            Core::pi(x_hat, ae, R::new(r))
        }
        Value::Atom => Core::Atom,
        _ => todo!("{:?}", tv),
    }
}

pub fn read_back(ctx: &Ctx, tv: &Value, v: &Value) -> Core {
    use Value::*;
    let ntv = now(tv);
    let nv = now(v);

    // first try combinations where we don't need to own v
    match (&*ntv, &*nv) {
        (Universe, v) => return read_back_type(ctx, &v),
        (Nat, Value::Zero) => return Core::Zero,
        (Nat, Value::Add1(n_minus_one)) => return Core::add1(read_back(ctx, tv, n_minus_one)),
        _ => {}
    }

    // the remaining combinations need to take ownership of v
    match (&*ntv, nv.into_owned()) {
        (Atom, Quote(a)) => Core::Quote(a),
        (
            Pi {
                arg_name: x,
                arg_type: a,
                result_type: c,
            },
            f,
        ) => {
            let y = match &f {
                Value::Lam { arg_name, .. } => arg_name,
                _ => x,
            };
            let x_hat = fresh(ctx, y);
            return Core::lambda(
                x_hat.clone(),
                read_back(
                    &ctx.bind_free(x_hat.clone(), (**a).clone()).unwrap(),
                    &c.val_of(Value::neu(a.clone(), N::Var(x_hat.clone()))),
                    &do_ap(f, Value::neu(a.clone(), N::Var(x_hat))),
                ),
            );
        }
        (_, Value::Neu(_, ne)) => read_back_neutral(ctx, ne),
        (ntv, nv) => todo!("{:?} {:?}", ntv, nv),
    }
}

fn read_back_neutral(_ctx: &Ctx, ne: N) -> Core {
    match ne {
        N::Var(x) => Core::Symbol(x),
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
        assert_eq!(*now(&delayed_value), Value::Universe);
    }
}
