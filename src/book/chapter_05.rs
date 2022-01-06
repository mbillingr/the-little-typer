use crate::book::common_definitions::with_book_context;
use crate::book::{with_empty_context, Checker, ResultBoolAssertions};

fn with_chapter_context() -> Checker {
    with_book_context()
        .claim("toppings", "(List Atom)")
        .define("toppings", "(:: 'potato (:: 'butter nil))")
        .unwrap()
        .claim("condiments", "(List Atom)")
        .define("condiments", "(:: 'chives (:: 'mayonnaise nil))")
        .unwrap()
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
    with_empty_context().core("nil").is_a("(List 'potato)");
}

#[test]
fn test_39_try_length() {
    with_book_context()
        .core("((length Atom) nil)")
        .and("0")
        .are_the_same("Nat")
        .assert(true);

    with_book_context()
        .core("((length Atom) (:: 'a nil))")
        .and("1")
        .are_the_same("Nat")
        .assert(true);

    with_book_context()
        .core("((length Atom) (:: 'b (:: 'a nil)))")
        .and("2")
        .are_the_same("Nat")
        .assert(true);
}

#[test]
fn test_50_append() {
    with_book_context()
        .core("(append Atom (:: 'cucumber (:: 'tomato nil)) (:: 'rye-bread nil))")
        .and("(:: 'cucumber (:: 'tomato (:: 'rye-bread nil)))")
        .are_the_same("(List Atom)")
        .assert(true);
}

#[test]
fn test_67_reverse() {
    with_chapter_context()
        .claim("kartoffelmad", "(List Atom)")
        .define("kartoffelmad", "(append Atom (append Atom condiments toppings) (reverse Atom (:: 'plate (:: 'rye-bread nil))))").unwrap()
        .core("kartoffelmad")
        .and("(:: 'chives (:: 'mayonnaise (:: 'potato (:: 'butter (:: 'rye-bread (:: 'plate nil))))))")
        .are_the_same("(List Atom)")
        .assert(true);
}
