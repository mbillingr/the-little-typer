import re
from contextlib import contextmanager


def evaluate(expr, env):
    match expr:
        case _ if is_constructor(expr):
            return expr
        case ["car", p]:
            p = evaluate(p, env)
            assert p[0] == "cons"
            return p[1]
        case ["cdr", p]:
            p = evaluate(p, env)
            assert p[0] == "cons"
            return p[2]
        case ["+", a, "zero"]: return evaluate(a, env)
        case ["+", "zero", b]: return evaluate(b, env)
        case ["+", ["add-1", a], b]: return evaluate(["+", a, ["add-1", b]], env)
        case [func, *args]:
            func = evaluate(func, env)
            args = [evaluate(a, env) for a in args]
            assert func[0] == "lambda"
            mapping = {p: a for p, a in zip(func[1], args)}
            return substitute(func[2], mapping)
        case str() as name if name.startswith('_'):
            return name
        case str() as name:
            return env[name][1]
        case _:
            raise NotImplementedError(f"evaluate {expr}")


def is_constructor(expr):
    match expr:
        case "Atom": return True
        case ["quote", _]: return True

        case ["->", _, *_]: return True
        case ["lambda", _, _]: return True

        case ["Pair", _, _]: return True
        case ["cons", _, _]: return True

        case "zero": return True
        case ["add-1", _]: return True

        case _: return False


def is_type(expr):
    match expr:
        case "Atom": return True
        case ["->", *A, R]: return is_type(R) and all(is_type(a) for a in A)
        case ["Pair", A, D]: return is_type(A) and is_type(D)
        case _: return False


def normalize(expr, env):
    match evaluate(expr, env):
        case ["lambda", params, body]:
            with scope():
                mapping = {p: neutral() for p in params}
                body = normalize(substitute(body, mapping), env)
            return ["lambda", list(mapping.values()), body]
        case ["cons", ["car", p1], ["cdr", p2]] if p1 is p2: return p1
        case ["cons", a, d]: return ["cons", normalize(a, env), normalize(d, env)]
        case ["add-1", n]: return ["add-1", normalize(n, env)]
        case value: return value


def claim(name, expr, env):
    assert name not in env
    expr = normalize(expr, env)
    assert is_type(expr)
    env[name] = (expr, None)


def define(name, expr, env):
    assert name in env
    claimed_type, expr_place = env[name]
    assert claimed_type is not None
    assert expr_place is None
    expr = normalize(expr, env)
    assert_is_a(expr, claimed_type)
    env[name] = (claimed_type, expr)


def is_a(expr, claim, env=None):
    if env is not None:
        expr = normalize(expr, env)
        claim = normalize(claim, env)
    match (expr, claim):
        case (["quote", str()], "Atom"):
            return True
        case (["lambda", [*args], body], ["->", *Args, Ret]):
            return is_a(body, Ret) and all(is_a(a, A) for a, A in zip(args, Args))
        case (["cons", a, d], ["Pair", A, D]):
            return is_a(a, A) and is_a(d, D)
        case (str(), _) if expr.startswith('_'):
            return True  # this is not quite correct... a variable can't be different types at different times
        case _:
            return False


def assert_is_a(expr, claim, env=None):
    if not is_a(expr, claim, env):
        raise TypeError(f"{expr} is not a {claim}")


def is_the_same(claim, a, b, env=None):
    if env is not None:
        claim = normalize(claim, env)
        a = normalize(a, env)
        b = normalize(b, env)
    assert_is_a(a, claim)
    assert_is_a(b, claim)
    return a == b


def substitute(expr, mapping):
    if not mapping:
        return expr
    match expr:
        case ["lambda", params, body]:
            mapping = mapping.copy()
            for p in params:
                del mapping[p]
            return ["lambda", params, substitute(body, mapping)]
        case [*items]:
            return list(substitute(x, mapping) for x in items)
        case _ if expr in mapping:
            return mapping[expr]
        case _:
            return expr


def stringify(expr):
    match expr:
        case ["quote", x]:
            return "'" + stringify(x)
        case [*items]:
            return "(" + " ".join(stringify(x) for x in items) + ")"
        case _:
            return str(expr)


_n_vars = 0

@contextmanager
def scope():
    global _n_vars
    saved = _n_vars
    yield
    _n_vars = saved


def neutral():
    global _n_vars
    name = f"_{_n_vars}"
    _n_vars += 1
    return name


def parse(inp):
    if isinstance(inp, str):
        inp = [x for x in re.split("\s+|([)(])", inp)[::-1] if x]
    match inp.pop():
        case '(':
            items = []
            while True:
                if inp[-1] == ')':
                    inp.pop()
                    return items
                items.append(parse(inp))
        case ')':
            raise SyntaxError("Unmatched )")
        case s if s.startswith("'"):
            return ["quote", s[1:]]
        case s:
            return s


def cons(a, d):
    return ["cons", a, d]


def car(p):
    return ["car", p]


def cdr(p):
    return ["cdr", p]


def zero():
    return ["zero"]


def add1(n):
    return ["add-1", n]


global_env = {}


print(stringify(evaluate(parse("((lambda (flavor) (cons flavor 'lentils)) 'garlic)"), global_env)))

print(stringify(evaluate(parse("((lambda (root) (cons root (cons (+ 1 2) root))) 'potato)"), global_env)))

print(stringify(evaluate(parse("((lambda (root) (cons root (lambda (root) root))) 'carrot)"), global_env)))

print(stringify(normalize(parse("(add-1 (+ (add-1 zero) (add-1 zero)))"), global_env)))

print(stringify(evaluate(parse("(lambda (x) (car (cons x x)))"), global_env)))
print(stringify(normalize(parse("(lambda (x) (car (cons x x)))"), global_env)))

claim("vegetables", parse("(Pair Atom Atom)"), global_env)
define("vegetables", parse("(cons 'celery 'carrot)"), global_env)

print(stringify(evaluate(parse("vegetables"), global_env)))

assert is_the_same(parse("(Pair Atom Atom)"),
                   evaluate(parse("vegetables"), global_env),
                   evaluate(parse("(cons (car vegetables) (cdr vegetables))"), global_env),
                   global_env)

assert is_the_same(parse("(-> (Pair Atom Atom) (Pair Atom Atom))"),
                   evaluate(parse("(lambda (p) p)"), global_env),
                   evaluate(parse("(lambda (p) (cons (car p) (cdr p)))"), global_env),
                   global_env)
