use crate::book::{chapter_13, Checker, ResultBoolAssertions};
use lazy_static::lazy_static;

lazy_static! {
    static ref CHAPTER_CONTEXT: Checker = chapter_13::with_chapter_context()
        // -------
        //  Maybe
        // -------
        .claim("Maybe", "(-> U U)")
        .define("Maybe", "(λ (T) (Either T Trivial))")
        .unwrap()
        // ---------
        //  nothing
        // ---------
        .claim("nothing", "(Π ((E U)) (Maybe E))")
        .define("nothing", "(λ (E) (right sole))")
        .unwrap()
        // ------
        //  just
        // ------
        .claim("just", "(Π ((E U)) (-> E (Maybe E)))")
        .define("just", "(λ (E e) (left e))")
        .unwrap()
        // ------------
        //  maybe-head
        // ------------
        .claim("maybe-head", "(Π ((E U)) (-> (List E) (Maybe E)))")
        .define("maybe-head", "(λ (E es) (rec-List es (nothing E) (λ (hd tl head_tl) (just E hd))))")
        .unwrap()
        // ------------
        //  maybe-tail
        // ------------
        .claim("maybe-tail", "(Π ((E U)) (-> (List E) (Maybe (List E))))")
        .define("maybe-tail", "(λ (E es) (rec-List es (nothing (List E)) (λ (hd tl head_tl) (just (List E) tl))))")
        .unwrap()
        // ----------
        //  list-ref
        // ----------
        .claim("list-ref", "(Π ((E U)) (-> Nat (List E) (Maybe E)))")
        .claim("step-list-ref", "(Π ((E U)) (-> Nat (-> (List E) (Maybe E)) (-> (List E) (Maybe E))))")
        .define(
            "step-list-ref",
            "(λ (E n list-ref_n-1 es)
                (ind-Either (maybe-tail E es)
                    (λ (maybe_tl) (Maybe E))
                    (λ (tl) (list-ref_n-1 tl))
                    (λ (empty) (nothing E))))")
        .unwrap()
        .define("list-ref", "(λ (E n) (rec-Nat n (maybe-head E) (step-list-ref E)))")
        .unwrap()
    ;
}

pub fn with_chapter_context() -> Checker {
    CHAPTER_CONTEXT.clone()
}

#[test]
fn frame_18_reference_empty_list() {
    with_chapter_context()
        .core("(list-ref Atom 0 nil)")
        .and("(nothing Atom)")
        .are_the_same("(Maybe Atom)")
        .assert(true)
}

#[test]
fn frame_19_reference_first_item() {
    with_chapter_context()
        .core("(list-ref Atom 0 (:: 'a (:: 'b nil)))")
        .and("(just Atom 'a)")
        .are_the_same("(Maybe Atom)")
        .assert(true)
}
