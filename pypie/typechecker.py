from pypie.alpha import is_alpha_equivalent
from pypie.env import Ctx, val_in_ctx
from pypie.value import Value
from pypie.expr import Cons, Expr, The, synth
from pypie import value


class ConversionError(Exception):
    def __init__(self, tx, ax, bx):
        super().__init__(f"The expressions {ax} and {bx} are not the same {tx}")


class TypeMismatch(Exception):
    def __init__(self, given, expected):
        super().__init__(f"Expected {expected} but given {given}")


class TypeCheckError(Exception): pass


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

    raise NotImplementedError(f"check({exp}, {tv})")


def check_same(ctx: Ctx, t: Expr, a: Expr, b: Expr):
    renaming = {}
    t_out = t.as_type(ctx, renaming)
    t_val = val_in_ctx(ctx, t_out)
    a_out = check(ctx, {}, a, t_val)
    b_out = check(ctx, {}, b, t_val)
    a_val = val_in_ctx(ctx, a_out)
    b_val = val_in_ctx(ctx, b_out)
    return convert(ctx, t_val, a_val, b_val)


def is_a(ctx: Ctx, t: Expr, e: Expr):
    renaming = {}
    t_out = t.as_type(ctx, renaming)
    t_val = val_in_ctx(ctx, t_out)
    try:
        _ = check(ctx, {}, e, t_val)
    except TypeMismatch:
        return False
    return True
