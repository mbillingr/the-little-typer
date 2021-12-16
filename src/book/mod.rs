use crate::basics::{Core, Ctx, Renaming};
use crate::errors::Error;
use crate::normalize::val_in_ctx;
use crate::rep;
use crate::typechecker::{check, is_type, synth};

mod chapter_01;

fn in_context(ctx: &Ctx) -> Checker {
    Checker { ctx }
}

struct Checker<'a> {
    ctx: &'a Ctx,
}

impl<'a> Checker<'a> {
    fn core(self, s: &'static str) -> CoreChecker<'a> {
        CoreChecker {
            ctx: self.ctx,
            expr: s.parse().unwrap(),
        }
    }

    fn check_same(&self, t: &'static str, a: &'static str, b: &'static str) {
        let t = t.parse().unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();
        rep::check_same(&self.ctx, &t, &a, &b).unwrap();
    }

    fn check_not_same(&self, t: &'static str, a: &'static str, b: &'static str) {
        let t = t.parse().unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();
        match rep::check_same(&self.ctx, &t, &a, &b) {
            Err(Error::NotTheSame(_, _, _)) => {}
            other => panic!("{:?}", other),
        }
    }
}

struct CoreChecker<'a> {
    ctx: &'a Ctx,
    expr: Core,
}

impl CoreChecker<'_> {
    fn is_a(&self, t: &Core) -> bool {
        let t_out = is_type(self.ctx, &Renaming::new(), t).unwrap();
        let tv = val_in_ctx(self.ctx, &t_out);
        check(self.ctx, &Renaming::new(), &self.expr, &tv).is_ok()
    }

    fn check(&self) {
        synth(self.ctx, &Renaming::new(), &self.expr).unwrap();
    }
}
