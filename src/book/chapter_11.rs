use crate::book::common_definitions::with_book_context;
use crate::book::{Checker, ResultBoolAssertions};

fn with_chapter_context() -> Checker {
    with_book_context()
}

#[test]
fn frame_27_vec_append() {
    let ctx = with_chapter_context()
        .claim(
            "vec-append",
            "(Π ((E U) (l Nat) (j Nat)) (-> (Vec E l) (Vec E j) (Vec E (+ l j))) )",
        )
        .claim(
            "mot-vec-append",
            "(Π ((E U) (j Nat) (k Nat)) (-> (Vec E k) U))",
        )
        .define("mot-vec-append", "(λ (E j k es) (Vec E (+ k j)))")
        .unwrap()
        .claim(
            "step-vec-append",
            "(Π ((E U) (j Nat) (k Nat) (e E) (es (Vec E k)))
                (-> (mot-vec-append E j k es) (mot-vec-append E j (add1 k) (vec:: e es))))",
        )
        .define(
            "step-vec-append",
            "(λ (E j l-1 e es vec-append_es) (vec:: e vec-append_es))",
        )
        .unwrap()
        .define(
            "vec-append",
            "(λ (E l j es end) (ind-Vec l es (mot-vec-append E j) end (step-vec-append E j)))",
        )
        .unwrap();

    ctx.core("(vec-append Atom 1 1 (vec:: 'a vecnil) (vec:: 'b vecnil))")
        .and("(vec:: 'a (vec:: 'b vecnil))")
        .are_the_same("(Vec Atom 2)")
        .assert(true);
}
