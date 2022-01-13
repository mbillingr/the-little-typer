use crate::book::common_definitions::with_book_context;
use crate::book::{with_empty_context, Checker, ResultAssertions, ResultBoolAssertions};

fn with_chapter_context() -> Checker {
    with_book_context()
        // ------
        //  incr
        // ------
        .claim("incr", "(-> Nat Nat)")
        .define("incr", "(λ (n) (iter-Nat n 1 (+ 1)))")
        .unwrap()
        // --------
        //  double
        // --------
        .claim("double", "(-> Nat Nat)")
        .define("double", "(λ (n) (iter-Nat n 0 (+ 2)))")
        .unwrap()
}

#[test]
fn frame_19_use_replace_instead_of_cong() {
    let ctx = with_chapter_context()
        .claim("incr=add1", "(Π ((n Nat)) (= Nat (incr n) (add1 n)))")
        .claim("base-incr=add1", "(= Nat (incr zero) (add1 zero))")
        .define("base-incr=add1", "(same (add1 zero))")
        .unwrap()
        .claim("mot-incr=add1", "(-> Nat U)")
        .define("mot-incr=add1", "(λ (k) (= Nat (incr k) (add1 k)))")
        .unwrap()
        .claim(
            "step-incr=add1",
            "(Π ((n-1 Nat)) \
                (-> (= Nat (incr n-1) (add1 n-1)) \
                    (= Nat (add1 (incr n-1)) (add1 (add1 n-1)))))",
        )
        .claim("mot-step-incr=add1", "(-> Nat Nat U)")
        .define("mot-step-incr=add1", "(λ (n-1 k) (= Nat (add1 (incr n-1)) (add1 k)))").unwrap()
        .define(
            "step-incr=add1",
            "(λ (n-1) (λ (incr=add1_n-1) (replace incr=add1_n-1 (mot-step-incr=add1 n-1) (same (add1 (incr n-1))))))",
        )
        .unwrap()
        .define(
            "incr=add1",
            "(λ (n) (ind-Nat n mot-incr=add1 base-incr=add1 step-incr=add1))",
        )
        .unwrap();

    ctx.core("(incr=add1 2)").is_a("(= Nat 3 3)").assert(true);

    ctx.core("(incr=add1 2)")
        .and("(same 3)")
        .are_the_same("(= Nat 3 3)")
        .assert(true);
}
