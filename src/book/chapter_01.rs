use crate::book::{with_empty_context, ResultAssertions, ResultBoolAssertions};

#[test]
fn test_002_a_quote_is_an_atom() {
    assert!(with_empty_context().core("'atom").is_a("Atom"));
}

#[test]
fn test_019_the_result_of_cons_is_a_pair() {
    with_empty_context()
        .core("(the (Pair Atom Atom) (cons 'ratatouille 'baguette))")
        .checks();
}

#[test]
fn test_022_024_sameness_of_pairs() {
    with_empty_context()
        .core("(cons 'ratatouille 'baguette)")
        .and("(cons 'ratatouille 'baguette)")
        .are_the_same("(Pair Atom Atom)")
        .assert(true);
    with_empty_context()
        .core("(cons 'ratatouille 'baguette)")
        .and("(cons 'baguette 'baguette)")
        .are_the_same("(Pair Atom Atom)")
        .assert(false);
}

#[test]
fn test_026_a_pair_of_two_atoms_is_a_type() {
    with_empty_context().core("(Pair Atom Atom)").is_a_type();
}

#[test]
fn test_the_law_of_atom() {
    with_empty_context().core("Atom").is_a_type();
}

#[test]
fn test_031_032_compare_types() {
    with_empty_context()
        .core("Atom")
        .and("(Pair Atom Atom)")
        .are_not_the_same_type();
    with_empty_context()
        .core("(Pair Atom Atom)")
        .and("(Pair Atom Atom)")
        .are_the_same_type();
}

#[test]
//#[should_panic(expected = "NotAType")]
fn test_033_compare_over_non_type() {
    with_empty_context()
        .core("'peche")
        .and("'peche")
        .are_the_same("'fruit")
        .assert_err();
}

#[test]
fn test_038_car_gets_first_element_of_pair() {
    with_empty_context()
        .core("(car (the (Pair Atom Atom) (cons 'ratatouille 'baguette)))")
        .and("'ratatouille")
        .are_the_same("Atom")
        .assert(true);
}

#[test]
fn test_039_cdr_gets_second_element_of_pair() {
    with_empty_context()
        .core("(cdr (the (Pair Atom Atom) (cons 'ratatouille 'baguette)))")
        .and("'baguette")
        .are_the_same("Atom")
        .assert(true);
}

#[test]
fn test_040_nested_cons() {
    with_empty_context()
        .core(
            "(the (Pair (Pair Atom Atom) Atom)
                    (cons (cons 'aubergine 'courgette) 'tomato))",
        )
        .checks()
}

#[test]
fn test_041_access_nested_cons() {
    with_empty_context()
        .core(
            "(car (cdr (the (Pair Atom (Pair Atom Atom))
                              (cons 'ratatouille (cons 'baguette 'olive-oil)))))",
        )
        .and("'baguette")
        .are_the_same("Atom")
        .assert_ok();
}

#[test]
fn test_056_only_the_normal_form_matters_for_sameness() {
    with_empty_context()
        .core("(car (the (Pair U Atom) (cons Atom 'olive)))")
        .check()
        .and("(cdr (the (Pair Atom U) (cons 'oil Atom)))")
        .check()
        .are_the_same_type();
}

#[test]
fn test_063_one_is_a_nat() {
    with_empty_context().core("1").is_a("Nat");
}

#[test]
fn test_064_a_big_positive_integer_is_a_nat() {
    with_empty_context().core("1729").is_a("Nat");
}

#[test]
fn test_065_minus_one_is_not_a_nat() {
    with_empty_context().core("-1").is_not_a("Nat");
}

#[test]
fn test_068_0_is_a_nat() {
    with_empty_context().core("0").is_a("Nat");
}

#[test]
fn test_072_different_nats_are_not_the_same() {
    with_empty_context()
        .core("0")
        .and("26")
        .are_the_same("Nat")
        .assert(false);
}

#[test]
fn test_076_zero_is_a_nat() {
    with_empty_context().core("zero").is_a("Nat");
}

#[test]
fn test_077_identifiers_must_be_claimed_before_definition() {
    with_empty_context()
        .define("one", "(add1 zero)")
        .assert_err();
}

#[test]
fn test_079_identifiers_can_be_defined_after_claiming() {
    with_empty_context()
        .claim("one", "Nat")
        .define("one", "(add1 zero)")
        .unwrap()
        .core("one")
        .and("1")
        .are_the_same("Nat")
        .assert(true);
}
