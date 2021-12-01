import pytest

from pypie.atom import Atom
from pypie.context import Context
from pypie.core import (
    are_same,
    are_same_type,
    claim_define,
    is_type,
    the,
    typecheck,
    NotATypeError,
    TypeMismatch,
    U,
)
from pypie.fun import Fun
from pypie.nat import add1, iter_nat, Nat, plus, which_nat, zero
from pypie.pair import car, cdr, cons, Pair
from pypie.typevar import TypeVar


def test_027_define_plus():
    step_plus = claim_define(Fun(Nat, Nat), lambda x: add1(x))
    plus = claim_define(Fun(Nat, Nat, Nat), lambda n, j: iter_nat(n, j, step_plus))
