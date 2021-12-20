use crate::basics::{CoreInterface, Ctx};
use crate::errors::Error;
use crate::rep::{norm_type, rep};
use crate::types::{cores::*, values};
use lazy_static::lazy_static;

lazy_static! {
    static ref CTX: Ctx = Ctx::new();
}

#[test]
fn the_nat_zero_evaluates_to_zero() {
    let ctx = &CTX;
    let env = &ctx.to_env();
    let e = &the(nat(), zero());
    assert_eq!(e.val_of(env), values::zero());
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

#[test]
fn simple_use_of_ind_nat() {
    assert_eq!(
        rep(
            &CTX,
            &"(ind-Nat (add1 (add1 zero)) \
                       (lambda (x) Nat) \
                       (add1 zero)\
                       (lambda (n-1 ih) (add1 ih)))"
                .parse()
                .unwrap()
        ),
        Ok(the(nat(), add1(add1(add1(zero())))))
    )
}

#[test]
fn indnat_takes_neutral_targets() {
    assert_eq!(
        rep(
            &CTX,
            &"(the (-> Nat Nat Nat)
                   (lambda (x y)
                      (ind-Nat x
                               (lambda (x) Nat)
                               y
                               (lambda (n-1 ih) (add1 ih)))))"
                .parse()
                .unwrap()
        ),
        Ok(the(
            pi("x", nat(), pi("x₁", nat(), nat())),
            lambda(
                "x",
                lambda(
                    "y",
                    ind_nat(
                        refer("x"),
                        lambda("x₁", nat()),
                        refer("y"),
                        lambda("n-1", lambda("ih", add1(refer("ih"))))
                    )
                )
            )
        ))
    )
}

#[test]
fn function_type_expands_to_pi() {
    assert_eq!(
        rep(&CTX, &"(the U (-> Nat Nat))".parse().unwrap()),
        Ok(the(universe(), pi("x", nat(), nat())))
    )
}

#[test]
fn cant_have_function_map_from_u_to_u() {
    assert_eq!(
        rep(&CTX, &"(the U (-> U U))".parse().unwrap()),
        Err(Error::UhasNoType)
    )
}

#[test]
fn function_type_expands_to_pis() {
    assert_eq!(
        rep(&CTX, &"(-> Nat Nat Nat Nat Nat)".parse().unwrap()),
        Ok(the(
            universe(),
            pi(
                "x",
                nat(),
                pi("x₁", nat(), pi("x₂", nat(), pi("x₃", nat(), nat())))
            )
        ))
    )
}

#[test]
fn n_ary_pi_expands_to_sequence_of_unary_pies() {
    assert_eq!(
        rep(&CTX, &"(∏ ((x Nat) (y Nat)) Nat)".parse().unwrap()),
        Ok(the(universe(), pi("x", nat(), pi("y", nat(), nat()))))
    )
}

#[test]
fn zero_is_not_a_type() {
    assert_eq!(
        rep(&CTX, &"(the zero zero)".parse().unwrap()),
        Err(Error::NotAType(zero()))
    )
}

#[test]
fn wrong_return_type_in_lambda() {
    assert_eq!(
        rep(&CTX, &"(the (-> Nat U) (lambda (x) x))".parse().unwrap())
            .unwrap_err()
            .to_string(),
        "Expected type U but got Nat"
    )
}

#[test]
fn wrong_type_for_application() {
    assert_eq!(
        rep(&CTX, &"(zero zero)".parse().unwrap()),
        Err(Error::NotAFunctionType(nat()))
    )
}

#[test]
fn variables_must_be_bound() {
    assert_eq!(
        rep(&CTX, &"x".parse().unwrap()),
        Err(Error::UnknownVariable("x".into()))
    )
}

#[test]
fn the_normal_form_of_nat_is_nat() {
    assert_eq!(norm_type(&CTX, &"Nat".parse().unwrap()), Ok(nat()))
}

#[test]
fn pi_must_return_a_type() {
    assert_eq!(
        norm_type(&CTX, &"(∏ ((x Nat)) x)".parse().unwrap()),
        Err(Error::WrongType(nat(), universe()))
    )
}

#[test]
fn variables_must_be_bound_in_normal_type_too() {
    assert_eq!(
        norm_type(&CTX, &"x".parse().unwrap()),
        Err(Error::UnknownVariable("x".into()))
    )
}

#[test]
fn atom_types_are_automatically_inferred() {
    assert_eq!(
        rep(&CTX, &"'a".parse().unwrap()),
        Ok(the(atom(), quote("a")))
    )
}

#[test]
fn atom_can_be_optionally_annotated() {
    assert_eq!(
        rep(&CTX, &"(the Atom 'a)".parse().unwrap()),
        Ok(the(atom(), quote("a")))
    )
}

#[test]
fn atom_is_a_type() {
    assert_eq!(
        rep(&CTX, &"Atom".parse().unwrap()),
        Ok(the(universe(), atom()))
    )
}

#[test]
fn pair_desugars_to_sigma() {
    assert_eq!(
        rep(&CTX, &"(Pair Atom Atom)".parse().unwrap()),
        Ok(the(universe(), sigma("a", atom(), atom())))
    )
}

#[test]
fn nary_sigma_desugars_to_nested_sigmas() {
    assert_eq!(
        rep(&CTX, &"(Σ ((x Nat) (y Atom)) Nat)".parse().unwrap()),
        Ok(the(
            universe(),
            sigma("x", nat(), sigma("y", atom(), nat()))
        ))
    )
}

#[test]
fn cons_builds_sigma_but_can_be_annotated_as_pair() {
    assert_eq!(
        rep(
            &CTX,
            &"(the (Pair Atom Atom) (cons 'olive 'oil))".parse().unwrap()
        ),
        Ok(the(
            sigma("x", atom(), atom()),
            cons(quote("olive"), quote("oil"))
        ))
    )
}

#[test]
fn car_gets_a_pairs_first_element() {
    assert_eq!(
        rep(
            &CTX,
            &"(car (the (Pair Atom Atom) (cons 'olive 'oil)))"
                .parse()
                .unwrap()
        ),
        Ok(the(atom(), quote("olive"),))
    )
}

#[test]
fn cdr_gets_a_pairs_second_element() {
    assert_eq!(
        rep(
            &CTX,
            &"(cdr (the (Pair Atom Atom) (cons 'olive 'oil)))"
                .parse()
                .unwrap()
        ),
        Ok(the(atom(), quote("oil"),))
    )
}
