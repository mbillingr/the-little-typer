from pypie.core import assert_type, ParametricType
from pypie.typevar import TypeVar


class Fun(ParametricType):
    def __init__(self, *types):
        for t in types:
            assert_type(t)
        self.Args = types[:-1]
        self.Ret = types[-1]

    def __str__(self):
        return f"(-> {' '.join(self.Args)} {self.Ret})"

    def check(self, obj):
        variables = [TypeVar(a) for a in self.Args]
        res = obj(*variables)
        return self.Ret.check(res)

    def __eq__(self, other):
        return (
            isinstance(other, Fun) and self.Args == other.Args and self.Ret == other.Ret
        )

    def compare(self, a, b):
        self.check(a)
        self.check(b)
        variables = [TypeVar(a) for a in self.Args]
        res_a = a(*variables)
        res_b = b(*variables)
        return self.Ret.compare(res_a, res_b)
