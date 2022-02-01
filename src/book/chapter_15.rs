use crate::book::{chapter_14, Checker, ResultBoolAssertions};
use lazy_static::lazy_static;

lazy_static! {
    static ref CHAPTER_CONTEXT: Checker = chapter_14::with_chapter_context()
        // --------------
        //  =consequence
        // --------------
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
        // -------------------
        //  =consequence-same
        // -------------------
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
        // ----------
        //  use-Nat=
        // ----------
        .claim(
            "use-Nat=",
            "(Π ((n Nat) (j Nat)) (-> (= Nat n j) (=consequence n j)))"
        )
        .define(
            "use-Nat=",
            "(λ (n j n=j) (replace n=j (λ (k) (=consequence n k)) (=consequence-same n)))"
        )
        .unwrap()
        // ---------------
        //  zero-not-add1
        // ---------------
        .claim(
            "zero-not-add1",
            "(Π ((n Nat)) (-> (= Nat zero (add1 n)) Absurd))"
        )
        .define(
            "zero-not-add1",
            "(λ (n) (use-Nat= zero (add1 n)))"
        )
        .unwrap()
        // -------
        //  sub1=
        // -------
        .claim("sub1=", "(Π ((n Nat) (j Nat)) (-> (= Nat (add1 n) (add1 j)) (= Nat n j)))")
        .define("sub1=", "(λ (n j) (use-Nat= (add1 n) (add1 j)))")
        .unwrap()
        // -------
        //  front
        // -------
        .claim("front", "(Π ((E U) (n Nat)) (-> (Vec E (add1 n)) E))")
        .claim("mot-front", "(Π ((E U) (k Nat)) (-> (Vec E k) U))")
        .define("mot-front", "(λ (E k es) (Π ((j Nat)) (-> (= Nat k (add1 j)) E)))")
        .unwrap()
        .claim(
            "step-front",
            "(Π ((E U) (l Nat) (e E) (es (Vec E l)))
                (-> (mot-front E l es) (mot-front E (add1 l) (vec:: e es))))")
        .define("step-front", "(λ (E l e es front_es j eq) e)")
        .unwrap()
        .define(
            "front",
            "(λ (E l es)
                ((ind-Vec (add1 l) es
                    (mot-front E)
                    (λ (j eq) (ind-Absurd (zero-not-add1 j eq) E))
                    (step-front E))
                 l (same (add1 l))))")
        .unwrap()
    ;
}

pub fn with_chapter_context() -> Checker {
    CHAPTER_CONTEXT.clone()
}

#[test]
fn frame_48_donut_absurdity() {
    with_chapter_context()
        .claim(
            "donut-absurdity",
            "(-> (= Nat 0 6) (= Atom 'powdered 'glazed))",
        )
        .define(
            "donut-absurdity",
            "(λ (zero=six) (ind-Absurd (zero-not-add1 5 zero=six) (= Atom 'powdered 'glazed)))",
        )
        .unwrap();
}

#[test]
fn frame_51_prove_one_is_not_six() {
    with_chapter_context()
        .claim("one-not-six", "(-> (= Nat 1 6) Absurd)")
        .define(
            "one-not-six",
            "(λ (one=six) (zero-not-add1 4 (sub1= 0 5 one=six)))",
        )
        .unwrap();
}
