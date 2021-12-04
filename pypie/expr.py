from dataclasses import dataclass

import pypie.value

from pypie import value as v, Ctx, Env, Expr


@dataclass
class The(Expr):
    typ: Expr
    exp: Expr

    def synth(self, ctx: Ctx, renaming):
        t_out = is_type(ctx, renaming, self.typ)
        e_out = check(ctx, renaming, self.exp, val_in_ctx(ctx, t_out))
        return The(t_out, e_out)

    def eval(self, env: Env) -> v.Value:
        return value_of(env, self.exp)


@dataclass
class U(Expr):
    def eval(self, env: Env) -> v.Value:
        return v.Universe()


@dataclass
class Atom(Expr):
    def synth(self, ctx: Ctx, renaming):
        return The(U(), self)

    def eval(self, env: Env) -> v.Value:
        return v.Atom()


@dataclass
class Pair(Expr):
    A: Expr
    D: Expr

    def synth(self, ctx: Ctx, renaming):
        # placeholder until we have Sigma pairs
        return The(U(),
                   Pair(check(ctx, renaming, self.A, v.Universe()),
                        check(ctx, renaming, self.D, v.Universe())))

    def eval(self, env: Env) -> v.Value:
        return v.Pair(v.later(env, self.A), v.later(env, self.D))


@dataclass
class Cons(Expr):
    car: Expr
    cdr: Expr

    def eval(self, env: Env) -> v.Value:
        return v.Cons(v.later(env, self.car), v.later(env, self.cdr))


@dataclass
class Car(Expr):
    pair: Expr

    def eval(self, env: Env) -> v.Value:
        return v.do_car(v.later(env, self.pair))


@dataclass
class Cdr(Expr):
    pair: Expr

    def eval(self, env: Env) -> v.Value:
        return v.do_cdr(v.later(env, self.pair))


def value_of(env: Env, expr: Expr) -> v.Value:
    if isinstance(expr, str):
        return v.Quote(expr)
    return expr.eval(env)


# it's ugly, but serves as a quick fix of cicrular imports
from pypie.typechecker import check, is_type, val_in_ctx
