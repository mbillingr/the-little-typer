use the_little_typer as tlt;

use sexpr_matcher::match_sexpr;
use sexpr_parser::parse;
use std::{io, io::Write};
use the_little_typer::sexpr::Sexpr;
use tlt::{
    basics::{Core, Ctx},
    rep::norm,
    resugar::resugar,
};

fn main() -> io::Result<()> {
    let mut ctx = Ctx::new();
    prelude(&mut ctx);
    loop {
        match read_eval_normalize(&mut ctx) {
            Ok(None) => {}
            Ok(Some(out)) => println!("{}", resugar(&out)),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn read_eval_normalize(ctx: &mut Ctx) -> Result<Option<Core>, String> {
    let src = read_line().map_err(|e| e.to_string())?;
    eval_normalize(ctx, &src)
}

fn eval_normalize(ctx: &mut Ctx, src: &str) -> Result<Option<Core>, String> {
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

fn read_line() -> io::Result<String> {
    io::stdout().write(b"> ")?;
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn prelude(ctx: &mut Ctx) {
    let src = "
(claim +
    (-> Nat Nat
        Nat))

(claim step-+
    (-> Nat
        Nat))

(define step-+
    (λ (+_n-1)
        (add1 +_n-1)))

(define +
    (λ (n j)
        (iter-Nat n j step-+)))


(claim *
    (-> Nat Nat
        Nat))

(claim step-*
    (-> Nat Nat Nat
        Nat))

(define step-*
    (λ (j n-1 *_n-1)
        (+ j *_n-1)))

(define *
    (λ (n j)
        (rec-Nat n 0 (step-* j))))
";

    for stmt in src.split("\n\n") {
        eval_normalize(ctx, stmt).unwrap();
    }
}
