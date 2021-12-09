from dataclasses import dataclass
import typing


class Closure:
    pass


@dataclass
class FirstOrderClosure(Closure):
    env: "Env"
    var: str
    expr: "Expr"

    def value_of(self, argval):
        return self.expr.eval(self.env | {self.var: argval})


@dataclass
class HigherOrderClosure(Closure):
    proc: typing.Callable[["Value"], "Value"]

    def value_of(self, argval):
        return self.proc(argval)
