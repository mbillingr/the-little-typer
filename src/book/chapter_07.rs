use crate::book::common_definitions::with_book_context;
use crate::book::{with_empty_context, Checker, ResultBoolAssertions};

fn with_chapter_context() -> Checker {
    with_book_context()
}

#[test]
fn test_12_20_the_vec_type() {
    with_chapter_context()
        .claim("mot-peas", "(-> Nat U)")
        .define("mot-peas", "(λ (k) (Vec Atom k))")
        .unwrap()
        .claim(
            "step-peas",
            "(Π ((l-1 Nat)) (-> (mot-peas l-1) (mot-peas (add1 l-1)) ))",
        )
        .define(
            "step-peas",
            "(λ (l-1) (λ (peas_l-1) (vec:: 'pea peas_l-1)))",
        )
        .unwrap()
        .claim("peas", "(Π ((l Nat)) (Vec Atom l))")
        .define(
            "peas",
            "(λ (how-many-peas) (ind-Nat how-many-peas mot-peas vecnil step-peas))",
        )
        .unwrap()
        .core("(peas 2)")
        .and("(vec:: 'pea (vec:: 'pea vecnil))")
        .are_the_same("(Vec Atom 2)")
        .assert(true);
}

#[test]
fn test_27_also_rec_nat() {
    with_empty_context()
        .claim(
            "also-rec-Nat",
            "(Π ((X U) (target Nat) (base X) (step (-> Nat X X))) X)",
        )
        .define(
            "also-rec-Nat",
            "(λ (X target base step) (ind-Nat target (λ (k) X) base step))",
        )
        .unwrap();
}
