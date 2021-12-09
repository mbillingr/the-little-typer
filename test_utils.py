
from pypie import typechecker as tc
from pypie.expr import U, The
from pypie.typechecker import ConversionError


def same(t, a, b, ctx={}):
    try:
        tc.check_same(ctx, t, The(t, a), The(t, b))
    except ConversionError:
        return False
    return True


def same_type(a, b, ctx={}):
    return same(U(), a, b, ctx)


def is_a(t, v, ctx={}):
    return tc.is_a(ctx, t, v)


def is_type(t, ctx={}):
    renaming = {}
    return t.as_type(ctx, renaming)
