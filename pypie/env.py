from dataclasses import dataclass
from pypie import Binder, Ctx, Env, Expr, Value
from pypie.fresh import freshen
from pypie import neutral as neu
import pypie


def ctx_to_env(ctx: Ctx) -> Env:
    def convert_entry(x, v):
        return v.to_env_entry(x)
        #match v:
        #    case ("def", tv, v):
        #        return v
        #    case ("free", tv):
        #        return Neutral(tv, NVar(x))
        #    case ("claim", tv):
        #        return None

    maybe_converted_entries = ((x, convert_entry(x, v)) for x, v in ctx.items())
    only_non_none_entries = filter(lambda entry: entry[1], maybe_converted_entries)
    return dict(only_non_none_entries)


def val_in_ctx(ctx: Ctx, expr: Expr) -> Value:
    return pypie.expr.value_of(ctx_to_env(ctx), expr)


def fresh_binder(ctx: Ctx, expr: Expr, name: str) -> str:
    return freshen(set(ctx.keys()) | expr.occurring_names(), name)


@dataclass
class Free(Binder):
    type: Value

    def to_env_entry(self, name):
        return pypie.value.Neutral(self.type, neu.NVar(name))


def bind_free(ctx: Ctx, name: str, tv: "Value") -> Ctx:
    assert name not in ctx
    return ctx | {name: Free(tv)}
