from abc import ABC, abstractmethod
from dataclasses import dataclass
import typing

from pypie import Ctx, Expr, Env, quote, expr


class Value(ABC):
    def read_back_type(self, ctx: Ctx) -> Expr:
        raise NotImplementedError(f"{self.__class__.__name__}.read_back_type()")

    def read_back(self, val: "Value", ctx: Ctx) -> Expr:
        raise NotImplementedError(f"{self.__class__.__name__}.read_back({val})")


@dataclass
class DelayClos:
    env: Env
    expr: Expr


@dataclass
class Delay(Value):
    val: typing.Union[DelayClos, Value]

    def read_back_type(self, ctx: Ctx) -> Expr:
        return now(self).read_back_type(ctx)

    def read_back(self, typ_val: Value, ctx: Ctx) -> Expr:
        return now(self).read_back(typ_val, ctx)


@dataclass
class Universe(Value):
    def read_back_type(self, ctx: Ctx) -> Expr:
        return "U"

    def read_back(self, val: Value, ctx: Ctx) -> Expr:
        return val.read_back_type(ctx)


@dataclass
class Quote(Value):
    name: str


@dataclass
class Atom(Value):
    def read_back_type(self, ctx: Ctx) -> Expr:
        return "Atom"

    def read_back(self, val: Value, ctx: Ctx) -> Expr:
        val = now(val)
        if isinstance(val, Quote):
            return quote(val.name)
        else:
            return super().read_back(val, ctx)


@dataclass
class Pair(Value):
    """Placeholder because we don't have the more general "Sigma" pairs yet"""
    A: Delay
    D: Delay

    def read_back_type(self, ctx: Ctx) -> Expr:
        return ["Pair", self.A.read_back_type(ctx), self.D.read_back_type(ctx)]

    def read_back(self, val: Value, ctx: Ctx) -> Expr:
        pv = now(val)
        the_car = do_car(pv)
        the_cdr = do_cdr(pv)
        return ["cons", self.A.read_back(the_car, ctx), self.D.read_back(the_cdr, ctx)]


@dataclass
class Cons(Value):
    car: Delay
    cdr: Delay


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
