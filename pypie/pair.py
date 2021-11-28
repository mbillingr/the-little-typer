from pypie.core import assert_type, ParametricType, TypeMismatch
from pypie.typevar import TypeVar


class Pair(ParametricType):
    def __init__(self, car_type, cdr_type):
        assert_type(car_type)
        assert_type(cdr_type)
        self.A = car_type
        self.D = cdr_type

    def check(self, obj):
        if isinstance(obj, TypeVar) or isinstance(obj, tuple) and len(obj) == 2:
            return self.A.check(car(obj)) and self.D.check(cdr(obj))
        raise TypeMismatch(f"not a Pair: {obj}")

    def __eq__(self, other):
        return isinstance(other, Pair) and self.A == other.A and self.D == other.D

    def compare(self, a, b):
        self.check(a)
        self.check(b)
        return self.A.compare(car(a), car(b)) and self.D.compare(cdr(a), cdr(b))


def cons(a, d):
    return (a, d)


def car(p):
    if isinstance(p, TypeVar):
        assert isinstance(p.typ, Pair)
        return p.request("A")
    return p[0]


def cdr(p):
    if isinstance(p, TypeVar):
        assert isinstance(p.typ, Pair)
        return p.request("D")
    return p[1]
