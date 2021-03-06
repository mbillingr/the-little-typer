use crate::basics::{Core, CoreInterface, Ctx, Renaming};
use crate::errors::Result;
use crate::normalize::{read_back, val_in_ctx};
use crate::sexpr::Sexpr;
use crate::typechecker::convert;
use sexpr_matcher::match_sexpr;
use sexpr_parser::parse;
use std::result;

pub fn norm_type(ctx: &Ctx, e: &Core) -> Result<Core> {
    let e_out = e.is_type(ctx, &Renaming::new())?;
    val_in_ctx(ctx, &e_out).read_back_type(ctx)
}

pub fn rep(ctx: &Ctx, e: &Core) -> Result<Core> {
    let (t_out, e_out) = e.synth(ctx, &Renaming::new())?;
    let tv = val_in_ctx(ctx, &t_out);
    let v = val_in_ctx(ctx, &e_out);
    let vx = read_back(ctx, &tv, &v)?;
    let tx = tv.read_back_type(ctx)?;
    Ok(Core::the(tx, vx))
}

pub fn norm(ctx: &Ctx, e: &Core) -> Result<Core> {
    match rep(ctx, e) {
        Ok(e) => Ok(e),
        Err(err) => match norm_type(ctx, e) {
            Ok(out) => Ok(out),
            _ => Err(err),
        },
    }
}

pub fn check_same(ctx: &Ctx, t: &Core, a: &Core, b: &Core) -> Result<()> {
    let t_out = t.is_type(ctx, &Renaming::new())?;
    let tv = val_in_ctx(ctx, &t_out);
    let a_out = a.check(ctx, &Renaming::new(), &tv)?;
    let b_out = b.check(ctx, &Renaming::new(), &tv)?;
    let av = val_in_ctx(ctx, &a_out);
    let bv = val_in_ctx(ctx, &b_out);
    convert(ctx, &tv, &av, &bv)
}

