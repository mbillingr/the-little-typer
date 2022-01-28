use crate::book::{chapter_13, Checker, ResultBoolAssertions};
use lazy_static::lazy_static;

lazy_static! {
    static ref CHAPTER_CONTEXT: Checker = chapter_13::with_chapter_context()
        // -------
        //  Maybe
        // -------
        .claim("Maybe", "(-> U U)")
        .define("Maybe", "(λ (T) (Either T Trivial))")
        .unwrap()
    ;
}

pub fn with_chapter_context() -> Checker {
    CHAPTER_CONTEXT.clone()
}

#[test]
fn frame___() {
    with_chapter_context();
}
