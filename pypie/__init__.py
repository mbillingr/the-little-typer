from abc import ABC
import typing as _ty


Env = _ty.Dict[str, "Value"]
Ctx = _ty.Dict[str, "Value"]


class Expr:
    def eval(self, env: Env) -> "Value":
        raise NotImplementedError(f"{self.__class__.__name__}.eval()")


class Value(ABC):
    """Values are the results of evaluating expressions."""

    def read_back_type(self, ctx: Ctx) -> Expr:
        """Convert a type-value back to an expression."""
        raise NotImplementedError(f"{self.__class__.__name__}.read_back_type()")

    def read_back(self, val: "Value", ctx: Ctx) -> Expr:
        """Assuming self is a type-value, convert a value of that type back to an expression."""
        raise NotImplementedError(f"{self.__class__.__name__}.read_back({val})")

    def now(self) -> "Value":
        """Return the actual value, computing it if necessary."""
        return self
