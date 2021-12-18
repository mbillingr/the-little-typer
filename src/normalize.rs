use crate::basics::{ctx_to_env, Core, CoreInterface, Ctx, The, Value, ValueInterface, N};
use crate::errors::Result;
use crate::types::cores;
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
    Ok(match ne {
        N::WhichNat(tgt, The(b_tv, b_v), The(s_tv, s_v)) => cores::which_nat(
            read_back_neutral(ctx, tgt)?,
            cores::the(read_back_type(ctx, b_tv)?, read_back(ctx, b_tv, b_v)?),
            read_back(ctx, s_tv, s_v)?,
        ),
        N::Var(x) => cores::refer(x.clone()),
        N::App(tgt, The(typ, val)) => {
            Core::app(read_back_neutral(ctx, tgt)?, read_back(ctx, typ, val)?)
        }
    })
}

pub fn val_in_ctx(ctx: &Ctx, e: &Core) -> Value {
    let env = &ctx_to_env(ctx);
    e.val_of(env)
}
