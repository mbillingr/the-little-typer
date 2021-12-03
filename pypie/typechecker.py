from pypie.alpha import is_alpha_equivalent
from pypie.env import Ctx, val_in_ctx
from pypie.value import Value
from pypie.expr import Atom, Car, Cdr, Cons, Expr, The, Pair, U
from pypie import value


class ConversionError(Exception):
    def __init__(self, tx, ax, bx):
        super().__init__(f"The expressions {ax} and {bx} are not the same {tx}")


class TypeMismatch(Exception):
    def __init__(self, given, expected):
        super().__init__(f"Expected {expected} but given {given}")


class TypeCheckError(Exception): pass


def is_type(ctx: Ctx, renaming, e: Expr) -> Expr:
    match e:
        case U() | Atom(): return e
        case Pair(A, D): return Pair(is_type(ctx, renaming, A), is_type(ctx, renaming, D))
        case _: raise NotImplementedError(f"is_type(..., {e})")


def same_type(ctx: Ctx, given: Value, expected: Value):
    given_e = given.read_back_type(ctx)
    expected_e = expected.read_back_type(ctx)
    if not is_alpha_equivalent(given_e, expected_e):
        raise TypeMismatch(given_e, expected_e)


def convert(ctx: Ctx, tv: Value, av: Value, bv: Value):
    """Check the form of judgment Γ ⊢ c ≡ c : c"""
    a = tv.read_back(av, ctx)
    b = tv.read_back(bv, ctx)
    if not is_alpha_equivalent(a, b):
        raise ConversionError(tv.read_back_type(ctx), a, b)
    return "ok"


def synth(ctx: Ctx, renaming, exp: Expr) -> Expr:
    match exp:
        case Atom():
            return The(U(), exp)
        case Pair(A, D):
            # placeholder until we have Sigma pairs
            return The(U(),
                       Pair(check(ctx, renaming, A, value.Universe()),
                            check(ctx, renaming, D, value.Universe())))
        case str(s):
            return The(Atom(), s)
        case The(t, e):
            t_out = is_type(ctx, renaming, t)
            e_out = check(ctx, renaming, e, val_in_ctx(ctx, t_out))
            return The(t_out, e_out)
        case _:
            raise NotImplementedError(f"synth({ctx}, {renaming}, {exp})")


def check(ctx: Ctx, renaming, exp: Expr, tv: Value) -> Expr:
    match exp:
        case Cons(a, d):
            match tv.now():
                case value.Pair(A, D):
                    return Cons(check(ctx, renaming, a, A), check(ctx, renaming, d, D))
                case non_sigma:
                    raise TypeCheckError(f"cons requires a Pair or Σ type, but was used as a {non_sigma.read_back_type(ctx)}")

    match synth(ctx, renaming, exp):
        case The(t_out, e_out):
            same_type(ctx, val_in_ctx(ctx, t_out), tv)
            return e_out


def check_same(ctx: Ctx, t: Expr, a: Expr, b: Expr):
    t_out = is_type(ctx, {}, t)
    t_val = val_in_ctx(ctx, t_out)
    a_out = check(ctx, {}, a, t_val)
    b_out = check(ctx, {}, b, t_val)
    a_val = val_in_ctx(ctx, a_out)
    b_val = val_in_ctx(ctx, b_out)
    return convert(ctx, t_val, a_val, b_val)


def is_a(ctx: Ctx, t: Expr, e: Expr):
    t_out = is_type(ctx, {}, t)
    t_val = val_in_ctx(ctx, t_out)
    try:
        _ = check(ctx, {}, e, t_val)
    except TypeMismatch:
        return False
    return True
