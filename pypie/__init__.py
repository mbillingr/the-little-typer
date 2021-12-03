import typing as _ty


class Expr:
    def eval(self, env: "Env") -> "Value":
        raise NotImplementedError(f"{self.__class__.__name__}.eval()")


Env = _ty.Dict[str, "Value"]
Ctx = _ty.Dict[str, "Value"]
