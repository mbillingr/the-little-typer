use crate::book::common_definitions::with_book_context;
use crate::book::{with_empty_context, Checker, ResultAssertions, ResultBoolAssertions};

fn with_chapter_context() -> Checker {
    with_book_context()
        // ------
        //  incr
        // ------
        .claim("incr", "(-> Nat Nat)")
        .define("incr", "(λ (n) (iter-Nat n 1 (+ 1)))")
        .unwrap()
}

#[test]
fn frame_05_incr_zero() {
    with_chapter_context()
        .core("(incr 0)")
        .and("1")
        .are_the_same("Nat")
        .assert(true);
}

#[test]
fn frame_06_incr_zero() {
    with_chapter_context()
        .core("(incr 3)")
        .and("4")
        .are_the_same("Nat")
        .assert(true);
}

#[test]
fn frame_14_eq_is_a_type() {
    with_empty_context()
        .core("(= Atom 'kale 'blackberries)")
        .is_a_type()
        .assert(true);
}

#[test]
fn frame_35_same_is_an_equal() {
    with_book_context()
        .core("(same 21)")
        .is_a("(= Nat (+ 17 4) (+ 11 10))")
        .assert(true);
}

#[test]
fn frame_38_prove_addition() {
    with_book_context()
        .claim("+1=add1", "(Π ((n Nat)) (= Nat (+ 1 n) (add1 n)))")
        .define("+1=add1", "(λ (n) (same (add1 n)))")
        .unwrap();
}

#[test]
fn frame_44_cant_simply_prove_incr() {
    with_chapter_context()
        .claim("incr=add1", "(Π ((n Nat)) (= Nat (incr n) (add1 n)))")
        .define("incr=add1", "(λ (n) (same (add1 n)))")
        .assert_err();
}
