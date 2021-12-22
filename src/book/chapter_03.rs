use crate::book::{with_empty_context, ResultBoolAssertions};

#[test]
fn test_020_simple_iter_nat_example() {
    with_empty_context()
        .core("(iter-Nat 5 3 (lambda (smaller) (add1 smaller)))")
        .and("8")
        .are_the_same("Nat")
        .assert(true);
}
