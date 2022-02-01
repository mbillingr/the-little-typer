use crate::basics::{Core, CoreInterface, Ctx, Renaming, Value};
use crate::errors::{Error, Result};
use crate::normalize::val_in_ctx;
use crate::rep;
use crate::types::cores;

mod chapter_01;
mod chapter_02;
mod chapter_03;
mod chapter_04;
mod chapter_05;
mod chapter_06;
mod chapter_07;
mod chapter_08;
mod chapter_09;
mod chapter_10;
mod chapter_11;
mod chapter_12;
mod chapter_13;
mod chapter_14;
mod chapter_15;
mod chapter_16;
mod common_definitions;

fn with_empty_context() -> Checker {
    Checker { ctx: Ctx::new() }
}

#[derive(Clone)]
pub struct Checker {
    ctx: Ctx,
}

impl Checker {
    pub fn into_context(self) -> Ctx {
        self.ctx
    }

    fn core(&self, s: &'static str) -> CoreChecker {
        CoreChecker {
            ctx: self.ctx.clone(),
            expr: s.parse().unwrap(),
        }
    }

    fn claim(mut self, name: &str, expr: &str) -> Self {
        let t: Core = expr.parse().unwrap();
        self.ctx = self.ctx.claim(name, t).unwrap();
        self
    }

    fn define(mut self, name: &str, expr: &str) -> Result<Self> {
        let v: Core = expr.parse().unwrap();
        self.ctx = self.ctx.define(name, v)?;
        Ok(self)
    }
}

struct CoreChecker {
    ctx: Ctx,
    expr: Core,
}

impl CoreChecker {
    fn and(self, s: &'static str) -> TwoCoreChecker {
        TwoCoreChecker {
            ctx: self.ctx,
            expr1: self.expr,
            expr2: s.parse().unwrap(),
        }
    }

    fn is_a_type(&self) -> bool {
        self.expr.is_type(&self.ctx, &Renaming::new()).unwrap();
        true
    }

    fn is_a(&self, t: &'static str) -> bool {
        let t: Core = t.parse().unwrap();
        let t_out = t.is_type(&self.ctx, &Renaming::new()).unwrap();
        let tv = val_in_ctx(&self.ctx, &t_out);
        self.expr.check(&self.ctx, &Renaming::new(), &tv).unwrap();
        true
    }

    fn is_not_a(&self, t: &'static str) -> bool {
        let t: Core = t.parse().unwrap();
        let t_out = t.is_type(&self.ctx, &Renaming::new()).unwrap();
        let tv = val_in_ctx(&self.ctx, &t_out);
        match self.expr.check(&self.ctx, &Renaming::new(), &tv) {
            Err(_) => true,
            other => panic!("{:?}", other),
        }
    }

    fn checks(&self) {
        self.expr.synth(&self.ctx, &Renaming::new()).unwrap();
    }

    fn check(self) -> Self {
        self.expr.synth(&self.ctx, &Renaming::new()).unwrap();
        self
    }

    fn evaluates_to(&self, v: &'static str) {
        let v: Value = v.parse().unwrap();
        let (_, e_out) = self.expr.synth(&self.ctx, &Renaming::new()).unwrap();
        assert_eq!(val_in_ctx(&self.ctx, &e_out), v);
    }
}

struct TwoCoreChecker {
    ctx: Ctx,
    expr1: Core,
    expr2: Core,
}

impl TwoCoreChecker {
    fn are_the_same(&self, t: &'static str) -> Result<bool> {
        let t = t.parse().unwrap();
        match rep::check_same(&self.ctx, &t, &self.expr1, &self.expr2) {
            Ok(_) => Ok(true),
            Err(Error::NotTheSame(_, _, _)) => Ok(false),
            Err(e) => Err(e),
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
        self.expr1.synth(&self.ctx, &Renaming::new()).unwrap();
        self.expr2.synth(&self.ctx, &Renaming::new()).unwrap();
        self
    }
}

trait ResultAssertions {
    fn assert_ok(self);
    fn assert_err(self);
}

impl<T> ResultAssertions for Result<T> {
    fn assert_ok(self) {
        self.unwrap();
    }
    fn assert_err(self) {
        if self.is_ok() {
            panic!("Expected error, but was ok")
        }
    }
}

trait ResultBoolAssertions {
    fn assert(self, value: bool);
}

impl ResultBoolAssertions for Result<bool> {
    fn assert(self, value: bool) {
        match self {
            Ok(b) if b == value => {}
            Ok(b) => panic!("expected {}, got {}", value, b),
            Err(e) => panic!("{}", e),
        }
    }
}

impl ResultBoolAssertions for bool {
    fn assert(self, value: bool) {
        if self != value {
            panic!("expected {}, got {}", value, self)
        }
    }
}
