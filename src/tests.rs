use crate::basics::{ctx_to_env, Ctx};
use crate::errors::Error;
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
            lambda("x", lambda("y", refer("x")))
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

#[test]
fn which_nat_applies_steps() {
    assert_eq!(
        rep(&CTX, &"(which-Nat 1 2 (lambda (x) x))".parse().unwrap()),
        Ok(the(nat(), zero()))
    )
}

#[test]
fn which_nat_zero_returns_base() {
    assert_eq!(
        rep(&CTX, &"(which-Nat 0 2 (lambda (x) x))".parse().unwrap()),
        Ok(the(nat(), add1(add1(zero()))))
    )
}

#[test]
fn a_type_error() {
    assert_eq!(
        rep(
            &CTX,
            &"(the (-> Nat Nat Nat) (lambda (x) x))".parse().unwrap()
        ),
        Err(Error::WrongType(nat(), pi("x₁", nat(), nat())))
    )
}

#[test]
fn pass_nat_as_function_error() {
    assert_eq!(
        rep(
            &CTX,
            &"(the (-> (-> Nat Nat) Nat) \
                      (lambda (f x) (f x)))"
                .parse()
                .unwrap()
        ),
        Err(Error::NotAFunctionType(nat()))
    )
}

#[test]
fn some_higher_order_identity_thing() {
    assert_eq!(
        rep(
            &CTX,
            &"(the (-> (-> Nat Nat) Nat Nat) (lambda (f x) (f x)))"
                .parse()
                .unwrap()
        ),
        Ok(the(
            pi("x", pi("x", nat(), nat()), pi("x₁", nat(), nat())),
            lambda("f", lambda("x", app(refer("f"), refer("x"))))
        ))
    )
}

#[test]
fn which_nat_resolves_constant_target_inside_a_function() {
    assert_eq!(
        rep(
            &CTX,
            &"(the (-> Nat (-> Nat Nat) Nat) (lambda (x f) (which-Nat 2 x f)))"
                .parse()
                .unwrap()
        ),
        Ok(the(
            pi("x", nat(), pi("x₁", pi("x₁", nat(), nat()), nat())),
            lambda("x", lambda("f", app(refer("f"), add1(zero()))))
        ))
    )
}

#[test]
fn which_nat_stays_unresolved_if_target_is_neutral() {
    assert_eq!(
        rep(
            &CTX,
            &"(the (-> Nat (-> Nat Nat) Nat) (lambda (x f) (which-Nat x (add1 (add1 zero)) f)))"
                .parse()
                .unwrap()
        ),
        Ok(the(
            pi("x", nat(), pi("x₁", pi("x₁", nat(), nat()), nat())),
            lambda(
                "x",
                lambda(
                    "f",
                    which_nat(
                        refer("x"),
                        the(nat(), add1(add1(zero()))),
                        //lambda("n", app(refer("f"), refer("n")))  -- the reference implementation has this redundant lambda in... no idea why we don't have it
                        refer("f")
                    )
                )
            )
        ))
    )
}

#[test]
fn u_has_no_type() {
    assert_eq!(rep(&CTX, &universe()), Err(Error::UhasNoType))
}

#[test]
fn a_simple_explicit_pi_type() {
    assert_eq!(
        rep(
            &CTX,
            &"(the (Pi ((A U)) U) (lambda (B) B))".parse().unwrap()
        ),
        Ok(the(
            pi("A", universe(), universe()),
            lambda("B", refer("B"))
        ))
    )
}

#[test]
fn a_complexer_explicit_pi_type() {
    assert_eq!(
        rep(
            &CTX,
            &"(the (Pi ((A U) (a A)) A) (lambda (B b) b))"
                .parse()
                .unwrap()
        ),
        Ok(the(
            pi("A", universe(), pi("a", refer("A"), refer("A"))),
            lambda("B", lambda("b", refer("b")))
        ))
    )
}