pub fn eval_normalize(ctx: &mut Ctx, src: &str) -> result::Result<Option<Core>, String> {
    let sexpr = parse::<Sexpr>(&src).map_err(|e| e.to_string())?;

    match_sexpr!(
        &sexpr,
        case ("claim", [Sexpr::Symbol(ident)], expr) => {
            *ctx = ctx.claim(ident.clone(), expr.into()).map_err(|e| e.to_string())?;
            return Ok(None);
        },
        case ("define", [Sexpr::Symbol(ident)], expr) => {
            *ctx = ctx.define(ident.clone(), expr.into()).map_err(|e| e.to_string())?;
            return Ok(None);
        },
        case ("reclaim", [Sexpr::Symbol(ident)], expr) => {
            *ctx = ctx.reclaim(ident.clone(), expr.into()).map_err(|e| e.to_string())?;
            return Ok(None);
        },
        case ("redefine", [Sexpr::Symbol(ident)], expr) => {
            *ctx = ctx.redefine(ident.clone(), expr.into()).map_err(|e| e.to_string())?;
            return Ok(None);
        },
        else => {},
    );

    norm(ctx, &(&sexpr).into())
        .map(Some)
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::errors::Error;
    use crate::types::cores;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref CTX: Ctx = Ctx::new();
    }

    #[test]
    fn just_an_atom() {
        assert_eq!(
            rep(&CTX, &Core::quote("atom")),
            Ok(Core::the(cores::atom(), cores::quote("atom")))
        );
    }

    #[test]
    fn just_a_type() {
        assert_eq!(
            rep(&CTX, &cores::atom()),
            Ok(Core::the(cores::universe(), cores::atom()))
        );
    }

    #[test]
    fn u_does_not_have_a_type() {
        assert_eq!(rep(&CTX, &cores::universe()), Err(Error::UhasNoType));
    }

    #[test]
    fn a_function_type() {
        assert_eq!(
            rep(&CTX, &Core::fun(vec![cores::atom()], cores::atom())),
            Ok(Core::the(
                cores::universe(),
                Core::pi("x", cores::atom(), cores::atom())
            ))
        );
        assert_eq!(
            rep(&CTX, &"(-> Atom Atom)".parse().unwrap()),
            Ok(Core::the(
                cores::universe(),
                Core::pi("x", cores::atom(), cores::atom())
            ))
        );
    }

    #[test]
    fn type_annotation() {
        assert_eq!(
            rep(&CTX, &"(the Atom 'atom)".parse().unwrap()),
            Ok(Core::the(cores::atom(), Core::quote("atom")))
        );
    }

    #[test]
    fn type_annotation_mismatch() {
        assert_eq!(
            rep(&CTX, &"(the Nat 'atom)".parse().unwrap()),
            Err(Error::WrongType(cores::atom(), cores::nat()))
        );
    }

    #[test]
    fn a_curried_function_type() {
        assert_eq!(
            rep(&CTX, &"(-> Atom (-> Atom Atom))".parse().unwrap()),
            Ok(Core::the(
                cores::universe(),
                Core::pi(
                    "x",
                    cores::atom(),
                    Core::pi("x???", cores::atom(), cores::atom())
                )
            ))
        );
    }

    #[test]
    fn a_function_type_and_various_implementations() {
        // A function that takes an Atom and returns an Atom
        let expected_type = Core::pi("x", cores::atom(), cores::atom());

        // The identity function satisfies that type.
        assert_eq!(
            rep(
                &CTX,
                &"(the (-> Atom Atom) (lambda (x) x))".parse().unwrap()
            ),
            Ok(Core::the(
                expected_type.clone(),
                Core::lambda("x", Core::symbol("x"))
            ))
        );

        // A function that ignores its argument and always returns a constant Atom satisfies that type too.
        assert_eq!(
            rep(
                &CTX,
                &"(the (-> Atom Atom) (lambda (x) 'y))".parse().unwrap()
            ),
            Ok(Core::the(
                expected_type.clone(),
                Core::lambda("x", Core::quote("y"))
            ))
        );
    }

    #[test]
    fn the_number_zero() {
        assert_eq!(
            rep(&CTX, &"0".parse().unwrap()),
            Ok(Core::the(cores::nat(), Core::nat(0),))
        );
        assert_eq!(
            rep(&CTX, &"zero".parse().unwrap()),
            Ok(Core::the(cores::nat(), Core::nat(0)))
        );
    }

    #[test]
    fn one_more() {
        assert_eq!(
            rep(&CTX, &"(add1 0)".parse().unwrap()),
            Ok(Core::the(cores::nat(), Core::nat(1)))
        );

        assert_eq!(
            rep(&CTX, &"1".parse().unwrap()),
            Ok(Core::the(cores::nat(), Core::nat(1)))
        );
    }

    #[test]
    fn two_more() {
        assert_eq!(
            rep(&CTX, &"(add1 (add1 0))".parse().unwrap()),
            Ok(Core::the(cores::nat(), Core::nat(2)))
        );

        assert_eq!(
            rep(&CTX, &"2".parse().unwrap()),
            Ok(Core::the(cores::nat(), Core::nat(2)))
        );
    }

    #[test]
    fn same_atoms() {
        assert_eq!(
            check_same(
                &CTX,
                &cores::atom(),
                &Core::quote("apple"),
                &Core::quote("apple")
            ),
            Ok(())
        );
    }

    #[test]
    fn different_atoms() {
        assert_eq!(
            check_same(
                &CTX,
                &cores::atom(),
                &Core::quote("apple"),
                &Core::quote("pear")
            ),
            Err(Error::NotTheSame(
                cores::atom(),
                Core::quote("apple"),
                Core::quote("pear")
            ))
        );
    }

    #[test]
    fn function_application() {
        assert_eq!(
            rep(
                &CTX,
                &"((the (-> Atom Atom) (lambda (x) x)) 'foo)"
                    .parse()
                    .unwrap()
            ),
            Ok(Core::the(cores::atom(), Core::quote("foo")))
        );
    }

    #[test]
    fn applying_a_variable_function() {
        assert!(rep(
            &CTX,
            &"(the (-> (-> Atom Atom) Atom) (lambda (x) (x 'foo)))"
                .parse()
                .unwrap()
        )
        .is_ok());
    }

    #[test]
    fn expressions_without_type_are_only_normalized() {
        assert_eq!(norm(&CTX, &"U".parse().unwrap()), Ok(cores::universe()));

        assert_eq!(
            norm(&CTX, &"(Pair U U)".parse().unwrap()),
            Ok(cores::sigma("x", cores::universe(), cores::universe()))
        );

        assert_eq!(
            norm(&CTX, &"(-> U U)".parse().unwrap()),
            Ok(cores::pi("x", cores::universe(), cores::universe()))
        );
    }
}
