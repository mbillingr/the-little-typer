use crate::basics::{Core, CoreInterface, Ctx, Renaming};
use crate::errors::Error;
use crate::normalize::val_in_ctx;
use crate::rep;
use crate::types::cores;

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

    fn define(self, name: &str, expr: &str) -> Self {
        todo!()
    }
}

struct CoreChecker<'a> {
    ctx: &'a Ctx,
    expr: Core,
}

impl<'a> CoreChecker<'a> {
    fn and(self, s: &'static str) -> TwoCoreChecker<'a> {
        TwoCoreChecker {
            ctx: self.ctx,
            expr1: self.expr,
            expr2: s.parse().unwrap(),
        }
    }

    fn is_a_type(&self) -> bool {
        self.expr.is_type(self.ctx, &Renaming::new()).unwrap();
        true
    }

    fn is_a(&self, t: &'static str) -> bool {
        let t: Core = t.parse().unwrap();
        let t_out = t.is_type(self.ctx, &Renaming::new()).unwrap();
        let tv = val_in_ctx(self.ctx, &t_out);
        self.expr.check(self.ctx, &Renaming::new(), &tv).unwrap();
        true
    }

    fn is_not_a(&self, t: &'static str) -> bool {
        let t: Core = t.parse().unwrap();
        let t_out = t.is_type(self.ctx, &Renaming::new()).unwrap();
        let tv = val_in_ctx(self.ctx, &t_out);
        match self.expr.check(self.ctx, &Renaming::new(), &tv) {
            Err(_) => true,
            other => panic!("{:?}", other),
        }
    }

    fn checks(&self) {
        self.expr.synth(self.ctx, &Renaming::new()).unwrap();
    }

    fn check(self) -> Self {
        self.expr.synth(self.ctx, &Renaming::new()).unwrap();
        self
    }
}

struct TwoCoreChecker<'a> {
    ctx: &'a Ctx,
    expr1: Core,
    expr2: Core,
}

impl<'a> TwoCoreChecker<'a> {
    fn are_the_same(&self, t: &'static str) {
        let t = t.parse().unwrap();
        rep::check_same(&self.ctx, &t, &self.expr1, &self.expr2).unwrap();
    }

    fn are_not_the_same(&self, t: &'static str) {
        let t = t.parse().unwrap();
        match rep::check_same(&self.ctx, &t, &self.expr1, &self.expr2) {
            Err(Error::NotTheSame(_, _, _)) => {}
            other => panic!("{:?}", other),
        }
    }
    fn are_the_same_type(&self) {
        rep::check_same(&self.ctx, &cores::universe(), &self.expr1, &self.expr2).unwrap();
    }

    fn are_not_the_same_type(&self) {
        match rep::check_same(&self.ctx, &cores::universe(), &self.expr1, &self.expr2) {
            Err(Error::NotTheSame(_, _, _)) => {}
            other => panic!("{:?}", other),
        }
    }

    fn check(self) -> Self {
        self.expr1.synth(self.ctx, &Renaming::new()).unwrap();
        self.expr2.synth(self.ctx, &Renaming::new()).unwrap();
        self
    }
}
