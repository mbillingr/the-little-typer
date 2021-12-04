from dataclasses import dataclass
import typing

from pypie import Ctx, Expr, Env, expr, Value


@dataclass
class DelayClos:
    env: Env
    expr: Expr


@dataclass
class Delay(Value):
    val: typing.Union[DelayClos, Value]

    def read_back_type(self, ctx: Ctx) -> Expr:
        return self.now().read_back_type(ctx)

    def read_back(self, typ_val: Value, ctx: Ctx) -> Expr:
        return self.now().read_back(typ_val, ctx)

    def now(self) -> Value:
        if isinstance(self.val, DelayClos):
            self.val = undelay(self.val)
        return self.val


@dataclass
class Universe(Value):
    def read_back_type(self, ctx: Ctx) -> Expr:
        return expr.U()

    def read_back(self, val: Value, ctx: Ctx) -> Expr:
        return val.read_back_type(ctx)


@dataclass
class Quote(Value):
    name: str


@dataclass
class Atom(Value):
    def read_back_type(self, ctx: Ctx) -> Expr:
        return expr.Atom()

    def read_back(self, val: Value, ctx: Ctx) -> Expr:
        val = val.now()
        if isinstance(val, Quote):
            return val.name
        else:
            return super().read_back(val, ctx)


@dataclass
class Pair(Value):
    """Placeholder because we don't have the more general "Sigma" pairs yet"""

    A: Delay
    D: Delay

    def read_back_type(self, ctx: Ctx) -> Expr:
        return expr.Pair(self.A.read_back_type(ctx), self.D.read_back_type(ctx))

    def read_back(self, val: Value, ctx: Ctx) -> Expr:
        pv = val.now()
        return expr.Cons(self.A.read_back(pv.car, ctx), self.D.read_back(pv.cdr, ctx))


@dataclass
class Cons(Value):
    car: Delay
    cdr: Delay


def later(env, expr):
    return Delay(DelayClos(env, expr))


def undelay(clos):
    return expr.value_of(clos.env, clos.expr).now()
