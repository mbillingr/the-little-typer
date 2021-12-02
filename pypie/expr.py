from pypie import value as v, Expr, Env
from pypie import is_quote, unquote


def value_of(env: Env, expr: Expr) -> v.Value:
    match expr:
        case ["the", typ, exp]:
            return value_of(env, exp)
        case "U":
            return "UNIVERSE"
        case "Atom":
            return "ATOM"
        case ["Pair", A, D]:  # placeholder until we have 'Sigma' pairs
            return v.Pair(v.later(env, A), v.later(env, D))
        case ["cons", a, d]:
            return v.Cons(v.later(env, a), v.later(env, d))
        case ["car", p]:
            return v.do_car(v.later(env, p))
        case ["cdr", p]:
            return v.do_cdr(v.later(env, p))
        case str(s) if is_quote(s):
            return v.Quote(unquote(s))
        case x:
            raise SyntaxError(f"No evaluator for {x}")
