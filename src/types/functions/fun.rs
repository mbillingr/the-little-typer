use crate::basics::{fresh_binder, Core, CoreInterface, Ctx, Env, Renaming, Value};
use crate::errors::Result;
use crate::normalize::val_in_ctx;
use crate::symbol::Symbol;
use crate::types::{cores, values};
use std::collections::HashSet;
use std::fmt::Formatter;

/// Syntactic Sugar: n-ary function type, desugars to nested `Pi`s
#[derive(Debug, Clone, PartialEq)]
pub struct Fun(pub Vec<Core>);

impl CoreInterface for Fun {
    impl_core_defaults!((), as_any, same, no_alpha_equiv, check_by_synth);

    fn occurring_names(&self) -> HashSet<Symbol> {
        let mut names = HashSet::new();
        for t in &self.0 {
            names = &names | &t.occurring_names();
        }
        names
    }

    fn val_of(&self, _env: &Env) -> Value {
        panic!("Attempt to evaluate -> (should have been desugared to Pi)")
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        match &self.0[..] {
            [a, b] => {
                let x = fresh_binder(ctx, b, &Symbol::new("x"));
                let inp = a;
                let a_out = inp.is_type(ctx, r)?;
                let ctx = &ctx.bind_free(x.clone(), val_in_ctx(ctx, &a_out))?;
                let inp = b;
                let b_out = inp.is_type(ctx, r)?;
                Ok(Core::pi(x, a_out, b_out))
            }
            [a, b, cs @ ..] => {
                let x = fresh_binder(
                    ctx,
                    &Core::app_star(b.clone(), cs.to_vec()),
                    &Symbol::new("x"),
                );
                let inp = a;
                let a_out = inp.is_type(ctx, r)?;
                let mut rest = vec![b.clone()];
                rest.extend(cs.iter().cloned());
                let ctx = &ctx.bind_free(x.clone(), val_in_ctx(ctx, &a_out))?;
                let inp = &cores::fun(rest);
                let t_out = inp.is_type(ctx, r)?;
                Ok(Core::pi(x, a_out, t_out))
            }
            _ => panic!("invalid fun types {:?}", self.0),
        }
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        match &self.0[..] {
            [a, b] => {
                let z = fresh_binder(ctx, b, &Symbol::new("x"));
                let e = a;
                let tv = &values::universe();
                let a_out = e.check(ctx, r, tv)?;
                let ctx = &ctx.bind_free(z.clone(), val_in_ctx(ctx, &a_out))?;
                let e = b;
                let tv = &values::universe();
                let b_out = e.check(ctx, r, tv)?;
                Ok((cores::universe(), Core::pi(z, a_out, b_out)))
            }
            [a, b, cs @ ..] => {
                let z = fresh_binder(
                    ctx,
                    &Core::app_star(b.clone(), cs.to_vec()),
                    &Symbol::new("x"),
                );
                let e = a;
                let tv = &values::universe();
                let a_out = e.check(ctx, r, tv)?;
                let mut out_args = vec![b.clone()];
                out_args.extend(cs.iter().cloned());
                let ctx = &ctx.bind_free(z.clone(), val_in_ctx(ctx, &a_out))?;
                let e = &cores::fun(out_args);
                let tv = &values::universe();
                let t_out = e.check(ctx, r, tv)?;
                Ok((cores::universe(), Core::pi(z, a_out, t_out)))
            }
            _ => todo!(),
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        todo!()
    }
}

impl std::fmt::Display for Fun {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(->")?;
        for t in &self.0 {
            write!(f, " {}", t)?;
        }
        write!(f, ")")
    }
}
