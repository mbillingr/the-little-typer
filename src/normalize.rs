use crate::basics::{ctx_to_env, Core, Ctx, Delayed, Env, Value};
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

fn later(env: Env, exp: Core) -> Value {
    Value::Delay(Rc::new(RefCell::new(Delayed::Later(env, exp))))
}

fn undelay(c: &Rc<RefCell<Delayed>>) -> Value {
    match &*c.borrow() {
        Delayed::Later(env, exp) => now(&val_of(env, exp)).into_owned(),
        _ => unreachable!(),
    }
}

fn now(v: &Value) -> Cow<Value> {
    match v {
        Value::Delay(delayed) => {
            if let Delayed::Value(x) = &*delayed.borrow() {
                return Cow::Owned(x.clone())
            }
            let the_value = undelay(delayed);
            delayed.replace(Delayed::Value(the_value.clone()));
            Cow::Owned(the_value)
        }
        other => Cow::Borrowed(other),
    }
}

pub fn val_of(_env: &Env, e: &Core) -> Value {
    match e {
        Core::U => Value::Universe,
        Core::Atom => Value::Atom,
        Core::Quote(a) => Value::Quote(a.clone()),
        _ => todo!("{:?}", e),
    }
}

pub fn read_back_type(_ctx: &Ctx, tv: &Value) -> Core {
    match *now(tv) {
        Value::Universe => Core::U,
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
