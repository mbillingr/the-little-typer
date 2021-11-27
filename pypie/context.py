from pypie.core import assert_type, typecheck


class Context:
    def __init__(self):
        self.claims = {}
        self.definitions = {}

    def claim(self, name, typ):
        assert name not in self.claims
        assert_type(typ)
        self.claims[name] = typ

    def define(self, name, expr):
        assert name in self.claims
        assert name not in self.definitions
        typecheck(expr, self.claims[name])
        self.definitions[name] = expr

    def __getattr__(self, item):
        return self.definitions[item]
