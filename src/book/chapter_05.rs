use crate::book::common_definitions::with_book_context;
use crate::book::Checker;

fn with_chapter_context() -> Checker {
    with_book_context()
}

#[test]
fn test_03_list_of_expectations() {
    with_chapter_context()
        .claim("expectations", "(List Atom)")
        .define("expectations", "(:: 'understood (:: 'slept nil))")
        .unwrap();
}
