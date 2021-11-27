"""Some stuff I tried before switching to the book-driven approach"""

class Π:
    def __init__(self, argtypes, body):
        self.argtypes = argtypes
        self.body = body

    def __str__(self):
        vars = [f"_{i}" for i in range(len(self.argtypes))]
        args = " ".join(f"({v} {t})" for v, t in zip(vars, self.argtypes))
        res = self.body(*vars)
        return f"(Π ({args}) {res})"

    def check(self, obj):
        vars = [TypeVar(t) for t in self.argtypes]
        res_type = self(*vars)
        res = obj(*vars)
        return res_type.check(res)

    def __call__(self, *args):
        return self.body(*args)





class Int(Type):
    @staticmethod
    def check(obj):
        if isinstance(obj, TypeVar):
            obj_type = obj.typ
        else:
            obj_type = type(obj)

        if obj_type != int and obj_type != Int:
            raise TypeError(int, obj_type)

        return "ok"


Identity = Π((type,), lambda T: Fun(T, T))


def ident(T):
    return lambda x: x


def wrong_ident(T):
    return lambda x: 0


def assert_raises(exception, call, *args, **kwargs):
    try:
        call(*args, **kwargs)
    except exception:
        return "OK"
    else:
        raise AssertionError(f"did not raise {exception.__name__}")


if __name__ == "__main__":
    print(Pair(Int, Int).check(cons(1, 2)))
    print(Pair(Int, Int).check(TypeVar(Pair(Int, Int))))

    print(Identity)
    print(Identity.check(ident))
    print(Identity.check(lambda T: lambda x: x))
    print(Identity(Int).check(lambda x: 42))
    assert_raises(TypeError, Identity.check, wrong_ident)
