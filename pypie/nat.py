from pypie.core import Type, TypeMismatch
from pypie.typevar import TypeVar


zero = 0


class Nat(Type):
    @staticmethod
    def check(obj):
        if isinstance(obj, TypeVar):
            obj_type = obj.typ
        else:
            obj_type = type(obj)

        if obj_type == int and obj >= 0 or obj_type == Nat:
            return "ok"

        raise TypeMismatch(f"not a Nat: {obj}")

    @classmethod
    def compare(cls, a, b):
        cls.check(a)
        cls.check(b)
        return a == b


def add1(n):
    Nat.check(n)
    return n + 1


def which_nat(target, base, step):
    if target == zero:
        return base
    else:
        return step(target - 1)
