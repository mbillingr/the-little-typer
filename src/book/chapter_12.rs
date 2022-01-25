use crate::book::{chapter_11, Checker};
use lazy_static::lazy_static;

lazy_static! {
    static ref CHAPTER_CONTEXT: Checker = chapter_11::with_chapter_context()
        // ------
        //  Even
        // ------
        .claim("Even", "(-> Nat U)")
        .define("Even", "(λ (n) (Σ ((half Nat)) (= Nat n (double half))))")
        .unwrap()
        // --------------
        //  zero-is-even
        // --------------
        .claim("zero-is-even", "(Even 0)")
        .define("zero-is-even", "(cons 0 (same 0))")
        .unwrap()
        // -----------
        //  +two-even
        // -----------
        .claim("+two-even", "(Π ((n Nat)) (-> (Even n) (Even (+ 2 n))))")
        .define(
            "+two-even",
            "(λ (n e_n) (cons (add1 (car e_n)) (cong (cdr e_n) (+ 2))))",
        )
        .unwrap()
        // -----
        //  Odd
        // -----
        .claim("Odd", "(-> Nat U)")
        .define(
            "Odd",
            "(λ (n) (Σ ((haf Nat)) (= Nat n (add1 (double haf)))))",
        )
        .unwrap()
        // --------------
        //  one-is-odd
        // --------------
        .claim("one-is-odd", "(Odd 1)")
        .define("one-is-odd", "(cons 0 (same 1))")
        .unwrap()
        // ----------------
        //  add1-even->odd
        // ----------------
        .claim(
            "add1-even->odd",
            "(Π ((n Nat)) (-> (Even n) (Odd (add1 n))))",
        )
        .define(
            "add1-even->odd",
            "(λ (n e_n) (cons (car e_n) (cong (cdr e_n) (+ 1))))",
        )
        .unwrap()
        // ----------------
        //  add1-odd->even
        // ----------------
        .claim(
            "add1-odd->even",
            "(Π ((n Nat)) (-> (Odd n) (Even (add1 n))))",
        )
        .define(
            "add1-odd->even",
            "(λ (n o_n) (cons (add1 (car o_n)) (cong (cdr o_n) (+ 1))))",
        )
        .unwrap()
        // --------
        //  repeat
        // --------
        .claim("repeat", "(-> (-> Nat Nat) Nat Nat)")
        .define(
            "repeat",
            "(λ (f n) (iter-Nat n (f 1) (λ (iter_f,n-1) (f iter_f,n-1))))",
        )
        .unwrap()
        // -----------
        //  ackermann
        // -----------
        .claim("ackermann", "(-> Nat Nat Nat)")
        .define(
            "ackermann",
            "(λ (n) (iter-Nat n (+ 1) (λ (ackermann_n-1) (repeat ackermann_n-1))))",
        )
        .unwrap();
}

pub fn with_chapter_context() -> Checker {
    CHAPTER_CONTEXT.clone()
}

#[test]
fn frame_08_prove_that_10_is_even() {
    with_chapter_context()
        .core("(the (Even 10) (cons 5 (same 10)))")
        .checks()
}

#[test]
fn frame_28_prove_that_two_is_even() {
    // for every natural number n, if n is even, n+2 is even
    with_chapter_context()
        .claim("two-is-even", "(Even 2)")
        .define("two-is-even", "(+two-even 0 zero-is-even)")
        .unwrap();
}

#[test]
fn frame_35_prove_that_13_is_odd() {
    with_chapter_context()
        .core("(the (Odd 13) (cons 6 (same 13)))")
        .checks()
}

#[test]
fn frame_58_ackermann() {
    let ctx = with_chapter_context();
    ctx.core("(ackermann 0 0)").evaluates_to("1");
    ctx.core("(ackermann 0 1)").evaluates_to("2");
    ctx.core("(ackermann 1 0)").evaluates_to("2");
    ctx.core("(ackermann 1 1)").evaluates_to("3");
    ctx.core("(ackermann 1 2)").evaluates_to("4");
    ctx.core("(ackermann 2 1)").evaluates_to("5");
    ctx.core("(ackermann 2 2)").evaluates_to("7");
    ctx.core("(ackermann 2 3)").evaluates_to("9");
    ctx.core("(ackermann 3 2)").evaluates_to("29");
    ctx.core("(ackermann 3 3)").evaluates_to("61");
}
