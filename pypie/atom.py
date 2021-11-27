from pypie import Type
from pypie.typevar import TypeVar


class Atom(Type):
    @staticmethod
    def check(obj):
        if isinstance(obj, TypeVar):
            obj_type = obj.typ
        else:
            obj_type = type(obj)

        if obj_type != str and obj_type != Atom:
            raise TypeError(str, obj_type)

        return "ok"

    @classmethod
    def compare(cls, a, b):
        cls.check(a)
        cls.check(b)
        return a == b