use crate::book::{with_empty_context, Checker};

pub fn with_book_context() -> Checker {
    with_empty_context()
        .claim("step-+", "(-> Nat Nat)")
        .define("step-+", "(lambda (+_n-1) (add1 +_n-1))")
        .unwrap()
        .claim("+", "(-> Nat Nat Nat)")
        .define("+", "(lambda (n j) (iter-Nat n j step-+))")
        .unwrap()
        .claim("step-*", "(-> Nat Nat Nat Nat)")
        .define("step-*", "(lambda (j n-1 *_n-1) (+ j *_n-1))")
        .unwrap()
        .claim("*", "(-> Nat Nat Nat)")
        .define("*", "(lambda (n j) (rec-Nat n 0 (step-* j)))")
        .unwrap()
}
