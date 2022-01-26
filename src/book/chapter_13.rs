use crate::book::{chapter_12, Checker, ResultBoolAssertions};
use lazy_static::lazy_static;

lazy_static! {
    static ref CHAPTER_CONTEXT: Checker = chapter_12::with_chapter_context()
        // -------------
        //  even-or-odd
        // -------------
        .claim("even-or-odd", "(Π ((n Nat)) (Either (Even n) (Odd n)))")
        .claim("mot-even-or-odd", "(-> Nat U)")
        .define("mot-even-or-odd", "(λ (k) (Either (Even k) (Odd k)))")
        .unwrap()
        .claim("step-even-or-odd", "(Π ((n-1 Nat)) (-> (mot-even-or-odd n-1) (mot-even-or-odd (add1 n-1))))")
        .define(
            "step-even-or-odd",
            "(λ (n-1 e-or-o_n-1)
                (ind-Either e-or-o_n-1
                    (λ (e-or-o)
                       (mot-even-or-odd (add1 n-1)))
                    (λ (e_n-1) (right (add1-even->odd n-1 e_n1)))
                    (λ (o_n-1) (left (add1-odd->even n-1 o_n1)))))")
        .unwrap()
        ;
}

pub fn with_chapter_context() -> Checker {
    CHAPTER_CONTEXT.clone()
}

#[test]
fn frame_03_when_either_is_a_type() {
    with_chapter_context()
        .core("(Either Nat Atom)")
        .is_a_type()
        .assert(true);
}

#[test]
fn frame_04_when_either_is_a_type() {
    with_chapter_context()
        .core("(left 1)")
        .and("(left 1)")
        .are_the_same("(Either Nat Atom)")
        .assert(true);

    with_chapter_context()
        .core("(right 'a)")
        .and("(right 'b)")
        .are_the_same("(Either Nat Atom)")
        .assert(false)
}
