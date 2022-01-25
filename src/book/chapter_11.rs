use crate::book::{chapter_10, Checker, ResultBoolAssertions};

pub fn with_chapter_context() -> Checker {
    chapter_10::with_chapter_context()
        // -----------
        //  list->vec
        // -----------
        .claim(
            "list->vec",
            "(Π ((E U) (es (List E))) (Vec E (length E es)))",
        )
        .claim("mot-list->vec", "(Π ((E U)) (-> (List E) U))")
        .define("mot-list->vec", "(λ (E es) (Vec E (length E es)))")
        .unwrap()
        .claim(
            "step-list->vec",
            "(Π ((E U) (e E) (es (List E))) (-> (mot-list->vec E es) (mot-list->vec E (:: e es))))",
        )
        .define(
            "step-list->vec",
            "(λ (E e es list->vec_es) (vec:: e list->vec_es))",
        )
        .unwrap()
        .define(
            "list->vec",
            "(λ (E es) (ind-List es (mot-list->vec E) vecnil (step-list->vec E)))",
        )
        .unwrap()
        // -----------
        //  vec->list
        // -----------
        .claim("vec->list", "(Π ((E U) (l Nat)) (-> (Vec E l) (List E)))")
        .claim("mot-vec->list", "(Π ((E U) (l Nat)) (-> (Vec E l) U))")
        .define("mot-vec->list", "(λ (E l es) (List E))")
        .unwrap()
        .claim(
            "step-vec->list",
            "(Π ((E U) (l-1 Nat) (e E) (es (Vec E l-1)))
                (-> (mot-vec->list E l-1 es) (mot-vec->list E (add1 l-1) (vec:: e es))))",
        )
        .define(
            "step-vec->list",
            "(λ (E l-1 e es vec->list_es) (:: e vec->list_es))",
        )
        .unwrap()
        .define(
            "vec->list",
            "(λ (E l es) (ind-Vec l es (mot-vec->list E) nil (step-vec->list E)))",
        )
        .unwrap()
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

#[test]
fn frame_35_55_the_external_proof() {
    let _ctx = with_chapter_context()
        .claim(
            "list->vec->list=",
            "(Π ((E U) (es (List E)))
            (= (List E)
               es
               (vec->list E
                   (length E es)
                   (list->vec E es))))",
        )
        .claim("mot-list->vec->list=", "(Π ((E U)) (-> (List E) U))")
        .define(
            "mot-list->vec->list=",
            "(λ (E es) (= (List E) es (vec->list E (length E es) (list->vec E es))))",
        )
        .unwrap()
        .claim(
            "step-list->vec->list=",
            "(Π ((E U) (e E) (es (List E)))
                (-> (mot-list->vec->list= E es) (mot-list->vec->list= E (:: e es))))",
        )
        .claim("::-fun", "(Π ((E U)) (-> E (List E) (List E)))")
        .define("::-fun", "(λ (E e es) (:: e es))")
        .unwrap()
        .define(
            "step-list->vec->list=",
            "(λ (E e es list->vec->list=_es) (cong list->vec->list=_es (::-fun E e)))",
        )
        .unwrap()
        .define(
            "list->vec->list=",
            "(λ (E es)
                (ind-List es (mot-list->vec->list= E) (same nil) (step-list->vec->list= E)))",
        )
        .unwrap();
}

#[test]
fn frame_45_46_proving_the_treat_statement() {
    let _ctx = with_chapter_context()
        .claim("Treat-Statement", "U")
        .define(
            "Treat-Statement",
            "(Π ((some-treats (List Atom))
                 (more-treats (List Atom)))
                (-> (= (List Atom) some-treats more-treats)
                    (= (List Atom) (:: 'plättar some-treats) (:: 'plättar more-treats))))",
        )
        .unwrap()
        .claim("::-plättar", "(-> (List Atom) (List Atom))")
        .define(
            "::-plättar",
            "(λ (tasty-treats) (:: 'plättar tasty-treats))",
        )
        .unwrap()
        .claim("treat-proof", "Treat-Statement")
        .define(
            "treat-proof",
            "(λ (some-treats more-treats treats=) (cong treats= ::-plättar))",
        );
}

#[test]
fn frame_48_proving_two_equal_lists_have_equal_length() {
    let _ctx = with_chapter_context()
        .claim(
            "length-treats=",
            "(Π ((some-treats (List Atom))
                 (more-treats (List Atom)))
                (-> (= (List Atom) some-treats more-treats)
                    (= Nat (length Atom some-treats) (length Atom more-treats))))",
        )
        .define(
            "length-treats=",
            "(λ (some-treats more-treats treats=) (cong treats= (length Atom)))",
        )
        .unwrap();
}
