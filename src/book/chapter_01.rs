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
        .checks();
}

#[test]
fn test_022_024_sameness_of_pairs() {
    in_context(&CTX)
        .core("(cons 'ratatouille 'baguette)")
        .and("(cons 'ratatouille 'baguette)")
        .are_the_same("(Pair Atom Atom)");
    in_context(&CTX)
        .core("(cons 'ratatouille 'baguette)")
        .and("(cons 'baguette 'baguette)")
        .are_not_the_same("(Pair Atom Atom)");
}

#[test]
fn test_026_a_pair_of_two_atoms_is_a_type() {
    in_context(&CTX).core("(Pair Atom Atom)").is_a_type();
}

#[test]
fn test_the_law_of_atom() {
    in_context(&CTX).core("Atom").is_a_type();
}

#[test]
fn test_031_032_compare_types() {
    in_context(&CTX)
        .core("Atom")
        .and("(Pair Atom Atom)")
        .are_not_the_same_type();
    in_context(&CTX)
        .core("(Pair Atom Atom)")
        .and("(Pair Atom Atom)")
        .are_the_same_type();
}

#[test]
#[should_panic(expected = "NotAType")]
fn test_033_compare_over_non_type() {
    in_context(&CTX)
        .core("'peche")
        .and("'peche")
        .are_the_same("'fruit");
}

#[test]
fn test_038_car_gets_first_element_of_pair() {
    in_context(&CTX)
        .core("(car (the (Pair Atom Atom) (cons 'ratatouille 'baguette)))")
        .and("'ratatouille")
        .are_the_same("Atom");
}
