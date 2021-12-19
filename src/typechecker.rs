use crate::alpha::is_alpha_equiv;
use crate::basics::{Ctx, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::normalize::read_back;

pub fn same_type(ctx: &Ctx, given: &Value, expected: &Value) -> Result<()> {
    let given_e = given.read_back_type(ctx)?;
    let expected_e = expected.read_back_type(ctx)?;
    if is_alpha_equiv(&given_e, &expected_e) {
        Ok(())
    } else {
        Err(Error::WrongType(given_e, expected_e))
    }
}

pub fn convert(ctx: &Ctx, tv: &Value, av: &Value, bv: &Value) -> Result<()> {
    let a = read_back(ctx, tv, av)?;
    let b = read_back(ctx, tv, bv)?;
    if is_alpha_equiv(&a, &b) {
        Ok(())
    } else {
        Err(Error::NotTheSame(tv.read_back_type(ctx)?, a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::basics::{Core, CoreInterface, Renaming};
    use crate::symbol::Symbol;
    use crate::types::{cores, values};

    #[test]
    fn pi_is_a_type() {
        let ctx = &Ctx::new();
        let r = &Renaming::new();
        let e = &Core::pi(Symbol::new("x"), cores::nat(), cores::nat());
        let tv = &values::universe();
        assert!(e.check(ctx, r, tv).is_ok());
    }
}
