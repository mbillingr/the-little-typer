use crate::basics::{
    Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, The, Value, ValueInterface, N,
};
use crate::errors::{Error, Result};
use crate::normalize::read_back;
use crate::types::values::later;
use crate::types::{cores, values};
use std::any::Any;

/// The absurd type (also known as empty type)
#[derive(Debug, Copy, Clone)]
pub struct Absurd;

#[derive(Debug, Clone, PartialEq)]
pub struct IndAbsurd {
    target: Core,
    motive: Core,
}

impl IndAbsurd {
    pub fn new(target: Core, motive: Core) -> Self {
        IndAbsurd { target, motive }
    }
}

#[derive(Debug)]
pub struct NeutralIndAbsurd {
    target: N,
    motive: The,
}

impl CoreInterface for Absurd {
    impl_core_defaults!(
        _,
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        simple_type,
        check_by_synth,
        (resugar: absurd)
    );

    fn val_of(&self, _env: &Env) -> Value {
        values::absurd()
    }

    fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> Result<(Core, Core)> {
        Ok((cores::universe(), cores::absurd()))
    }
}

impl CoreInterface for IndAbsurd {
    impl_core_defaults!(
        (target, motive),
        as_any,
        same,
        occurring_names,
        alpha_equiv,
        no_type,
        check_by_synth,
        (resugar: ind_absurd)
    );

    fn val_of(&self, env: &Env) -> Value {
        do_ind_absurd(
            later(env.clone(), self.target.clone()),
            later(env.clone(), self.motive.clone()),
        )
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let tgt_out = self.target.check(ctx, r, &values::absurd())?;
        let mot_out = self.motive.check(ctx, r, &values::universe())?;
        Ok((mot_out.clone(), cores::ind_absurd(tgt_out, mot_out)))
    }
}

impl_sexpr_display!(Absurd, "Absurd");
impl_sexpr_display!(IndAbsurd, ("ind-Absurd", target, motive));

impl ValueInterface for Absurd {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Ok(cores::absurd())
    }

    fn read_back(&self, ctx: &Ctx, tv: &Value, v: &Value) -> Result<Core> {
        if let Some((_, ne)) = v.as_neutral() {
            Ok(cores::the(cores::absurd(), ne.read_back_neutral(ctx)?))
        } else {
            Err(Error::TypeMismatchVar(v.clone(), tv.clone()))
        }
    }
}

fn do_ind_absurd(tgt_v: Value, mot_v: Value) -> Value {
    if let Some((abs, ne)) = tgt_v.as_neutral() {
        if abs.try_as::<Absurd>().is_some() {
            return values::neutral(
                mot_v.clone(),
                NeutralIndAbsurd {
                    target: ne.clone(),
                    motive: The(values::universe(), mot_v),
                },
            );
        }
    }

    unreachable!()
}

impl NeutralInterface for NeutralIndAbsurd {
    fn read_back_neutral(&self, ctx: &Ctx) -> Result<Core> {
        let NeutralIndAbsurd {
            target: tgt,
            motive: The(tv, ttv),
        } = self;
        Ok(cores::ind_absurd(
            cores::the(cores::absurd(), tgt.read_back_neutral(ctx)?),
            read_back(ctx, tv, ttv)?,
        ))
    }
}
