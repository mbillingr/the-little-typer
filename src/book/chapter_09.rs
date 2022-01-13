use crate::book::common_definitions::with_book_context;
use crate::book::{Checker, ResultBoolAssertions};

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
        // -------
        //  twice
        // -------
        .claim("twice", "(-> Nat Nat)")
        .define("twice", "(λ (n) (+ n n))")
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

#[test]
fn frame_22_67_prove_that_twice_equals_double_then_use_that_to_define_twicevec() {
    let ctx = with_chapter_context()
        .claim(
            "twice=double",
            "(Π ((n Nat))
                (= Nat
                   (twice n)
                   (double n)))",
        )
        .claim(
            "add1+=+add1",
            "(Π ((n Nat) (j Nat))
                (= Nat
                   (add1 (+ n j))
                   (+ n (add1 j))))",
        )
        .claim("mot-add1+=+add1", "(-> Nat Nat U)")
        .define(
            "mot-add1+=+add1",
            "(λ (j k) (= Nat (add1 (+ k j)) (+ k (add1 j))))",
        )
        .unwrap()
        .claim(
            "step-add1+=+add1",
            "(Π ((j Nat) (n-1 Nat)) (-> (mot-add1+=+add1 j n-1) (mot-add1+=+add1 j (add1 n-1))))",
        )
        .define(
            "step-add1+=+add1",
            "(λ (j n-1 add1+=+add1_n-1) (cong add1+=+add1_n-1 (+ 1)))",
        )
        .unwrap()
        .define(
            "add1+=+add1",
            "(λ (n j) (ind-Nat n (mot-add1+=+add1 j) (same (add1 j)) (step-add1+=+add1 j)))",
        )
        .unwrap()
        .claim("mot-twice=double", "(-> Nat U)")
        .define("mot-twice=double", "(λ (k) (= Nat (twice k) (double k)))")
        .unwrap()
        .claim(
            "step-twice=double",
            "(Π ((n-1 Nat)) (-> (mot-twice=double n-1) (mot-twice=double (add1 n-1))))",
        )
        .claim(
            "mot-step-twice=double",
            "(-> Nat Nat U)",
        )
        .define(
            "mot-step-twice=double",
            "(λ (n-1 k) (= Nat (add1 k) (add1 (add1 (double n-1)))))",
        ).unwrap()
        .define(
            "step-twice=double",
            "(λ (n-1 twice=double_n-1) (replace (add1+=+add1 n-1 n-1) (mot-step-twice=double n-1) (cong twice=double_n-1 (+ 2))))",
        )
        .unwrap()
        .define(
            "twice=double",
            "(λ (n) (ind-Nat n mot-twice=double (same zero) step-twice=double))",
        )
        .unwrap();

    let ctx = ctx
        .claim("twice=double-of-17", "(= Nat (twice 17) (double 17))")
        .claim("twice=double-of-17-again", "(= Nat (twice 17) (double 17))")
        .define("twice=double-of-17", "(twice=double 17)")
        .unwrap()
        .define("twice=double-of-17-again", "(same 34)")
        .unwrap();

    ctx.core("(twice=double 17)")
        .and("(same 34)")
        .are_the_same("(= Nat (twice 17) (double 17))")
        .assert(true);

    let _ctx = ctx
        .claim(
            "twice-Vec",
            "(Π ((E U) (l Nat)) (-> (Vec E l) (Vec E (twice l))))",
        )
        .claim(
            "double-Vec",
            "(Π ((E U) (l Nat)) (-> (Vec E l) (Vec E (double l))))",
        )
        .claim(
            "base-double-Vec",
            "(Π ((E U)) (-> (Vec E zero) (Vec E (double zero))))",
        )
        .define("base-double-Vec", "(λ (E es) vecnil)")
        .unwrap()
        .claim("mot-double-Vec", "(-> U Nat U)")
        .define(
            "mot-double-Vec",
            "(λ (E k) (-> (Vec E k) (Vec E (double k))))",
        )
        .unwrap()
        .claim(
            "step-double-Vec",
            "(Π ((E U) (l-1 Nat))
                (-> (-> (Vec E l-1)
                        (Vec E (double l-1)))
                    (-> (Vec E (add1 l-1))
                        (Vec E (double (add1 l-1))))))",
        )
        .define(
            "step-double-Vec",
            "(λ (E l-1 double-Vec_l-1 es)
                (vec:: (head es)
                       (vec:: (head es)
                              (double-Vec_l-1 (tail es)))))",
        )
        .unwrap()
        .define(
            "double-Vec",
            "(λ (E l)
                (ind-Nat l
                    (mot-double-Vec E)
                    (base-double-Vec E)
                    (step-double-Vec E)))",
        )
        .unwrap()
        .define(
            "twice-Vec",
            "(λ (E l es)
                (replace (symm (twice=double l))
                         (λ (k) (Vec E k))
                         (double-Vec E l es)))",
        )
        .unwrap();
}
