use crate::book::common_definitions::with_book_context;
use crate::book::{with_empty_context, Checker, ResultBoolAssertions};

fn with_chapter_context() -> Checker {
    with_book_context()
        .claim("first", "(Π ((E U) (l Nat)) (-> (Vec E (add1 l)) E))")
        .define("first", "(λ (E l v) (head v))")
        .unwrap()
        .claim(
            "rest",
            "(Π ((E U) (l Nat)) (-> (Vec E (add1 l)) (Vec E l)))",
        )
        .define("rest", "(λ (E l v) (tail v))")
        .unwrap()
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

#[test]
fn test_36_first() {
    with_chapter_context()
        .core("(first Atom 3 (vec:: 'chicken-of-the-woods (vec:: 'chantrelle (vec:: 'lions-mane (vec:: 'puffball vecnil)))))")
        .and("'chicken-of-the-woods")
        .are_the_same("Atom")
        .assert(true);
}