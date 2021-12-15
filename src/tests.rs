use crate::basics::{ctx_to_env, Ctx};
use crate::normalize::val_of;
use crate::rep::rep;
use crate::types::{cores::*, values};
use lazy_static::lazy_static;

lazy_static! {
    static ref CTX: Ctx = Ctx::new();
}

#[test]
fn the_nat_zero_evaluates_to_zero() {
    assert_eq!(
        val_of(&ctx_to_env(&CTX), &the(nat(), zero())),
        values::zero()
    );
}

#[test]
fn function_types_are_sugar_for_simple_pi_types() {
    assert_eq!(
        rep(
            &CTX,
            &"(the (-> Nat Nat) (λ (my-var) my-var))".parse().unwrap()
        ),
        Ok(the(
            pi("x", nat(), nat()),
            lambda("my-var", refer("my-var"))
        ))
    );
}

#[test]
fn nary_functions_and_lamba_desugar_to_curried_unaries() {
    assert_eq!(
        rep(
            &CTX,
            &"(the (-> Nat Nat Nat) (lambda (x y) x))".parse().unwrap()
        ),
        Ok(the(
            pi("x", nat(), pi("x₁", nat(), nat())),
            lambda("x", lambda("x₁", refer("x")))
        ))
    )
}

#[test]
fn rightmost_lambda_param_takes_precedence() {
    assert_eq!(
        rep(
            &CTX,
            &"(the (-> Nat Nat Nat) (lambda (x x) x))".parse().unwrap()
        ),
        Ok(the(
            pi("x", nat(), pi("x₁", nat(), nat())),
            lambda("x", lambda("x₁", refer("x₁")))
        ))
    )
}
