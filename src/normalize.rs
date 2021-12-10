use crate::basics::{ctx_to_env, Core, Ctx, Env, Value};

fn now(v: &Value) -> &Value {
    match v {
        Value::Delay(_) => todo!(),
        other => other,
    }
}

pub fn val_of(env: &Env, e: &Core) -> Value {
    match e {
        Core::U => Value::Universe,
        Core::Atom => Value::Atom,
        Core::Quote(a) => Value::Quote(a.clone()),
        _ => todo!("{:?}", e),
    }
}

pub fn read_back_type(ctx: &Ctx, tv: &Value) -> Core {
    match now(tv) {
        Value::Universe => Core::U,
        Value::Atom => Core::Atom,
        _ => todo!("{:?}", tv),
    }
}

pub fn read_back(ctx: &Ctx, tv: &Value, v: &Value) -> Core {
    use Value::*;
    match (now(tv), now(v)) {
        (Universe, v) => read_back_type(ctx, v),
        (Atom, Quote(a)) => Core::Quote(a.clone()),
        (ntv, nv) => todo!("{:?} {:?}", ntv, nv),
    }
}

pub fn val_in_ctx(ctx: &Ctx, e: &Core) -> Value {
    val_of(&ctx_to_env(ctx), e)
}
