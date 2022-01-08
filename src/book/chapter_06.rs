use crate::book::common_definitions::with_book_context;
use crate::book::{with_empty_context, Checker, ResultBoolAssertions};

fn with_chapter_context() -> Checker {
    with_book_context()
}

#[test]
fn test_08_the_vec_type() {
    with_chapter_context()
        .core("(Vec Atom 3)")
        .is_a_type()
        .assert(true);
}

#[test]
fn test_10_vecnil() {
    with_empty_context()
        .core("vecnil")
        .is_a("(Vec Atom 0)")
        .assert(true)
}

#[test]
fn test_13_veccons() {
    with_empty_context()
        .core("(vec:: 'oyster vecnil)")
        .is_a("(Vec Atom 1)")
        .assert(true)
}

#[test]
fn test_14_veccons() {
    with_empty_context()
        .core("(vec:: 'crimini (vec:: 'shiitake vecnil))")
        .is_not_a("(Vec Atom 3)")
        .assert(true);
}

#[test]
fn test_22_head() {
    with_empty_context()
        .core("(head (the (Vec Atom 1) (vec:: 'a vecnil)))")
        .and("'a")
        .are_the_same("Atom")
        .assert(true);
}

#[test]
fn test_24_tail() {
    with_empty_context()
        .core("(tail (the (Vec Atom 1) (vec:: 'a vecnil)))")
        .and("vecnil")
        .are_the_same("(Vec Atom 0)")
        .assert(true);
}
