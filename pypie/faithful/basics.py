from abc import ABC
from dataclasses import dataclass
import typing as ty

from fresh import freshen


PieKeyword: ty.TypeAlias = ty.Union[
    ty.Literal["U"],
    ty.Literal["Nat"],
    ty.Literal["zero"],
    ty.Literal["add1"],
    ty.Literal["which-Nat"],
    ty.Literal["iter-Nat"],
    ty.Literal["rec-Nat"],
    ty.Literal["ind-Nat"],
    ty.Literal["->"],
    ty.Literal["→"],
    ty.Literal["Π"],
    ty.Literal["λ"],
    ty.Literal["Pi"],
    ty.Literal["∏"],
    ty.Literal["lambda"],
    ty.Literal["quote"],
    ty.Literal["Atom"],
    ty.Literal["car"],
    ty.Literal["cdr"],
    ty.Literal["cons"],
    ty.Literal["Σ"],
    ty.Literal["Sigma"],
    ty.Literal["Pair"],
    ty.Literal["Trivial"],
    ty.Literal["sole"],
    ty.Literal["List"],
    ty.Literal["::"],
    ty.Literal["nil"],
    ty.Literal["rec-List"],
    ty.Literal["ind-List"],
    ty.Literal["Absurd"],
    ty.Literal["ind-Absurd"],
    ty.Literal["="],
    ty.Literal["same"],
    ty.Literal["replace"],
    ty.Literal["trans"],
    ty.Literal["cong"],
    ty.Literal["symm"],
    ty.Literal["ind-="],
    ty.Literal["Vec"],
    ty.Literal["vecnil"],
    ty.Literal["vec::"],
    ty.Literal["head"],
    ty.Literal["tail"],
    ty.Literal["ind-Vec"],
    ty.Literal["Either"],
    ty.Literal["left"],
    ty.Literal["right"],
    ty.Literal["ind-Either"],
    ty.Literal["TODO"],
    ty.Literal["the"],
]


@dataclass
class Src:
    loc: "Loc"
    stx: "SrcStx"


def src_stx(s: Src) -> "SrtStx":
    return s.stx


# Source Expressions
SrcStx: ty.TypeAlias = ty.Union[
    ty.Tuple[ty.Literal["the"], Src, Src],
    ty.Literal["U"],
    ty.Literal["Nat"],
    ty.Literal["zero"],
    str,
    ty.Literal["Atom"],
    ty.Tuple[ty.Literal["quote"], str],
    ty.Tuple[ty.Literal["add1"], Src],
    ty.Tuple[ty.Literal["which-Nat"], Src, Src, Src],
    ty.Tuple[ty.Literal["iter-Nat"], Src, Src, Src],
    ty.Tuple[ty.Literal["rec-Nat"], Src, Src, Src],
    ty.Tuple[ty.Literal["ind-Nat"], Src, Src, Src, Src],
    # TODO
]


Core: ty.TypeAlias = ty.Union[
    ty.Tuple[ty.Literal["the"], "Core", "Core"],
    ty.Literal["U"],
    ty.Literal["Nat"],
    ty.Literal["zero"],
    str,
    ty.Tuple[ty.Literal["add1"], "Core"],
    ty.Tuple[
        ty.Literal["which-Nat"],
        "Core",
        ty.Tuple[ty.Literal["the"], "Core", "Core"],
        "Core",
    ],
    # TODO
]


@dataclass
class DELAY_CLOS:
    env: "Env"
    expr: Core


@dataclass
class DELAY:
    val: ty.Union[DELAY_CLOS, "Value"]


@dataclass
class QUOTE:
    name: str


@dataclass
class ADD1:
    smaller: "Value"


@dataclass
class PI:
    arg_name: str
    arg_type: "Value"
    result_type: "Closure"


@dataclass
class LAM:
    arg_name: str
    body: "Closure"


@dataclass
class SIGMA:
    car_name: str
    car_type: "Value"
    cdr_type: "Closure"


@dataclass
class CONS:
    car: "Value"
    cdr: "Value"


@dataclass
class LIST_CONS:
    head: "Value"
    tail: "Value"


@dataclass
class LIST:
    entry_type: "Value"


@dataclass
class EQUAL:
    type_: "Value"
    from_: "Value"
    to_: "Value"


@dataclass
class SAME:
    value: "Value"


@dataclass
class VEC:
    entry_type: "Value"
    length: "Value"


@dataclass
class VEC_CONS:
    head: "Value"
    tail: "Value"


@dataclass
class EITHER:
    left_type: "Value"
    right_type: "Value"


@dataclass
class LEFT:
    value: "Value"


@dataclass
class RIGHT:
    value: "Value"


@dataclass
class NEU:
    type: "Value"
    neutral: "Neutral"


