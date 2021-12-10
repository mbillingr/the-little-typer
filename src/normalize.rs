use crate::basics::{ctx_to_env, Core, Ctx, Env, Value};

fn now(v: &Value) -> &Value {
    match v {
        Value::Delay(_) => unimplemented!(),
        other => other,
    }
}

pub fn val_of(env: &Env, e: &Core) -> Value {
    match e {
        Core::Atom => Value::Atom,
        Core::Quote(a) => Value::Quote(a.clone()),
        _ => unimplemented!("{:?}", e),
    }
}

pub fn read_back_type(ctx: &Ctx, tv: &Value) -> Core {
    match now(tv) {
        Value::Atom => Core::Atom,
        _ => unimplemented!("{:?}", tv),
    }
}

pub fn read_back(ctx: &Ctx, tv: &Value, v: &Value) -> Core {
    use Value::*;
    match (now(tv), now(v)) {
        (Atom, Quote(a)) => Core::Quote(a.clone()),
        (ntv, nv) => unimplemented!("{:?} {:?}", ntv, nv),
    }
}

pub fn val_in_ctx(ctx: &Ctx, e: &Core) -> Value {
    val_of(&ctx_to_env(ctx), e)
}
