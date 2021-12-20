use crate::basics::Ctx;
use crate::book::in_context;

use lazy_static::lazy_static;

lazy_static! {
    static ref CTX: Ctx = Ctx::new();
}

#[test]
fn test_002_a_quote_is_an_atom() {
    assert!(in_context(&CTX).core("'atom").is_a("Atom"));
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

#[test]
fn test_039_cdr_gets_second_element_of_pair() {
    in_context(&CTX)
        .core("(cdr (the (Pair Atom Atom) (cons 'ratatouille 'baguette)))")
        .and("'baguette")
        .are_the_same("Atom");
}

#[test]
fn test_040_nested_cons() {
    in_context(&CTX)
        .core("(the (Pair (Pair Atom Atom) Atom)
                    (cons (cons 'aubergine 'courgette) 'tomato))")
        .checks()
}

#[test]
fn test_041_access_nested_cons() {
    in_context(&CTX)
        .core("(car (cdr (the (Pair Atom (Pair Atom Atom))
                              (cons 'ratatouille (cons 'baguette 'olive-oil)))))")
        .and("'baguette")
        .are_the_same("Atom");
}

#[test]
fn test_056_only_the_normal_form_matters_for_sameness() {
    in_context(&CTX)
        .core("(car (the (Pair U Atom) (cons Atom 'olive)))").check()
        .and("(cdr (the (Pair Atom U) (cons 'oil Atom)))").check()
        .are_the_same_type();
}

#[test]
fn test_063_one_is_a_nat() {
    in_context(&CTX).core("1").is_a("Nat");
}

#[test]
fn test_064_a_big_positive_integer_is_a_nat() {
    in_context(&CTX).core("1729").is_a("Nat");
}

#[test]
fn test_065_minus_one_is_not_a_nat() {
    in_context(&CTX).core("-1").is_not_a("Nat");
}

#[test]
fn test_068_0_is_a_nat() {
    in_context(&CTX).core("0").is_a("Nat");
}

#[test]
fn test_072_different_nats_are_not_the_same() {
    in_context(&CTX).core("0").and("26").are_not_the_same("Nat");
}

#[test]
fn test_076_zero_is_a_nat() {
    in_context(&CTX).core("zero").is_a("Nat");
}

#[test]
#[should_panic]
fn test_077_identifiers_must_be_claimed_before_definition() {
    in_context(&CTX).define("one", "(add1 zero)");
}