Value: ty.TypeAlias = ty.Union[
    ty.Literal["UNIVERSE"],
    ty.Literal["NAT"],
    ty.Literal["ZERO"],
    ADD1,
    QUOTE,
    ty.Literal["ATOM"],
    PI,
    LAM,
    SIGMA,
    CONS,
    ty.Literal["TRIVIAL"],
    ty.Literal["SOLE"],
    LIST,
    LIST_CONS,
    ty.Literal["NIL"],
    ty.Literal["ABSURD"],
    EQUAL,
    SAME,
    VEC,
    ty.Literal["VECNIL"],
    VEC_CONS,
    EITHER,
    LEFT,
    RIGHT,
    NEU,
    DELAY,
]


@dataclass
class NVar:
    name: str


@dataclass
class NTODO:
    where: "SrcLoc"
    type: Value


@dataclass
class NwhichNat:
    target: "Neutral"
    base: "Norm"
    step: "Norm"


@dataclass
class NiterNat:
    target: "Neutral"
    base: "Norm"
    step: "Norm"


@dataclass
class NrecNat:
    target: "Neutral"
    base: "Norm"
    step: "Norm"


@dataclass
class NindNat:
    target: "Neutral"
    motive: "Norm"
    base: "Norm"
    step: "Norm"


@dataclass
class Ncar:
    target: "Neutral"


@dataclass
class Ncdr:
    target: "Neutral"


@dataclass
class NrecList:
    target: "Neutral"
    base: "Norm"
    step: "Norm"


@dataclass
class NindList:
    target: "Neutral"
    motive: "Norm"
    base: "Norm"
    step: "Norm"


@dataclass
class NindAbsurd:
    target: "Neutral"
    motive: "Norm"


@dataclass
class Nreplace:
    target: "Neutral"
    motive: "Norm"
    base: "Norm"


@dataclass
class Ntrans1:
    target1: "Neutral"
    target2: "Norm"


@dataclass
class Ntrans2:
    target1: "Norm"
    target2: "Neutral"


@dataclass
class Ntrans12:
    target1: "Neutral"
    target2: "Neutral"


@dataclass
class Ncong:
    target1: "Neutral"
    function: "Norm"


@dataclass
class Nsymm:
    target1: "Neutral"


@dataclass
class NindMinusEq:
    target: "Neutral"
    motive: "Norm"
    base: "Norm"


@dataclass
class Nhead:
    target: "Neutral"


@dataclass
class Ntail:
    target: "Neutral"


@dataclass
class NindVec1:
    target1: "Neutral"
    target2: "Norm"
    motive: "Norm"
    base: "Norm"
    step: "Norm"


@dataclass
class NindVec2:
    target1: "Norm"
    target2: "Neutral"
    motive: "Norm"
    base: "Norm"
    step: "Norm"


@dataclass
class NindVec12:
    target1: "Neutral"
    target2: "Neutral"
    motive: "Norm"
    base: "Norm"
    step: "Norm"


@dataclass
class NindEither:
    target: "Neutral"
    motive: "Norm"
    base_left: "Norm"
    base_right: "Norm"


@dataclass
class Nap:
    rator: "Neutral"
    rand: "Norm"


Neutral: ty.TypeAlias = ty.Union[
    NVar,
    NTODO,
    NwhichNat,
    NiterNat,
    NrecNat,
    NindNat,
    Ncar,
    Ncdr,
    NrecList,
    NindList,
    NindAbsurd,
    Nreplace,
    Ntrans1,
    Ntrans2,
    Ntrans12,
    Ncong,
    Nsymm,
    NindMinusEq,
    Nhead,
    Ntail,
    NindVec1,
    NindVec2,
    NindVec12,
    NindEither,
    Nap,
]


@dataclass
class THE:
    type_: Value
    value: Value


Norm: ty.TypeAlias = THE


def is_var_name(x: str) -> bool:
    return not (
        x == "U"
        or x == "Nat"
        or x == "zero"
        or x == "add1"
        or x == "which-Nat"
        or x == "ind-Nat"
        or x == "rec-Nat"
        or x == "iter-Nat"
        or x == "->"
        or x == "→"
        or x == "Π"
        or x == "Pi"
        or x == "∏"
        or x == "λ"
        or x == "lambda"
        or x == "quote"
        or x == "Atom"
        or x == "Σ"
        or x == "Sigma"
        or x == "Pair"
        or x == "cons"
        or x == "car"
        or x == "cdr"
        or x == "Trivial"
        or x == "sole"
        or x == "::"
        or x == "nil"
        or x == "List"
        or x == "rec-List"
        or x == "ind-List"
        or x == "Absurd"
        or x == "ind-Absurd"
        or x == "="
        or x == "same"
        or x == "replace"
        or x == "symm"
        or x == "trans"
        or x == "cong"
        or x == "ind-="
        or x == "Vec"
        or x == "vec::"
        or x == "vecnil"
        or x == "head"
        or x == "tail"
        or x == "ind-Vec"
        or x == "Either"
        or x == "left"
        or x == "right"
        or x == "ind-Either"
        or x == "the"
        or x == "TODO"
    )


