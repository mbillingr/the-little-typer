use crate::basics::{Core, CoreInterface, Ctx, Value, ValueInterface, N};
use crate::errors::Result;
use std::borrow::Cow;

pub fn now(v: &Value) -> Cow<Value> {
    match v.now() {
        None => Cow::Borrowed(v),
        Some(x) => Cow::Owned(x),
    }
}

pub fn read_back_type(ctx: &Ctx, tv: &Value) -> Result<Core> {
    tv.read_back_type(ctx)
}

pub fn read_back(ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
    let ntv = now(tv);
    let nv = now(v);

    if let Some((_, ne)) = nv.as_neutral() {
        read_back_neutral(ctx, ne)
    } else {
        ntv.read_back(ctx, tv, &nv)
    }
}

pub fn read_back_neutral(ctx: &Ctx, ne: &N) -> Result<Core> {
    ne.read_back_neutral(ctx)
}

pub fn val_in_ctx(ctx: &Ctx, e: &Core) -> Value {
    let env = &ctx.to_env();
    e.val_of(env)
}
