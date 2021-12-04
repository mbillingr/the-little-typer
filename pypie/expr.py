from dataclasses import dataclass

from pypie import value as v, Ctx, Env, Expr


@dataclass
class The(Expr):
    typ: Expr
    exp: Expr

    def synth(self, ctx: Ctx, renaming):
        t_out = self.typ.as_type(ctx, renaming)
        e_out = check(ctx, renaming, self.exp, val_in_ctx(ctx, t_out))
        return The(t_out, e_out)

    def eval(self, env: Env) -> v.Value:
        return value_of(env, self.exp)


@dataclass
class U(Expr):
    def as_type(self, ctx: Ctx, renaming):
        return self

    def eval(self, env: Env) -> v.Value:
        return v.Universe()


@dataclass
class Atom(Expr):
    def as_type(self, ctx: Ctx, renaming):
        return self

    def synth(self, ctx: Ctx, renaming):
        return The(U(), self)

    def eval(self, env: Env) -> v.Value:
        return v.Atom()


@dataclass
class Pair(Expr):
    A: Expr
    D: Expr

    def as_type(self, ctx: Ctx, renaming):
        return Pair(self.A.as_type(ctx, renaming), self.D.as_type(ctx, renaming))

    def synth(self, ctx: Ctx, renaming):
        # placeholder until we have Sigma pairs
        return The(
            U(),
            Pair(
                check(ctx, renaming, self.A, v.Universe()),
                check(ctx, renaming, self.D, v.Universe()),
            ),
        )

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


def synth(ctx: Ctx, renaming, exp: Expr) -> Expr:
    if isinstance(exp, str):
        return The(Atom(), exp)
    return exp.synth(ctx, renaming)


def as_type(ctx: Ctx, renaming, exp: Expr) -> Expr:
    if isinstance(exp, str):
        raise NotATypeError(exp)
    return exp.as_type(ctx, renaming)


# it's ugly, but serves as a quick fix of cicrular imports
from pypie.typechecker import check, val_in_ctx, NotATypeError
