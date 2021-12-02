from pypie.alpha import is_alpha_equivalent
from pypie.env import Ctx, Env, val_in_ctx
from pypie.value import Value, read_back, read_back_type
from pypie.expr import Expr
from pypie import is_quote


class ConversionError(Exception):
    def __init__(self, tx, ax, bx):
        super().__init__(f"The expressions {ax} and {bx} are not the same {tx}")


class TypeMismatch(Exception):
    def __init__(self, given, expected):
        super().__init__(f"Expected {expected} but given {given}")


def is_type(ctx: Ctx, renaming, expr: Expr) -> Expr:
    match expr:
        case "U" | "Atom": return expr
        case _: raise NotImplementedError(f"is_type(..., {expr})")


def same_type(ctx: Ctx, given: Value, expected: Value):
    given_e = read_back_type(ctx, given)
    expected_e = read_back_type(ctx, expected)
    if not is_alpha_equivalent(given_e, expected_e):
        raise TypeMismatch(given_e, expected_e)


def convert(ctx: Ctx, tv: Value, av: Value, bv: Value):
    """Check the form of judgment Γ ⊢ c ≡ c : c"""
    a = read_back(ctx, tv, av)
    b = read_back(ctx, tv, bv)
    if not is_alpha_equivalent(a, b):
        raise ConversionError(read_back_type(ctx, tv), a, b)
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


def check(ctx: Ctx, renaming, expr: Expr, tv: Value):
    match expr:
        case _:
            pass

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
