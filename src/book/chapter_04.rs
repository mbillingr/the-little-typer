use crate::book::common_definitions::with_book_context;
use crate::book::{with_empty_context, Checker, ResultBoolAssertions};

fn with_chapter_context() -> Checker {
    with_book_context()
        // elim-Pair
        .claim(
            "elim-Pair",
            "(Pi ((A U) (D U) (X U)) (-> (Pair A D) (-> A D X) X))",
        )
        .define(
            "elim-Pair",
            "(lambda (A D X) (lambda (p f) (f (car p) (cdr p))))",
        )
        .unwrap()
        // kar ((Pair Nat Nat) eliminator)
        .claim("kar", "(-> (Pair Nat Nat) Nat)")
        .define(
            "kar",
            "(lambda (p) (elim-Pair Nat Nat Nat p (lambda (a d) a)))",
        )
        .unwrap()
        // kdr ((Pair Nat Nat) eliminator)
        .claim("kdr", "(-> (Pair Nat Nat) Nat)")
        .define(
            "kdr",
            "(lambda (p) (elim-Pair Nat Nat Nat p (lambda (a d) d)))",
        )
        .unwrap()
}

#[test]
fn test_27_value_of_flipped_pair() {
    with_chapter_context()
        .core("((flip Nat Atom) (cons 17 'apple))")
        .and("(cons 'apple 17)")
        .are_the_same("(Pair Atom Nat)")
        .assert(true);
}
