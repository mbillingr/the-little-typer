import pytest

from pypie.atom import Atom
from pypie.context import Context
from pypie.core import (
    are_same,
    are_same_type,
    claim_define,
    is_type,
    typecheck,
    NotATypeError,
    TypeMismatch,
    U,
)
from pypie.fun import Fun
from pypie.nat import add1, Nat, plus, which_nat, zero
from pypie.pair import car, cdr, cons, Pair
from pypie.typevar import TypeVar


def test_015_functions_types():
    assert typecheck(lambda x: cons(x, x), Fun(Atom, Pair(Atom, Atom)))


def test_016_functions_types_can_be_computed():
    assert are_same_type(
        Fun(Atom, Pair(Atom, Atom)),
        Fun(car(cons(Atom, "pepper")), Pair(cdr(cons("salt", Atom)), Atom)),
    )


def test_019_different_functions_with_same_body_are_the_same():
    assert are_same(Fun(Nat, Pair(Nat, Nat)))(
        lambda x: cons(x, x), lambda y: cons(y, y)
    )


def test_019_different_functions_with_same_body_are_the_same():
    assert not are_same(Fun(Atom, Atom, Pair(Atom, Atom)))(
        lambda a, d: cons(a, d), lambda d, a: cons(a, d)
    )


def test_initial_second_commandment_of_lambda():
    """I interpret it as 'Two functions are the same if they behave the same'"""
    F = Fun(Nat, Pair(Nat, Nat))

    def f(x):
        return cons(x, x)

    assert are_same(F)(f, lambda y: f(y))


def test_035_define_names():
    vegetables = claim_define(Pair(Atom, Atom), cons("celery", "carrot"))

    assert are_same(Pair(Atom, Atom))(vegetables, cons("celery", "carrot"))


def test_037_consing_the_parts_of_a_pair_yields_the_same_pair():
    vegetables = claim_define(Pair(Atom, Atom), cons("celery", "carrot"))

    assert are_same(Pair(Atom, Atom))(
        vegetables, cons(car(vegetables), cdr(vegetables))
    )


def test_038_consing_the_parts_of_any_pair_yields_the_same_pair():
    t = Pair(Atom, Atom)
    p = TypeVar(t)
    assert are_same(t)(p, cons(car(p), cdr(p)))


def test_046_which_nat_zero():
    assert are_same(Atom)(which_nat(zero, "naught", lambda _: "more"), "naught")


def test_049_which_nat_zero():
    assert are_same(Atom)(which_nat(4, "naught", lambda _: "more"), "more")


def test_075_we_can_cons_types():
    assert typecheck(cons(Atom, Atom), Pair(U, U))


def test_082_pears_are_pairs_of_nats():
    Pear = claim_define(U, Pair(Nat, Nat))
    assert typecheck(cons(3, 5), Pear)


def test_095_elim_pear():
    Pear = claim_define(U, Pair(Nat, Nat))
    PearMaker = claim_define(U, Fun(Nat, Nat, Pear))
    elim_pear = claim_define(
        Fun(Pear, PearMaker, Pear), lambda pear, maker: maker(car(pear), cdr(pear))
    )

    assert are_same(Pear)(elim_pear(cons(3, 17), lambda a, d: cons(d, a)), cons(17, 3))


def test_100_pairwise_plus():
    Pear = claim_define(U, Pair(Nat, Nat))
    PearMaker = claim_define(U, Fun(Nat, Nat, Pear))
    elim_pear = claim_define(
        Fun(Pear, PearMaker, Pear), lambda pear, maker: maker(car(pear), cdr(pear))
    )

    pearwise_add = claim_define(
        Fun(Pear, Pear, Pear),
        lambda anjou, bosc: elim_pear(
            anjou,
            lambda a1, d1: elim_pear(
                bosc, lambda a2, d2: cons(plus(a1, a2), plus(d1, d2))
            ),
        ),
    )

    assert are_same(Pear)(pearwise_add(cons(3, 8), cons(7, 6)), cons(10, 14))
