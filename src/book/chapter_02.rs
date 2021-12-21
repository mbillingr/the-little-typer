use crate::book::{with_empty_context, ResultBoolAssertions};

#[test]
fn test_016_functions_types_can_be_computed() {
    with_empty_context()
        .core("(-> Atom (Pair Atom Atom))")
        .and(
            "(-> (car (the (Pair U Atom)
                            (cons Atom 'pepper)))
                  (Pair (cdr (the (Pair Atom U)
                                  (cons 'salt Atom)))
                        Atom))",
        )
        .are_the_same_type()
}

#[test]
fn test_019_different_functions_with_same_body_are_the_same() {
    with_empty_context()
        .core("(lambda (x) (cons x x))")
        .and("(lambda (y) (cons y y))")
        .are_the_same("(-> Nat (Pair Nat Nat))")
        .assert(true);

    with_empty_context()
        .core("(lambda (a d) (cons a d))")
        .and("(lambda (d a) (cons a d))")
        .are_the_same("(-> Nat Nat (Pair Nat Nat))")
        .assert(false);
}

#[test]
fn test_initial_second_commandment_of_lambda() {
    // I interpret it as 'Two functions are the same if they behave the same'
    with_empty_context()
        .claim("f", "(-> Nat (Pair Nat Nat))")
        .define("f", "(lambda (x) (cons x x))")
        .unwrap()
        .core("f")
        .and("(lambda (y) (f y))")
        .are_the_same("(-> Nat (Pair Nat Nat))")
        .assert(true);
}

#[test]
fn test_035_define_names() {
    with_empty_context()
        .claim("vegetables", "(Pair Atom Atom)")
        .define("vegetables", "(cons 'celery 'carrot)")
        .unwrap()
        .core("vegetables")
        .and("(cons 'celery 'carrot)")
        .are_the_same("(Pair Atom Atom)")
        .assert(true);
}

#[test]
fn test_037_consing_the_parts_of_a_pair_yields_the_same_pair() {
    with_empty_context()
        .claim("vegetables", "(Pair Atom Atom)")
        .define("vegetables", "(cons 'celery 'carrot)")
        .unwrap()
        .core("vegetables")
        .and("(cons (car vegetables) (cdr vegetables))")
        .are_the_same("(Pair Atom Atom)")
        .assert(true);
}

#[test]
fn test_046_which_nat_zero() {
    with_empty_context()
        .core("(which-Nat zero 'naught (lambda (x) 'more))")
        .and("'naught")
        .are_the_same("Atom")
        .assert(true);
}

#[test]
fn test_049_which_nat_nonzero() {
    with_empty_context()
        .core("(which-Nat 4 'naught (lambda (x) 'more))")
        .and("'more")
        .are_the_same("Atom")
        .assert(true);
}

#[test]
fn test_075_we_can_cons_types() {
    with_empty_context()
        .core("(cons Atom Atom)")
        .is_a("(Pair U U)");
}

#[test]
fn test_pears() {
    // frame 82: pears are pairs of nats
    let chk = with_empty_context()
        .claim("Pear", "U")
        .define("Pear", "(Pair Nat Nat)")
        .unwrap();
    chk.core("(cons 3 5)").is_a("Pear");

    // frame 93: Pear-maker and elim-Pear
    let chk = chk
        .claim("Pear-maker", "U")
        .define("Pear-maker", "(-> Nat Nat Pear)")
        .unwrap()
        .claim("elim-Pear", "(-> Pear Pear-maker Pear)")
        .define(
            "elim-Pear",
            "(lambda (pear maker) (maker (car pear) (cdr pear)))",
        )
        .unwrap();

    // frame 95: elim-Pear applied
    chk.core("(elim-Pear (cons 3 17) (lambda (a d) (cons d a)))")
        .and("(cons 17 3)")
        .are_the_same("Pear")
        .assert(true);

    // frame 100: pearwise+ (todo)
}
