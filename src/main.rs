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
