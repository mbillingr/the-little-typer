use crate::basics::{ctx_to_env, Core, Ctx, Env, The, Value, ValueInterface, N};
use crate::types::cores;
use std::borrow::Cow;

pub fn now(v: &Value) -> Cow<Value> {
    match v.now() {
        None => Cow::Borrowed(v),
        Some(x) => Cow::Owned(x),
    }
}

pub fn val_of(env: &Env, e: &Core) -> Value {
    match e {
        Core::Fun(_) => panic!("Attempt to evaluate -> (should have been converted to Pi)"),
        Core::PiStar(_, _) => panic!("Attempt to evaluate Pi* (should have been converted to Pi)"),
        Core::LambdaStar(_, _) => panic!("Attempt to evaluate sugared lambda"),
        Core::AppStar(_, _) => panic!("Attempt to evaluate n-ary application (should have been converted to sequence of unary applications)"),
        Core::Object(obj) => obj.val_of(env),
    }
}

pub fn read_back_type(ctx: &Ctx, tv: &Value) -> Core {
    now(tv).read_back_type(ctx).unwrap()
}

pub fn read_back(ctx: &Ctx, tv: &Value, v: &Value) -> Core {
    let ntv = now(tv);
    let nv = now(v);

    if let Some((_, ne)) = nv.as_neutral() {
        read_back_neutral(ctx, ne)
    } else {
        ntv.read_back(ctx, tv, &nv).unwrap()
    }
}

pub fn read_back_neutral(ctx: &Ctx, ne: &N) -> Core {
    match ne {
        N::WhichNat(tgt, The(b_tv, b_v), The(s_tv, s_v)) => cores::which_nat(
            read_back_neutral(ctx, tgt),
            cores::the(read_back_type(ctx, b_tv), read_back(ctx, b_tv, b_v)),
            read_back(ctx, s_tv, s_v),
        ),
        N::Var(x) => cores::refer(x.clone()),
        N::App(tgt, The(typ, val)) => {
            Core::app(read_back_neutral(ctx, tgt), read_back(ctx, typ, val))
        }
    }
}

pub fn val_in_ctx(ctx: &Ctx, e: &Core) -> Value {
    val_of(&ctx_to_env(ctx), e)
}
