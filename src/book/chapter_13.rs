use crate::book::{chapter_12, Checker, ResultBoolAssertions};
use lazy_static::lazy_static;

lazy_static! {
    static ref CHAPTER_CONTEXT: Checker = chapter_12::with_chapter_context();
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
