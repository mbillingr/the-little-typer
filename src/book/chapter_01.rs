use crate::basics::{Core, Ctx};
use crate::book::in_context;

use Core::*;

use lazy_static::lazy_static;

lazy_static! {
    static ref CTX: Ctx = Ctx::new();
}

#[test]
fn test_002_a_quote_is_an_atom() {
    assert!(in_context(&CTX).core("'atom").is_a(&Atom));
}

/*#[test]
fn  test_019_the_result_of_cons_is_a_pair() {
    assert!(in_context(&CTX).core("(cons 'ratatuille 'baguette)").is_a(&Core::cons(Atom, Atom)));
}*/
