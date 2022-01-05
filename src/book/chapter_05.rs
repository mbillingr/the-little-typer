use crate::book::common_definitions::with_book_context;
use crate::book::{with_empty_context, Checker, ResultBoolAssertions};

fn with_chapter_context() -> Checker {
    with_book_context()
}

#[test]
fn test_03_list_of_expectations() {
    with_chapter_context()
        .claim("expectations", "(List Atom)")
        .define("expectations", "(:: 'understood (:: 'slept nil))")
        .unwrap();
}

#[test]
fn test_07_08_nil_is_a_list_of_any_type() {
    with_empty_context()
        .core("nil")
        .is_a("(List Atom)")
        .assert(true);

    with_empty_context()
        .core("nil")
        .is_a("(List Nat)")
        .assert(true);

    with_empty_context()
        .core("nil")
        .is_a("(List (List Atom))")
        .assert(true);
}

#[test]
#[should_panic]
fn test_09_10_nil_is_not_a_list_that_is_no_type() {
    with_empty_context()
        .core("nil")
        .is_a("(List 'potato)");
}
