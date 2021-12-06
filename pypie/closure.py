from dataclasses import dataclass
import typing


class Closure:
    pass


@dataclass
class FirstOrderClosure(Closure):
    env: "Env"
    var: str
    expr: "Expr"


@dataclass
class HigherOrderClosure(Closure):
    proc: typing.Callable[["Value"], "Value"]
