import typing as _ty


class Expr:
    pass


Env = _ty.Dict[str, "Value"]
Ctx = _ty.Dict[str, 'Value']
