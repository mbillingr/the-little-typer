use crate::basics::Ctx;
use crate::book::in_context;

use crate::types::cores;
use lazy_static::lazy_static;

lazy_static! {
    static ref CTX: Ctx = Ctx::new();
}

#[test]
fn test_002_a_quote_is_an_atom() {
    assert!(in_context(&CTX).core("'atom").is_a(&cores::atom()));
}

#[test]
fn test_019_the_result_of_cons_is_a_pair() {
    in_context(&CTX)
        .core("(the (Pair Atom Atom) (cons 'ratatouille 'baguette))")
        .check();
}

#[test]
fn test_022_024_sameness_of_pairs() {
    in_context(&CTX).check_same(
        "(Pair Atom Atom)",
        "(cons 'ratatouille 'baguette)",
        "(cons 'ratatouille 'baguette)",
    );
    in_context(&CTX).check_not_same(
        "(Pair Atom Atom)",
        "(cons 'ratatouille 'baguette)",
        "(cons 'baguette 'baguette)",
    );
}

#[test]
fn test_026_a_pair_of_two_atoms_is_a_type() {
    assert!(in_context(&CTX).core("(Pair Atom Atom)",).is_a_type());
}

#[test]
fn test_the_law_of_atom() {
    assert!(in_context(&CTX).core("Atom",).is_a_type());
}

#[test]
fn test_031_032_compare_types() {
    in_context(&CTX).check_not_same_type("Atom", "(Pair Atom Atom)");
    in_context(&CTX).check_same_type("(Pair Atom Atom)", "(Pair Atom Atom)");
}
