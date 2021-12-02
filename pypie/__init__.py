import typing as _ty


class Expr:
    pass


Env = _ty.Dict[str, "Value"]
Ctx = _ty.Dict[str, 'Value']


def quote(s):
    return f"'{s}"


def is_quote(s):
    return isinstance(s, str) and s.startswith("'")


def unquote(s):
    return s[1:]