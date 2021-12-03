from pypie import Ctx, Env, Expr, Value
from pypie.expr import value_of


def ctx_to_env(ctx: Ctx) -> Env:
    def convert_entry(x, v):
        match v:
            case ("def", tv, v):
                return v
            case ("free", tv):
                return Neutral(tv, NVar(x))
            case ("claim", tv):
                return None

    maybe_converted_entries = ((x, convert_entry(x, v)) for x, v in ctx.items())
    only_non_none_entries = filter(lambda _, v: v, maybe_converted_entries)
    return dict(only_non_none_entries)


def val_in_ctx(ctx: Ctx, expr: Expr) -> Value:
    return value_of(ctx_to_env(ctx), expr)
