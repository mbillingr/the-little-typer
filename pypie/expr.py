from dataclasses import dataclass

from pypie import value as v, Expr, Env


@dataclass
class The(Expr):
    typ: Expr
    exp: Expr


@dataclass
class U(Expr):
    pass


@dataclass
class Atom(Expr):
    pass


@dataclass
class Pair(Expr):
    A: Expr
    D: Expr


@dataclass
class Cons(Expr):
    car: Expr
    cdr: Expr


@dataclass
class Car(Expr):
    pair: Expr


@dataclass
class Cdr(Expr):
    pair: Expr


def value_of(env: Env, expr: Expr) -> v.Value:
    match expr:
        case The(typ, exp):
            return value_of(env, exp)
        case U():
            return v.Universe()
        case Atom():
            return v.Atom()
        case Pair(A, D):  # placeholder until we have 'Sigma' pairs
            return v.Pair(v.later(env, A), v.later(env, D))
        case Cons(a, d):
            return v.Cons(v.later(env, a), v.later(env, d))
        case Car(p):
            return v.do_car(v.later(env, p))
        case Cdr(p):
            return v.do_cdr(v.later(env, p))
        case str(s):
            return v.Quote(s)
        case x:
            raise SyntaxError(f"No evaluator for {x}")
