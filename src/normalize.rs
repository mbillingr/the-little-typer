use crate::basics::{
    ctx_to_env, fresh, Closure, Core, Ctx, Delayed, Env, SharedBox, SharedBoxGuard, Value, N, R,
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

fn now(v: &Value) -> Cow<Value> {
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
        Core::U => Value::Universe,
        Core::Nat => Value::Nat,
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
        Core::Atom => Value::Atom,
        Core::Quote(a) => Value::Quote(a.clone()),
        _ => todo!("{:?}", e),
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
            Core::Pi(x_hat, R::new(ae), R::new(r))
        }
        Value::Atom => Core::Atom,
        _ => todo!("{:?}", tv),
    }
}

pub fn read_back(ctx: &Ctx, tv: &Value, v: &Value) -> Core {
    use Value::*;
    let tv = now(tv);
    let v = now(v);

    // first try combinations where we don't need to own v
    match (&*tv, &*v) {
        (Universe, v) => return read_back_type(ctx, &v),
        _ => {}
    }

    // the remaining combinations need to take ownership of v
    match (&*tv, v.into_owned()) {
        (Atom, Quote(a)) => Core::Quote(a),
        (ntv, nv) => todo!("{:?} {:?}", ntv, nv),
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
