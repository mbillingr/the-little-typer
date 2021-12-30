use crate::book::{with_empty_context, Checker};

pub fn with_book_context() -> Checker {
    with_empty_context()
        // + (addition)
        .claim("step-+", "(-> Nat Nat)")
        .define("step-+", "(lambda (+_n-1) (add1 +_n-1))")
        .unwrap()
        .claim("+", "(-> Nat Nat Nat)")
        .define("+", "(lambda (n j) (iter-Nat n j step-+))")
        .unwrap()
        // * (multiplication)
        .claim("step-*", "(-> Nat Nat Nat Nat)")
        .define("step-*", "(lambda (j n-1 *_n-1) (+ j *_n-1))")
        .unwrap()
        .claim("*", "(-> Nat Nat Nat)")
        .define("*", "(lambda (n j) (rec-Nat n 0 (step-* j)))")
        .unwrap()
        // flip
        .claim("flip", "(Pi ((A U) (D U)) (-> (Pair A D) (Pair D A)))")
        .define("flip", "(lambda (A D) (lambda (p) (cons (cdr p) (car p))))")
        .unwrap()
}
