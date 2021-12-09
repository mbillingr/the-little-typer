from dataclasses import dataclass
import typing

from pypie import Ctx, Expr, Env, expr, Value, neutral as neu
from pypie.closure import Closure
from pypie.env import bind_free
from pypie.fresh import fresh


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
class Nat(Value):
    def read_back_type(self, ctx: Ctx) -> Expr:
        return expr.Nat()

    def read_back(self, val: Value, ctx: Ctx) -> Expr:
        val = val.now()
        match val:
            case Zero(): return 0
            case Add1(n): return 1 + self.read_back(n, ctx)
        raise NotImplementedError(val)


@dataclass
class Zero(Value):
    pass


@dataclass
class Add1(Value):
    n: Value


@dataclass
class Pi(Value):
    arg_name: str
    arg_type: Value
    result_type: Closure

    def read_back_type(self, ctx: Ctx) -> Expr:
        Ae = self.arg_type.read_back_type(ctx)
        x_hat = fresh(ctx, self.arg_name)
        extctx = bind_free(ctx, x_hat, self.arg_type)
        rtv = self.result_type.value_of(Neutral(self.arg_type, neu.NVar(x_hat)))
        Body = rtv.read_back_type(extctx)
        return expr.Pi(x_hat, Ae, Body)

    def read_back(self, val: Value, ctx: Ctx) -> Expr:
        pv = val.now()

        x_hat = fresh(ctx, self.arg_name)
        extctx = bind_free(ctx, x_hat, self.arg_type)
        rtv = self.result_type.value_of(Neutral(self.arg_type, neu.NVar(x_hat)))
        Body = rtv.read_back_type(extctx)

        body = Body.read_back(pv.body)

        return expr.Lambda([self.arg_name], body)


@dataclass
class Lambda(Value):
    arg_name: str
    body: Closure


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


@dataclass
class Neutral:
    type: Value
    neutral: neu.Neutral



def later(env, expr):
    return Delay(DelayClos(env, expr))


def undelay(clos):
    return expr.value_of(clos.env, clos.expr).now()
