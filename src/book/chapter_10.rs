use crate::book::common_definitions::with_book_context;
use crate::book::{with_empty_context, Checker, ResultBoolAssertions};

fn with_chapter_context() -> Checker {
    with_book_context()
        .claim("replicate", "(Π ((E U) (n Nat) (e E)) (Vec E n))")

        .claim(
        "list->vec",
        "(Π ((E U)) (-> (List E) (Σ ((l Nat)) (Vec E l))))",
    )
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
