use crate::alpha::is_alpha_equiv;
use crate::basics::{Core, Ctx, Renaming, Value};
use crate::errors::{Error, Result};
use crate::normalize::{read_back, read_back_type};
use crate::symbol::Symbol;

pub fn is_type(ctx: &Ctx, r: &Renaming, inp: &Core) -> Result<Core> {
    use Core::*;
    match inp {
        Object(obj) => obj.is_type(ctx, r),
    }
}

pub fn synth(ctx: &Ctx, r: &Renaming, inp: &Core) -> Result<(Core, Core)> {
    use Core::*;
    match inp {
        Object(obj) => obj.synth(ctx, r),
    }
}

pub fn check(ctx: &Ctx, r: &Renaming, e: &Core, tv: &Value) -> Result<Core> {
    match e {
        Core::Object(obj) => obj.check(ctx, r, tv),
    }
}

pub fn same_type(ctx: &Ctx, given: &Value, expected: &Value) -> Result<()> {
    let given_e = read_back_type(ctx, given);
    let expected_e = read_back_type(ctx, expected);
    if is_alpha_equiv(&given_e, &expected_e) {
        Ok(())
    } else {
        Err(Error::WrongType(given_e, expected_e))
    }
}

pub fn convert(ctx: &Ctx, tv: &Value, av: &Value, bv: &Value) -> Result<()> {
    let a = read_back(ctx, tv, av);
    let b = read_back(ctx, tv, bv);
    if is_alpha_equiv(&a, &b) {
        Ok(())
    } else {
        Err(Error::NotTheSame(read_back_type(ctx, tv), a, b))
    }
}

pub fn atom_is_ok(_: &Symbol) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{cores, values};

    #[test]
    fn pi_is_a_type() {
        assert!(check(
            &Ctx::new(),
            &Renaming::new(),
            &Core::pi(Symbol::new("x"), cores::nat(), cores::nat()),
            &values::universe()
        )
        .is_ok());
    }
}