Ctx: ty.TypeAlias = ty.Dict[str, "Binder"]


@dataclass
class Def:
    type_: Value
    value: Value


@dataclass
class Free:
    type_: Value


@dataclass
class Claim:
    type_: Value


Binder: ty.TypeAlias = ty.Union[Def, Free, Claim]


def var_type(Γ: Ctx, x: str) -> Value:
    match Γ.get(x):
        case None | Claim(): raise KeyError(f"Unknown variable {x}")
        case b: return binder_type(b)


def binder_type(b: Binder) -> Value:
    match b:
        case Claim(tv) | Def(tv, _) | Free(tv): return tv


init_ctx: Ctx = {}


def bind_free(Γ: Ctx, x: str, tv: Value) -> Ctx:
    if x in Γ:
        raise KeyError(f"bind-free: {x} is already bound in {Γ}")
    return Γ | {x: Free(tv)}


def bind_val(Γ: Ctx, x: str, tv: Value, v: Value) -> Ctx:
    return Γ | {x: Def(tv, v)}


SerializableCtx: ty.TypeAlias = ty.Dict[
    str,
    ty.Union[
        ty.Tuple[ty.Literal["free"], Core],
        ty.Tuple[ty.Literal["def"], Core, Core],
        ty.Tuple[ty.Literal["claim"], Core],
    ]
]


Env: ty.TypeAlias = ty.Dict[str, Value]


def ctx_to_env(Γ: Ctx) -> Env:
    env = {}
    for x, b in Γ.items():
        match b:
            case Def(tv, v): env[x] = v
            case Free(tv): env[x] = NEU(tv, NVar(x))
            case Claim(tv): pass
    return env


def extend_env(ρ: Env, x: str, v: Value) -> Env:
    return ρ | {x: v}


def var_val(ρ: Env, x: str) -> Value:
    return ρ[x]


@dataclass
class FO_CLOS:
    env: Env
    var: str
    expr: Core


@dataclass
class HO_CLOS:
    proc: ty.Callable[[Value], Value]


Closure: ty.TypeAlias = ty.Union[FO_CLOS, HO_CLOS]


def fresh(Γ: Ctx, x: str) -> str:
    return freshen(names_only(Γ), x)


def fresh_binder(Γ: Ctx, expr: Src, x: str) -> str:
    return freshen(names_only(Γ) | occurring_names(expr), x)


def names_only(Γ: Ctx) -> ty.Set[str]:
    return set(Γ.items())


def occurring_names(expr: Src) -> ty.Set[str]:
    match src_stx(expr):
        case ("the", t, e): return occurring_names(t) | occurring_names(e)
        case ("quote", _): return set()
        case ("add1", n): return occurring_names(n)
        case ("which-Nat", tgt, base, step) | ("iter-Nat", tgt, base, step) | ("rec-Nat", tgt, base, step):
            return occurring_names(tgt) | occurring_names(base) | occurring_names(step)
        case ("ind-Nat", tgt, mot, base, step):
            return occurring_names(tgt) | occurring_names(mot) | occurring_names(base) | occurring_names(step)
        case ("->", t0, *ts):
            names = occurring_names(t0)
            for t in ts:
                names |= occurring_names(t)
            return names
        case ("Π", bindings, t) | ("Σ", bindings, t):
            names = occurring_names(t)
            for b in bindings:
                names |= occurring_binder_names(b)
            return names
        case ("λ", bindings, t):
            return occurring_names(t) | map(binder_var, bindings)
        case ("Pair", A, D): return occurring_names(A) | occurring_names(D)
        case ("cons", a, d): return occurring_names(a) | occurring_names(d)
        case ("car", p) | ("cdr", p): return occurring_names(p)
        case ("::", a, d): return occurring_names(a) | occurring_names(d)
        case ("List", E): return occurring_names(E)
        case ("rec-List", tgt, base, step):
            return occurring_names(tgt) | occurring_names(base) | occurring_names(step)
        case ("ind-List", tgt, mot, base, step):
            return occurring_names(tgt) | occurring_names(mot) | occurring_names(base) | occurring_names(step)
        case ("ind-Absurd", tgt, mot):
            return occurring_names(tgt) | occurring_names(mot)
        case ("=", A, from_, to):
            return occurring_names(A) | occurring_names(from_) | occurring_names(to)
        case ("same", e): return occurring_names(e)
        case ("replace", tgt, mot, base):
            return occurring_names(tgt) | occurring_names(mot) | occurring_names(base)
        #===========
        # TODO ...
        #===========
        case str(x) if is_var_name(x): set(x)
        case _: return set()


def occurring_binder_names(b: "TypedBinder") -> ty.Set[str]:
    raise NotImplementedError()
