use crate::book::{chapter_14, Checker, ResultBoolAssertions};
use lazy_static::lazy_static;

lazy_static! {
    static ref CHAPTER_CONTEXT: Checker = chapter_14::with_chapter_context()
        .claim("=consequence", "(-> Nat Nat U)")
        .define(
            "=consequence",
            "(λ (n j)
                (which-Nat n
                    (which-Nat j
                        Trivial
                        (λ (j-1) Absurd))
                    (λ (n-1)
                        (which-Nat j
                            Absurd
                            (λ (j-1) (= Nat n-1 j-1))))))"
        )
        .unwrap()
        .claim("=consequence-same", "(Π ((n Nat)) (=consequence n n))")
        .define(
            "=consequence-same",
            "(λ (n)
                (ind-Nat n
                    (λ (k) (=consequence k k))
                    sole
                    (λ (n-1 =consequence_n-1) (same n-1))))"
        )
        .unwrap()
        .claim(
            "use-Nat=",
            "(Π ((n Nat) (j Nat)) (-> (= Nat n j) (=consequence n j)))"
        )
        .define(
            "use-Nat=",
            "(λ (n j n=j) (replace n=j (λ (k) (=consequence n k)) (=consequence-same n)))"
        )
        .unwrap();
}

pub fn with_chapter_context() -> Checker {
    CHAPTER_CONTEXT.clone()
}

#[test]
fn frame__() {
    with_chapter_context();
}
