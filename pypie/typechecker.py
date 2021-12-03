from pypie.alpha import is_alpha_equivalent
from pypie.env import Ctx, val_in_ctx
from pypie.value import Value
from pypie.expr import Expr
from pypie import is_quote, value


class ConversionError(Exception):
    def __init__(self, tx, ax, bx):
        super().__init__(f"The expressions {ax} and {bx} are not the same {tx}")


class TypeMismatch(Exception):
    def __init__(self, given, expected):
        super().__init__(f"Expected {expected} but given {given}")


class TypeCheckError(Exception): pass


def is_type(ctx: Ctx, renaming, expr: Expr) -> Expr:
    match expr:
        case "U" | "Atom": return expr
        case ["Pair", A, D]: return ["Pair", is_type(ctx, renaming, A), is_type(ctx, renaming, D)]
        case _: raise NotImplementedError(f"is_type(..., {expr})")


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


def synth(ctx: Ctx, renaming, expr: Expr) -> Expr:
    match expr:
        case str(s) if is_quote(s):
            return ["the", "Atom", s]
        case ["the", t, e]:
            t_out = is_type(ctx, renaming, t)
            e_out = check(ctx, renaming, e, val_in_ctx(ctx, t_out))
            return ["the", t_out, e_out]
        case _:
            raise NotImplementedError(f"synth({ctx}, {renaming}, {expr})")


def check(ctx: Ctx, renaming, expr: Expr, tv: Value) -> Expr:
    match expr:
        case ["cons", a, d]:
            match tv.now():
                case value.Pair(A, D):
                    return ["cons", check(ctx, renaming, a, A), check(ctx, renaming, d, D)]
                case non_sigma:
                    raise TypeCheckError(f"cons requires a Pair or Σ type, but was used as a {non_sigma.read_back_type(ctx)}")

    match synth(ctx, renaming, expr):
        case ["the", t_out, e_out]:
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


def is_a(ctx:Ctx, t: Expr, e: Expr):
    t_out = is_type(ctx, {}, t)
    t_val = val_in_ctx(ctx, t_out)
    try:
        _ = check(ctx, {}, e, t_val)
    except TypeMismatch:
        return False
    return True
