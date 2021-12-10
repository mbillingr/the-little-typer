use crate::basics::{ctx_to_env, Core, Ctx, Env, Value};

pub fn val_of(env: &Env, e: &Core) -> Value {
    unimplemented!()
}

pub fn read_back_type(ctx: &Ctx, tv: &Value) -> Core {
    unimplemented!()
}

pub fn read_back(ctx: &Ctx, tv: &Value, v: &Value) -> Core {
    unimplemented!()
}

pub fn val_in_ctx(ctx: &Ctx, e: &Core) -> Value {
    val_of(&ctx_to_env(ctx), e)
}
