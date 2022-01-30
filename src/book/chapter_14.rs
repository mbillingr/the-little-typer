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
        // -----
        //  Fin
        // -----
        .claim("Fin", "(-> Nat U)")
        .define("Fin", "(λ (n) (iter-Nat n Absurd Maybe))")
        .unwrap()
        // -------
        //  fzero
        // -------
        .claim("fzero", "(Π ((n Nat)) (Fin (add1 n)))")
        .define("fzero", "(λ (n) (nothing (Fin n)))")
        .unwrap()
        // -------
        //  add1
        // -------
        .claim("fadd1", "(Π ((n Nat)) (-> (Fin n) (Fin (add1 n))))")
        .define("fadd1", "(λ (n i-1) (just (Fin n) i-1))")
        .unwrap()
        // ---------
        //  vec-ref
        // ---------
        .claim("vec-ref", "(Π ((E U) (l Nat)) (-> (Fin l) (Vec E l) E))")
        .claim("base-vec-ref", "(Π ((E U)) (-> (Fin 0) (Vec E 0) E))")
        .define("base-vec-ref", "(λ (E no-value-ever es) (ind-Absurd no-value-ever E))")
        .unwrap()
        .claim(
            "step-vec-ref",
            "(Π ((E U) (l-1 Nat))
                (-> (-> (Fin l-1) (Vec E l-1) E)
                    (-> (Fin (add1 l-1)) (Vec E (add1 l-1)) E)))")
        .define(
            "step-vec-ref",
            "(λ (E l-1 vec-ref_l-1 i es)
                (ind-Either i
                    (λ (i) E)
                    (λ (i-1) (vec-ref_l-1 i-1 (tail es)))
                    (λ (triv) (head es))))")
        .unwrap()
        .define(
            "vec-ref",
            "(λ (E l)
                (ind-Nat l
                    (λ (k) (-> (Fin k) (Vec E k) E))
                    (base-vec-ref E)
                    (step-vec-ref E)))")
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

#[test]
fn frame_51_normal_form_of_fin() {
    with_chapter_context()
        .core("(Fin 1)")
        .and("(Either Absurd Trivial)")
        .are_the_same("U")
        .assert(true)
}

#[test]
fn frame_52_normal_form_of_fin() {
    with_chapter_context()
        .core("(Fin 2)")
        .and("(Either (Either Absurd Trivial) Trivial)")
        .are_the_same("U")
        .assert(true)
}

#[test]
fn frame_73_vec_ref() {
    with_chapter_context()
        .core("(vec-ref Atom 3 (fadd1 2 (fzero 1)) (vec:: 'a (vec:: 'b (vec:: 'c vecnil))))")
        .and("'b")
        .are_the_same("Atom")
        .assert(true)
}
