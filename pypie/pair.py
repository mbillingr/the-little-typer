from pypie.core import ParametricType, assert_type
from pypie.typevar import TypeVar


class Pair(ParametricType):
    def __init__(self, car_type, cdr_type):
        assert_type(car_type)
        assert_type(cdr_type)
        self.A = car_type
        self.D = cdr_type

    def check(self, obj):
        if isinstance(obj, TypeVar):
            obj_type = obj.typ
            if isinstance(obj_type, Pair):
                a = TypeVar(obj_type.A)
                d = TypeVar(obj_type.D)
                return self.A.check(a) and self.D.check(d)
        else:
            obj_type = type(obj)
            if obj_type == tuple and len(obj) == 2:
                return self.A.check(obj[0]) and self.D.check(obj[1])
        raise TypeError(Pair, obj_type)

    def __eq__(self, other):
        return isinstance(other, Pair) and self.A == other.A and self.D == other.D

    def compare(self, a, b):
        self.check(a)
        self.check(b)
        return self.A.compare(a[0], b[0]) and self.D.compare(a[1], b[1])


def cons(a, d):
    return (a, d)


def car(p):
    return p[0]


def cdr(p):
    return p[1]
