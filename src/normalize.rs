use crate::basics::{Core, CoreInterface, Ctx, Value};
use crate::errors::Result;

pub fn now(v: &Value) -> &Value {
    v.now()
}

pub fn read_back(ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
    let ntv = now(tv);
    let nv = now(v);

    if let Some((_, ne)) = nv.as_neutral() {
        ne.read_back_neutral(ctx)
    } else {
        ntv.read_back(ctx, tv, &nv)
    }
}

pub fn val_in_ctx(ctx: &Ctx, e: &Core) -> Value {
    let env = &ctx.to_env();
    e.val_of(env)
}
