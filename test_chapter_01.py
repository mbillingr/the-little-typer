import pytest

from pypie.typechecker import synth, check_same


# tests are numbered according to the frames in the book


def test_002_a_quote_is_an_atom():
    assert check_same({}, "Atom", "'atom", ['the', 'Atom', "'atom"])


def test_019_the_result_of_cons_is_a_pair():
    assert typecheck(cons("ratatouille", "baguette"), Pair(Atom, Atom))


def test_022_024_sameness_of_pairs():
    assert are_same(Pair(Atom, Atom))(
        cons("ratatouille", "baguette"), cons("ratatouille", "baguette")
    )
    assert not are_same(Pair(Atom, Atom))(
        cons("ratatouille", "baguette"), cons("baguette", "baguette")
    )


def test_026_a_pair_of_two_atoms_is_a_type():
    assert is_type(Pair(Atom, Atom))


def test_the_law_of_atom():
    assert is_type(Atom)


def test_031_compare_types():
    assert not are_same_type(Atom, Pair(Atom, Atom))


def test_032_compare_types():
    assert are_same_type(Pair(Atom, Atom), Pair(Atom, Atom))


def test_033_compare_over_non_type():
    with pytest.raises(NotATypeError):
        are_same("fruit")("peche", "peche")


def test_038_car_gets_first_element_of_pair():
    assert are_same(Atom)(car(cons("ratatouille", "baguette")), "ratatouille")


def test_039_car_gets_first_element_of_pair():
    assert are_same(Atom)(cdr(cons("ratatouille", "baguette")), "baguette")


def test_040_nested_cons():
    assert typecheck(
        car(cons(cons("aubergine", "courgette"), "tomato")), Pair(Atom, Atom)
    )


def test_041_access_nested_cons():
    assert are_same(Atom)(
        car(cdr(cons("ratatouille", cons("baguette", "olive oil")))), "baguette"
    )


def test_054_pair_type_only_accepts_types():
    with pytest.raises(NotATypeError):
        Pair(Atom, "olive")
    with pytest.raises(NotATypeError):
        Pair("oil", Atom)


def test_056_only_the_normal_form_matters():
    assert are_same_type(
        Pair(car(cons(Atom, "olive")), cdr(cons("oil", Atom))), Pair(Atom, Atom)
    )


def test_063_one_is_a_nat():
    assert typecheck(1, Nat)


def test_064_a_big_positive_integer_is_a_nat():
    assert typecheck(1729, Nat)


def test_065_minus_one_is_not_a_nat():
    with pytest.raises(TypeMismatch):
        typecheck(-1, Nat)


def test_068_0_is_a_nat():
    assert typecheck(0, Nat)


def test_072_different_nats_are_not_the_same():
    assert not are_same(Nat)(0, 26)


def test_076_zero_is_a_nat():
    assert typecheck(zero, Nat)


def test_077_identifiers_must_be_claimed_before_definition():
    ctx = Context()
    with pytest.raises(AssertionError):
        ctx.define("one", add1(zero))


def test_079_identifiers_can_be_defined_after_claiming():
    ctx = Context()
    ctx.claim("one", Nat)
    ctx.define("one", add1(zero))
    typecheck(ctx.one, Nat)


def test_120_nested_pair_types():
    assert typecheck(
        cons("basil", cons("thyme", "oregano")), Pair(Atom, Pair(Atom, Atom))
    )
