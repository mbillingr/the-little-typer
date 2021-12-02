from dataclasses import dataclass
import typing

from pypie import Ctx, Expr, Env, quote, expr


@dataclass
class DelayClos:
    env: Env
    expr: Expr


@dataclass
class Delay:
    val: typing.Union[DelayClos, "Value"]


@dataclass
class Quote:
    name: str


@dataclass
class Pair:
    """Placeholder because we don't have the more general "Sigma" pairs yet"""
    A: Delay
    D: Delay


@dataclass
class Cons:
    car: Delay
    cdr: Delay


Value = typing.Union[
    typing.Literal["UNIVERSE"],
    typing.Literal["ATOM"],
    Quote,
    Pair,
    Cons
]


def read_back_type(ctx: Ctx, typ_val: Value) -> Expr:
    match now(typ_val):
        case "ATOM": return "Atom"
        case Pair(A, D): return ["Pair", read_back_type(ctx, A), read_back_type(ctx, D)]
        case t: raise NotImplementedError(f"read_back_type({t})")


def read_back(ctx: Ctx, typ_val: Value, val: Value) -> Expr:
    match (now(typ_val), now(val)):
        case ("UNIVERSE", v): return read_back_type(ctx, v)
        case (Pair(A, D), pv):
            # placeholder until we have 'Sigma' pairs
            the_car = do_car(pv)
            the_cdr = do_cdr(pv)
            return ["cons", read_back(ctx, A, the_car), read_back(ctx, D, the_cdr)]
        case ("ATOM", Quote(name)):
            return quote(name)
        case (t, v):
            raise NotImplementedError(f"read_back({t}, {v})")


def later(env, expr):
    return Delay(DelayClos(env, expr))


def undelay(clos):
    return now(expr.value_of(clos.env, clos.expr))


def now(val):
    match val:
        case Delay(v):
            if isinstance(v, DelayClos):
                the_value = undelay(v)
                val.val = the_value
                return the_value
            return v
        case other:
            return other
