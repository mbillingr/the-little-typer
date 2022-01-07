use crate::basics::{Core, CoreInterface, Ctx, Value};
use crate::errors::Result;

pub fn read_back(ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
    if let Some((_, ne)) = v.as_neutral() {
        ne.read_back_neutral(ctx)
    } else {
        tv.read_back(ctx, &v)
    }
}

pub fn val_in_ctx(ctx: &Ctx, e: &Core) -> Value {
    let env = &ctx.to_env();
    e.val_of(env)
}
