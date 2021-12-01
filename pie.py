import re
from contextlib import contextmanager


def evaluate(expr, env):
    match expr:
        case _ if is_constructor(expr):
            return expr
        case ["car", p]:
            p = evaluate(p, env)
            if isinstance(p, str):
                return expr
            assert p[0] == "cons"
            return p[1]
        case ["cdr", p]:
            p = evaluate(p, env)
            if isinstance(p, str):
                return expr
            assert p[0] == "cons"
            return p[2]
        case ["+", a, "zero"]: return evaluate(a, env)
        case ["+", "zero", b]: return evaluate(b, env)
        case ["+", ["add-1", a], b]: return evaluate(["+", a, ["add-1", b]], env)
        case ["+", a, b] if isinstance(a, int) and isinstance(b, int): return a + b
        case ["+", _, _]: return expr
        case ["which-Nat", target, base, step]:
            target = evaluate(target, env)
            assert is_nat(target)
            if target == 0:
                return evaluate(base, env)
            else:
                return evaluate([step, target - 1], env)
        case [func, *args]:
            func = evaluate(func, env)
            args = [evaluate(a, env) for a in args]
            if isinstance(func, str):
                return [func, *args]
            if callable(func):
                # primitive application
                return func(*args)
            assert func[0] == "lambda"
            mapping = {p: a for p, a in zip(func[1], args)}
            return evaluate(substitute(func[2], mapping), env)
        case str() as name if name.startswith('_'):
            return name
        case str() as name:
            return env[name][1]
        case int() as x:
            return x
        case _:
            raise NotImplementedError(f"evaluate {expr}")


def is_constructor(expr):
    match expr:
        case "U": return True

        case "Atom": return True
        case ["quote", _]: return True

        case ["->", _, *_]: return True
        case ["lambda", _, _]: return True

        case ["Pair", _, _]: return True
        case ["cons", _, _]: return True

        case "Nat": return True
        case ["add-1", _]: return True

        case _: return False


def is_type(expr, env=None):
    if env is not None:
        expr = normalize(expr, env)
    match expr:
        case "U": return True
        case "Atom": return True
        case "Nat": return True
        case ["->", *A, R]: return is_type(R, env) and all(is_type(a, env) for a in A)
        case ["Pair", A, D]: return is_type(A, env) and is_type(D, env)
        case _: return False


def normalize(expr, env):
    value = evaluate(expr, env)
    match value:
        case ["->", *T]:
            return ["->", *[normalize(t, env) for t in T]]
        case ["lambda", params, body]:
            with scope():
                mapping = {p: neutral() for p in params}
                body = normalize(substitute(body, mapping), env)
            return ["lambda", list(mapping.values()), body]
        case ["Pair", A, D]: return ["Pair", normalize(A, env), normalize(D, env)]
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
    expr = evaluate(expr, env)
    assert_is_a(expr, claimed_type, env)
    env[name] = (claimed_type, expr)


def is_a(expr, claim, env=None):
    if env is not None:
        expr = normalize(expr, env)
        claim = normalize(claim, env)
    match (expr, claim):
        case (_, "U"):
            return is_type(expr, env)
        case (["quote", str()], "Atom"):
            return True
        case (["lambda", [*params], body], ["->", *Params, Ret]):
            return is_a(body, Ret, {a: (A, a) for a, A in zip(params, Params)})
        case (["cons", a, d], ["Pair", A, D]):
            return is_a(a, A, env) and is_a(d, D, env)
        case (["car", p], _):
            key, A, D = signature(p, env)
            assert key == "Pair"
            return A == claim
        case (["cdr", p], _):
            key, A, D = signature(p, env)
            assert key == "Pair"
            return D == claim
        case (["+", a, b], _):
            return is_a(a, "Nat", env) and is_a(b, "Nat", env) and claim == "Nat"
        case ([func, *args], _):
            key, *ptypes, rettype = signature(func, env)
            assert key == "->"
            assert len(args) == len(ptypes)
            return rettype == claim and all(is_a(a, p, env) for a, p in zip(args, ptypes))
        case (int(), Nat):
            return expr >= 0
        case (str(), _) if expr.startswith('_'):
            return True  # this is not quite correct... a variable can't be different types at different times
        case _:
            return False


def assert_is_a(expr, claim, env=None):
    if not is_a(expr, claim, env):
        raise TypeError(f"{expr} is not a {claim}")


def signature(func, env):
    match func:
        case ["car", p]:
            return signature(p)[1]
        case ["+", a, b]:
            raise NotImplementedError()
        case str():
            return signature(env[func][0], env)
        case _:
            return func


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
                try:
                    del mapping[p]
                except KeyError:
                    pass
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


def is_nat(obj):
    return isinstance(obj, int) and obj >= 0


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
            try:
                return int(s)
            except ValueError:
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


global_env = {
    "zero": ("Nat", 0),
}


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

assert normalize(parse("(which-Nat zero 'naught (lambda (n) 'more))"), global_env) == parse("'naught")
assert normalize(parse("(which-Nat 4 'naught (lambda (n) 'more))"), global_env) == parse("'more")

assert is_a(parse("(cons Atom Atom)"), parse("(Pair U U)"), global_env)

claim("Pear", parse("U"), global_env)
define("Pear", parse("(Pair Nat Nat)"), global_env)
assert is_a(parse("(cons 3 5)"), parse("Pear"), global_env)

claim('Pear-maker', parse("U"), global_env)
define("Pear-maker", parse("(-> Nat Nat Pear)"), global_env)

claim("elim-Pear", parse("(-> Pear Pear-maker Pear)"), global_env)
define("elim-Pear", parse("(lambda (pear maker) (maker (car pear) (cdr pear)))"), global_env)

assert is_the_same(parse("Pear"), parse("(elim-Pear (cons 3 17) (lambda (a d) (cons d a)))"), parse("(cons 17 3)"), global_env)

claim("pearwise+", parse("(-> Pear Pear Pear)"), global_env)
define("pearwise+", parse("(lambda (anjou bosc)"
                          "  (elim-Pear anjou"
                          "    (lambda (a1 d1)"
                          "      (elim-Pear bosc"
                          "        (lambda (a2 d2)"
                          "          (cons (+ a1 a2) (+ d1 d2)))))))"), global_env)

assert is_the_same(parse("Pear"), parse("(pearwise+ (cons 3 8) (cons 7 6))"), parse("(cons 10 14)"), global_env)
