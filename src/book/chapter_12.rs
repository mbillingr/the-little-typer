use crate::book::{chapter_11, Checker, ResultBoolAssertions};

pub fn with_chapter_context() -> Checker {
    chapter_11::with_chapter_context()
        .claim("Even", "(-> Nat U)")
        .define("Even", "(λ (n) (Σ ((half Nat)) (= Nat n (double half))))")
        .unwrap()
}

#[test]
fn frame_00() {
    let ctx = with_chapter_context();
}
