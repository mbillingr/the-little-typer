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

    @property
    def minus1(self):
        return Nat


def add1(n):
    Nat.check(n)
    return n + 1


def which_nat(target, base, step):
    if target == zero:
        return base
    else:
        return step(target - 1)


def plus(a, b):
    Nat.check(a)
    Nat.check(b)
    return _plus(a, b)


def _plus(a, b):
    if a == zero:
        return b
    if b == zero:
        return a
    if isinstance(a, TypeVar):
        assert a.typ == Nat, f"not a nat {a}: {a.typ}"
        return TypeVar(Nat)
    return which_nat(b, a, lambda b_minus_one: _plus(add1(a), b_minus_one))
