from pypie.atom import Atom
from pypie.core import (
    the,
    typecheck,
    U,
)
from pypie.nat import Nat
from pypie.pair import car, cons, Pair


def test_012_type_annotation_defines_the_type():
    assert typecheck(
        the(Pair(Atom, Atom), cons("spinach", "cauliflower")), Pair(Atom, Atom)
    )


def test_018_type_annotation_passes_through_the_value():
    assert car(the(Pair(Atom, Nat), cons("brussel-sprout", 4))) == "brussel-sprout"
