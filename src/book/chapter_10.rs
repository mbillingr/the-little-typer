use crate::book::common_definitions::with_book_context;
use crate::book::{with_empty_context, Checker, Result, ResultBoolAssertions};

fn with_chapter_context() -> Checker {
    with_book_context()
        // -----------
        //  replicate
        // -----------
        .claim("replicate", "(Π ((E U) (n Nat) (e E)) (Vec E n))")
        .define(
            "replicate",
            "(λ (E n e)
                (ind-Nat n
                    (λ (k) (Vec E k))
                    vecnil
                    (λ (n-1 replicate_n-1) (vec:: e replicate_n-1))))",
        )
        .unwrap()
    // -----------
    //  list->vec
    // -----------
}

#[test]
fn frame_08_cons_is_a_sigma() {
    with_chapter_context()
        .core("(cons 'bagel (same 'bagel))")
        .is_a("(Σ ((bread Atom)) (= Atom bread 'bagel))")
        .assert(true);

    with_empty_context()
        .core("(cons Nat 4)")
        .is_a("(Σ ((A U)) A)")
        .assert(true);

    with_empty_context()
        .core("(cons Atom 'porridge)")
        .is_a("(Σ ((A U)) A)")
        .assert(true);

    with_book_context()
        .core("(cons (-> Nat Nat) (+ 7))")
        .is_a("(Σ ((A U)) A)")
        .assert(true);

    with_empty_context()
        .core("(cons 'toast (same (:: 'toast nil)))")
        .is_a("(Σ ((food Atom)) (= (List Atom) (:: food nil) (:: 'toast nil)))")
        .assert(true);

    with_empty_context()
        .core("(cons 2 (vec:: 'toast-and-jam (vec:: 'tea vecnil)))")
        .is_a("(Σ ((l Nat)) (Vec Atom l))")
        .assert(true);
}

#[test]
fn frame_18_prove_there_is_a_list_thats_itself_reversed() {
    let the_statement = "(Σ ((es (List Atom))) (= (List Atom) es (reverse Atom es)))";
    with_book_context()
        .core("(cons nil (same nil))")
        .is_a(the_statement)
        .assert(true);

    with_book_context()
        .core("(cons (:: 'a (:: 'b (:: 'a nil))) (same (:: 'a (:: 'b (:: 'a nil)))))")
        .is_a(the_statement)
        .assert(true);
}

fn define_silly_list_to_vec(ctx: Checker) -> Result<Checker> {
    ctx.claim(
        "copy-52-times",
        "(Π ((E U)) (-> E (List E) (Σ ((l Nat)) (Vec E l)) (Σ ((l Nat)) (Vec E l))))",
    )
    .define(
        "copy-52-times",
        "(λ (E e es copy_es) (cons 52 (replicate E 52 e)))",
    )
    .unwrap()
    .define(
        "list->vec",
        "(λ (E es) (rec-List es
                                 (the (Σ ((l Nat)) (Vec E l))
                                      (cons 0 vecnil))
                                 (copy-52-times E)))",
    )
}

#[test]
fn frame_52_too_general_type_allows_silly_impls() {
    let ctx = with_chapter_context().claim(
        "list->vec",
        "(Π ((E U)) (-> (List E) (Σ ((l Nat)) (Vec E l))))",
    );
    define_silly_list_to_vec(ctx).unwrap();
}

#[test]
fn frame_54_specific_claim_disallows_silly_impl() {
    let ctx = with_chapter_context().claim(
        "list->vec",
        "(Π ((E U) (es (List E))) (Vec E (length E es)))",
    );
    assert!(define_silly_list_to_vec(ctx).is_err());
}
