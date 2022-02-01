use crate::book::{chapter_15, Checker};
use lazy_static::lazy_static;

lazy_static! {
    static ref CHAPTER_CONTEXT: Checker = chapter_15::with_chapter_context()
        // -------
        //  zero?
        // -------
        .claim("zero?", "(Π ((j Nat)) (Dec (= Nat zero j)))")
        .define(
            "zero?",
            "(λ (j) (ind-Nat j
                        (λ (k) (Dec (= Nat zero k)))
                        (left (same zero))
                        (λ (j-1 zero?_n-1) (right (zero-not-add1 j-1)))))"
        )
        .unwrap()
        // ---------------
        //  add1-not-zero
        // ---------------
        .claim(
            "add1-not-zero",
            "(Π ((n Nat)) (-> (= Nat (add1 n) zero) Absurd))"
        )
        .define(
            "add1-not-zero",
            "(λ (n) (use-Nat= (add1 n) zero))"
        )
        .unwrap()
        // -----------
        //  dec-add1=
        // -----------
        .claim(
            "dec-add1=",
            "(Π ((n-1 Nat) (j-1 Nat))
                (-> (Dec (= Nat n-1 j-1)) (Dec (= Nat (add1 n-1) (add1 j-1)))))"
        )
        .define(
            "dec-add1=",
            "(λ (n-1 j-1 eq-or-not)
                (ind-Either eq-or-not
                    (λ (target) (Dec (= Nat (add1 n-1) (add1 j-1))))
                    (λ (yes) (left (cong yes (+ 1))))
                    (λ (no) (right (λ (n=j) (no (sub1= n-1 j-1 n=j)))))))"
        )
        .unwrap()
        // -------
        //  nat=?
        // -------
        .claim("nat=?", "(Π ((n Nat) (j Nat)) (Dec (= Nat n j)))")
        .claim("mot-nat=?", "(-> Nat U)")
        .define("mot-nat=?", "(λ (k) (Π ((j Nat)) (Dec (= Nat k j))))")
        .unwrap()
        .claim("step-nat=?", "(Π ((n-1 Nat)) (-> (mot-nat=? n-1) (mot-nat=? (add1 n-1))))")
        .define(
            "step-nat=?",
            "(λ (n-1 nat=?_n-1 j)
                (ind-Nat j
                    (λ (k) (Dec (= Nat (add1 n-1) k)))
                    (right (add1-not-zero n-1))
                    (λ (j-1 nat=?_j-1) (dec-add1= n-1 j-1 (nat=?_n-1 j-1)))))")
        .unwrap()
        .define(
            "nat=?",
            "(λ (n j) ((ind-Nat n
                        mot-nat=?
                        zero?
                        step-nat=?)
                       j))"
        )
        .unwrap();
}

pub fn with_chapter_context() -> Checker {
    CHAPTER_CONTEXT.clone()
}

#[test]
fn no_frame() {
    // need to invoke the context to check the proofs
    with_chapter_context();
}
