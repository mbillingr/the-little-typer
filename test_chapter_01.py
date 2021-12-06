import pytest

from pypie.expr import Add1, Atom, Car, Cdr, Cons, Nat, Pair, Ref, The, U, zero
from pypie import typechecker as tc
from pypie.typechecker import Claim, ConversionError, TypeMismatch, NotATypeError, define, claim


# tests are numbered according to the frames in the book


def same(t, a, b, ctx={}):
    try:
        tc.check_same(ctx, t, a, b)
    except ConversionError:
        return False
    return True


def same_type(a, b, ctx={}):
    return same(U(), a, b, ctx)


def is_a(t, v, ctx={}):
    return tc.is_a(ctx, t, v)


def is_type(t, ctx={}):
    renaming = {}
    return t.as_type(ctx, renaming)


def test_002_a_quote_is_an_atom():
    assert is_a(Atom(), "atom")


def test_019_the_result_of_cons_is_a_pair():
    assert is_a(Pair(Atom(), Atom()), Cons("ratatouille", "baguette"))


def test_022_024_sameness_of_pairs():
    assert same(
        Pair(Atom(), Atom()),
        Cons("ratatouille", "baguette"),
        Cons("ratatouille", "baguette"),
    )

    assert not same(
        Pair(Atom(), Atom()),
        Cons("ratatouille", "baguette"),
        Cons("baguette", "baguette"),
    )


def test_026_a_pair_of_two_atoms_is_a_type():
    assert is_type(Pair(Atom(), Atom()))


def test_the_law_of_atom():
    assert is_type(Atom())


def test_031_compare_types():
    assert not same_type(Atom(), Pair(Atom(), Atom()))


def test_032_compare_types():
    assert same_type(Pair(Atom(), Atom()), Pair(Atom(), Atom()))


def test_033_compare_over_non_type():
    with pytest.raises(NotATypeError):
        same("fruit", "peche", "peche")


def test_038_car_gets_first_element_of_pair():
    the_pair = The(Pair(Atom(), Atom()), Cons("ratatouille", "baguette"))
    assert same(Atom(), Car(the_pair), "ratatouille")


def test_039_cdr_gets_second_element_of_pair():
    the_pair = The(Pair(Atom(), Atom()), Cons("ratatouille", "baguette"))
    assert same(Atom(), Cdr(the_pair), "baguette")


def test_040_nested_cons():
    outer_pair = The(
        Pair(Pair(Atom(), Atom()), Atom()),
        Cons(Cons("aubergine", "courgette"), "tomato"),
    )
    assert outer_pair.check()


def test_041_access_nested_cons():
    outer_pair = The(
        Pair(Atom(), Pair(Atom(), Atom())),
        Cons("ratatouille", Cons("baguette", "olive oil")),
    )
    assert same(Atom(), Car(Cdr(outer_pair)), "baguette")


def test_056_only_the_normal_form_matters():
    pair1 = The(Pair(U(), Atom()), Cons(Atom(), "olive"))
    pair2 = The(Pair(Atom(), U()), Cons("oil", Atom()))
    assert same_type(Pair(Car(pair1), Cdr(pair2)), Pair(Atom(), Atom()))


def test_063_one_is_a_nat():
    assert is_a(Nat(), 1)


def test_064_a_big_positive_integer_is_a_nat():
    assert is_a(Nat(), 1729)


def test_065_minus_one_is_not_a_nat():
    with pytest.raises(AssertionError):
        is_a(Nat(), -1)


def test_068_0_is_a_nat():
    assert is_a(Nat(), 0)


def test_072_different_nats_are_not_the_same():
    assert not same(Nat(), 0, 26)


def test_076_zero_is_a_nat():
    assert is_a(Nat(), zero)


def test_077_identifiers_must_be_claimed_before_definition():
    with pytest.raises(AssertionError):
        define({}, "one", Add1(zero))


def test_079_identifiers_can_be_defined_after_claiming():
    ctx = {}
    claim(ctx, "one", Nat())
    define(ctx, "one", Add1(zero))
    assert same(Nat(), Ref("one"), 1, ctx)


def test_120_nested_pair_types():
    assert The(
        Pair(Atom(), Pair(Atom(), Atom())),
        Cons("basil", Cons("thyme", "oregano"))
    ).check()
