use crate::book::{chapter_06, with_empty_context, Checker, ResultBoolAssertions};

pub fn with_chapter_context() -> Checker {
    chapter_06::with_chapter_context()
        // ------
        //  last
        // ------
        .claim("last", "(Π ((E U) (l Nat)) (-> (Vec E (add1 l)) E))")
        .claim("base-last", "(Π ((E U)) (-> (Vec E 1) E))")
        .define("base-last", "(λ (E) (λ (es) (head es)))")
        .unwrap()
        .claim("mot-last", "(-> U Nat U)")
        .define("mot-last", "(λ (E k) (-> (Vec E (add1 k)) E))")
        .unwrap()
        .claim(
            "step-last",
            "(Π ((E U) (l-1 Nat)) (-> (mot-last E l-1) (mot-last E (add1 l-1))))",
        )
        .define(
            "step-last",
            "(λ (E l-1) (λ (last_l-1) (λ (es) (last_l-1 (tail es)))))",
        )
        .unwrap()
        .define(
            "last",
            "(λ (E l) (ind-Nat l (mot-last E) (base-last E) (step-last E)))",
        )
        .unwrap()
        // -----------
        //  drop-last
        // -----------
        .claim(
            "drop-last",
            "(Π ((E U) (l Nat)) (-> (Vec E (add1 l)) (Vec E l)))",
        )
        .claim("base-drop-last", "(Π ((E U)) (-> (Vec E 1) (Vec E 0)))")
        .define("base-drop-last", "(λ (E es) vecnil)")
        .unwrap()
        .claim("mot-drop-last", "(-> U Nat U)")
        .define("mot-drop-last", "(λ (E k) (-> (Vec E (add1 k)) (Vec E k)))")
        .unwrap()
        .claim(
            "step-drop-last",
            "(Π ((E U) (l-1 Nat)) (-> (mot-drop-last E l-1) (mot-drop-last E (add1 l-1))))",
        )
        .define(
            "step-drop-last",
            "(λ (E l-1) (λ (drop-last_l-1) (λ (es) (vec:: (head es) (drop-last_l-1 (tail es))))))",
        )
        .unwrap()
        .define(
            "drop-last",
            "(λ (E l) (ind-Nat l (mot-drop-last E) (base-drop-last E) (step-drop-last E)))",
        )
        .unwrap()
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

#[test]
fn test_55_56_also_rec_nat() {
    with_chapter_context()
        .core("(last Atom 1 (vec:: 'carrot (vec:: 'celery vecnil)))")
        .and("'celery")
        .are_the_same("Atom")
        .assert(true);
}

/* Frames 70 - 72 performed manually:
    0. | (drop-last Atom 2)
    1. | (drop-last Atom (add1 (add1 zero)))
    2. | (ind-Nat (add1 (add1 zero))
       |   (mot-drop-last Atom)
       |   (base-drop-last Atom)
       |   (step-drop-last Atom))
    3. | (step-drop-last Atom (add1 zero)
       |   (ind-Nat (add1 zero)
       |     (mot-drop-last Atom)
       |     (base-drop-last Atom)
       |     (step-drop-last Atom)))
    4. | (λ (es)
       |    (vec:: (head es)
       |           ((ind-Nat (add1 zero)
       |              (mot-drop-last Atom)
       |              (base-drop-last Atom)
       |              (step-drop-last Atom))
       |            (tail es))))

    5. | (λ (es)
       |    (vec:: (head es)
       |           ((step-drop-last Atom zero
       |              (ind-Nat zero
       |                (mot-drop-last Atom)
       |                (base-drop-last Atom)
       |                (step-drop-last Atom)))
       |            (tail es))))
    6. | (λ (es)
       |    (vec:: (head es)
       |           ((λ (es)
       |               (vec:: (head es)
       |                      ((ind-Nat zero
       |                         (mot-drop-last Atom)
       |                         (base-drop-last Atom)
       |                         (step-drop-last Atom))
       |                       (tail es))))
       |            (tail es))))
    7. | (λ (es)
       |    (vec:: (head es)
       |           (vec:: (head (tail es))
       |                  ((ind-Nat zero
       |                     (mot-drop-last Atom)
       |                     (base-drop-last Atom)
       |                     (step-drop-last Atom))
       |                   (tail (tail es)))))
       |            )))
    8. | (λ (es)
       |    (vec:: (head es)
       |           (vec:: (head (tail es))
       |                  ((base-drop-last Atom)
       |                   (tail (tail es)))))
       |            )))
    9. | (λ (es)
       |    (vec:: (head es)
       |           (vec:: (head (tail es))
       |                  vecnil)))
*/

#[test]
fn test_70_27_drop_last() {
    with_chapter_context()
        .core("(drop-last Atom 2)")
        .and("(λ (es) (vec:: (head es) (vec:: (head (tail es)) vecnil)))")
        .are_the_same("(-> (Vec Atom 3) (Vec Atom 2))")
        .assert(true)
}
