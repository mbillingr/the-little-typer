from dataclasses import dataclass

from pypie import value as v, Ctx, Env, Expr


zero = 0


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

    def check(self, ctx: Ctx={}, renaming={}):
        t_out = as_type(ctx, renaming, self.typ)
        t_val = val_in_ctx(ctx, t_out)
        return check(ctx, renaming, self.exp, t_val)



@dataclass
class Ref(Expr):
    name: str

    def synth(self, ctx: Ctx, renaming):
        binder = ctx.get(self.name)
        t = binder.type.read_back_type(ctx)
        e = binder.type.read_back(binder.value, ctx)
        return The(t, e)

    def eval(self, env: Env) -> v.Value:
        return env[self.name]


@dataclass
class U(Expr):
    def as_type(self, ctx: Ctx, renaming):
        return self

    def eval(self, env: Env) -> v.Value:
        return v.Universe()


@dataclass
class Nat(Expr):
    def as_type(self, ctx: Ctx, renaming):
        return self

    def synth(self, ctx: Ctx, renaming):
        return The(U(), self)

    def eval(self, env: Env) -> v.Value:
        return v.Nat()


@dataclass
class Add1(Expr):
    n: Expr

    def synth(self, ctx: Ctx, renaming):
        return The(Nat(), self)

    def eval(self, env: Env) -> v.Value:
        return v.Add1(v.later(env, self.n))


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

    def synth(self, ctx: Ctx, renaming):
        p = synth(ctx, renaming, self.pair)
        match val_in_ctx(ctx, p.typ):
            case v.Pair(A, D):
                return The(A.read_back_type(ctx), p.exp.car)
        raise NotImplementedError(f"{self.__class__.__name__}.synth()")

    def eval(self, env: Env) -> v.Value:
        return v.do_car(v.later(env, self.pair))


@dataclass
class Cdr(Expr):
    pair: Expr

    def synth(self, ctx: Ctx, renaming):
        p = synth(ctx, renaming, self.pair)
        match val_in_ctx(ctx, p.typ):
            case v.Pair(A, D):
                return The(D.read_back_type(ctx), p.exp.cdr)
        raise NotImplementedError(f"{self.__class__.__name__}.synth()")

    def eval(self, env: Env) -> v.Value:
        return v.do_cdr(v.later(env, self.pair))


def value_of(env: Env, expr: Expr) -> v.Value:
    if isinstance(expr, str):
        return v.Quote(expr)
    if isinstance(expr, int):
        if expr == 0:
            return v.Zero()
        else:
            return v.Add1(v.later(env, expr - 1))
    return expr.eval(env)


def synth(ctx: Ctx, renaming, exp: Expr) -> The:
    if isinstance(exp, str):
        return The(Atom(), exp)
    if isinstance(exp, int):
        assert exp >= 0
        return The(Nat(), exp)
    return exp.synth(ctx, renaming)


def as_type(ctx: Ctx, renaming, exp: Expr) -> Expr:
    if isinstance(exp, str):
        raise NotATypeError(exp)
    return exp.as_type(ctx, renaming)


# it's ugly, but serves as a quick fix of cicrular imports
from pypie.typechecker import check, val_in_ctx, NotATypeError
