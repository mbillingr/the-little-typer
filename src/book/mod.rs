use crate::basics::{Core, Ctx, Renaming};
use crate::normalize::val_in_ctx;
use crate::typechecker::{check, is_type};

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
}
