use crate::basics::{ctx_to_env, is_var_name, Core, Ctx, Env, Norm, Value, ValueInterface, N};
use crate::types::values;
use std::borrow::Cow;

fn later(env: Env, exp: Core) -> Value {
    values::later(env, exp)
}

pub fn now(v: &Value) -> Cow<Value> {
    v.now(v)
}

pub fn val_of(env: &Env, e: &Core) -> Value {
    match e {
        Core::Nat => values::nat(),
        Core::Zero => values::zero(),
        Core::Add1(n) => values::add1(later(env.clone(), (**n).clone())),
        Core::Fun(_) => panic!("Attempt to evaluate -> (should have been converted to Pi)"),
        Core::PiStar(_, _) => panic!("Attempt to evaluate Pi* (should have been converted to Pi)"),
        Core::LambdaStar(_, _) => panic!("Attempt to evaluate sugared lambda"),
        Core::AppStar(_, _) => panic!("Attempt to evaluate n-ary application (should have been converted to sequence of unary applications)"),
        Core::Symbol(x) if is_var_name(x) => env.var_val(x).unwrap(),
        Core::Symbol(x) => panic!("No evaluator for {}", x.name()),
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
        N::Var(x) => Core::Symbol(x.clone()),
        N::App(tgt, Norm { typ, val }) => {
            Core::app(read_back_neutral(ctx, tgt), read_back(ctx, typ, val))
        }
    }
}

pub fn val_in_ctx(ctx: &Ctx, e: &Core) -> Value {
    val_of(&ctx_to_env(ctx), e)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{cores, values};

    #[test]
    fn test_delayed() {
        let env = Env::new();
        let delayed_value = later(env, cores::universe());
        assert_eq!(*now(&delayed_value), values::universe());
    }
}
