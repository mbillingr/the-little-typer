from dataclasses import dataclass

from pypie import value as v, Ctx, Env, Expr
from pypie.closure import FirstOrderClosure
from pypie.env import fresh_binder, bind_free


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

    def occurring_names(self):
        return self.typ.occurring_names() | self.exp.occurring_names()



@dataclass
class Ref(Expr):
    name: str

    def synth(self, ctx: Ctx, renaming):
        real_x = renaming.get(self.name, self.name)
        binder = ctx.get(real_x)
        t = binder.type.read_back_type(ctx)
        try:
            e = binder.type.read_back(binder.value, ctx)
        except AttributeError:
            e = real_x
        return The(t, e)

    def eval(self, env: Env) -> v.Value:
        return env[self.name]

    def occurring_names(self):
        return {self.name}


@dataclass
class U(Expr):
    def as_type(self, ctx: Ctx, renaming):
        return self

    def eval(self, env: Env) -> v.Value:
        return v.Universe()

    def occurring_names(self):
        return set()


@dataclass
class Nat(Expr):
    def as_type(self, ctx: Ctx, renaming):
        return self

    def synth(self, ctx: Ctx, renaming):
        return The(U(), self)

    def eval(self, env: Env) -> v.Value:
        return v.Nat()

    def occurring_names(self):
        return set()


@dataclass
class Add1(Expr):
    n: Expr

    def synth(self, ctx: Ctx, renaming):
        return The(Nat(), self)

    def eval(self, env: Env) -> v.Value:
        return v.Add1(v.later(env, self.n))

    def occurring_names(self):
        return self.n.occurring_names()


@dataclass
class Atom(Expr):
    def as_type(self, ctx: Ctx, renaming):
        return self

    def synth(self, ctx: Ctx, renaming):
        return The(U(), self)

    def eval(self, env: Env) -> v.Value:
        return v.Atom()

    def occurring_names(self):
        return set()


@dataclass(init=False)
class Fun(Expr):
    ParamTypes: [Expr]
    Body: Expr

    def __init__(self, *types):
        self.ParamTypes = types[:-1]
        self.Body = types[-1]

    def as_type(self, ctx: Ctx, renaming):
        if len(self.ParamTypes) == 1:
            return self.unary_as_type(ctx, renaming)
        else:
            raise NotImplementedError()

    def unary_as_type(self, ctx: Ctx, renaming):
        x = fresh_binder(ctx, self.Body, "x")
        A_out = self.ParamTypes[0].as_type(ctx, renaming)
        B_out = self.Body.as_type(bind_free(ctx, x, val_in_ctx(ctx, A_out)), renaming)
        return Pi(x, A_out, B_out)


@dataclass
class Pi(Expr):
    param_name: str
    ParamType: Expr
    Body: Expr

    def eval(self, env: Env) -> v.Value:
        Av = v.later(env, self.ParamType)
        return v.Pi(self.param_name, Av, FirstOrderClosure(env, self.param_name, self.Body))


@dataclass
class Lambda(Expr):
    param_names: [str]
    body: Expr

    def eval(self, env: Env) -> v.Value:
        assert len(self.param_names) == 1
        arg_name = self.param_names[0]
        return v.Lambda(arg_name, v.later(env, self.body))


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

    def occurring_names(self):
        return self.A.occurring_names() | self.D.occurring_names()


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
