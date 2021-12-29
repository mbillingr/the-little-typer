use crate::book::common_definitions::with_book_context;
use crate::book::{with_empty_context, ResultBoolAssertions};

#[test]
fn test_020_simple_iter_nat_example() {
    with_empty_context()
        .core("(iter-Nat 5 3 (lambda (smaller) (add1 smaller)))")
        .and("8")
        .are_the_same("Nat")
        .assert(true);
}

#[test]
fn test_024pp_defining_plus() {
    with_empty_context()
        .claim("step-+", "(-> Nat Nat)")
        .define("step-+", "(lambda (+n-1) (add1 +n-1))")
        .unwrap()
        .claim("+", "(-> Nat Nat Nat)")
        .define("+", "(lambda (n j) (iter-Nat n j step-+))")
        .unwrap()
        .core("(+ (add1 zero) 7)")
        .and("8")
        .are_the_same("Nat")
        .assert(true);
}

#[test]
fn test_036pp_recnat() {
    with_empty_context()
        .core("(rec-Nat (add1 zero) 0 (lambda (n-1 almost) (add1 (add1 almost))))")
        .and("2")
        .are_the_same("Nat")
        .assert(true);
}

#[test]
fn test_043_check_for_zero() {
    let chk = with_empty_context()
        .claim("step-zerop", "(-> Nat Atom Atom)")
        .define("step-zerop", "(lambda (n-1, zerop_n-1) 'nil)")
        .unwrap()
        .claim("zerop", "(-> Nat Atom)")
        .define("zerop", "(lambda (n) (rec-Nat n 't step-zerop))")
        .unwrap();

    chk.core("(zerop zero)")
        .and("'t")
        .are_the_same("Atom")
        .assert(true);

    chk.core("(zerop 42)")
        .and("'nil")
        .are_the_same("Atom")
        .assert(true);
}

#[test]
fn test_49pp_gauss_defined() {
    let chk = with_book_context()
        .claim("step-gauss", "(-> Nat Nat Nat)")
        .define(
            "step-gauss",
            "(lambda (n-1 gauss_n-1) (+ (add1 n-1) gauss_n-1))",
        )
        .unwrap()
        .claim("gauss", "(-> Nat Nat)")
        .define("gauss", "(lambda (n) (rec-Nat n 0 step-gauss))")
        .unwrap();

    chk.core("(gauss 0)")
        .and("0")
        .are_the_same("Nat")
        .assert(true);

    chk.core("(gauss 1)")
        .and("1")
        .are_the_same("Nat")
        .assert(true);

    chk.core("(gauss 5)")
        .and("15")
        .are_the_same("Nat")
        .assert(true);
}

#[test]
fn test_71_multiplication() {
    with_book_context()
        .core("(* 2 29)")
        .and("58")
        .are_the_same("Nat")
        .assert(true);
}
