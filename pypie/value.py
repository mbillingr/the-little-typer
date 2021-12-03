from abc import ABC, abstractmethod
from dataclasses import dataclass
import typing

from pypie import Ctx, Expr, Env, quote, expr


class Value(ABC):
    def read_back_type(self, ctx: Ctx) -> Expr:
        raise NotImplementedError(f"{self}.read_back_type()")


@dataclass
class DelayClos:
    env: Env
    expr: Expr


@dataclass
class Delay(Value):
    val: typing.Union[DelayClos, "Value"]

    def read_back_type(self, ctx: Ctx) -> Expr:
        return now(self).read_back_type(ctx)


@dataclass
class Universe(Value):
    def read_back_type(self, ctx: Ctx) -> Expr:
        return "U"


@dataclass
class Quote(Value):
    name: str


@dataclass
class Atom(Value):
    def read_back_type(self, ctx: Ctx) -> Expr:
        return "Atom"


@dataclass
class Pair(Value):
    """Placeholder because we don't have the more general "Sigma" pairs yet"""
    A: Delay
    D: Delay

    def read_back_type(self, ctx: Ctx) -> Expr:
        return ["Pair", self.A.read_back_type(ctx), self.D.read_back_type(ctx)]


@dataclass
class Cons(Value):
    car: Delay
    cdr: Delay


def read_back(ctx: Ctx, typ_val: Value, val: Value) -> Expr:
    match (now(typ_val), now(val)):
        case (Universe(), v): return v.read_back_type(ctx)
        case (Pair(A, D), pv):
            # placeholder until we have 'Sigma' pairs
            the_car = do_car(pv)
            the_cdr = do_cdr(pv)
            return ["cons", read_back(ctx, A, the_car), read_back(ctx, D, the_cdr)]
        case (Atom(), Quote(name)):
            return quote(name)
        case (t, v):
            raise NotImplementedError(f"read_back({t}, {v})")


def do_car(pv):
    match now(pv):
        case Cons(a, _): return a
        case _:
            raise NotImplementedError(f"do_car({pv})")


def do_cdr(pv):
    match now(pv):
        case Cons(_, d): return d
        case _:
            raise NotImplementedError(f"do_car({pv})")


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
