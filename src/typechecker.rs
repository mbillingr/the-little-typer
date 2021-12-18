use crate::alpha::is_alpha_equiv;
use crate::basics::{fresh, Core, Ctx, Renaming, Value};
use crate::errors::{Error, Result};
use crate::normalize::{read_back, read_back_type, val_in_ctx};
use crate::symbol::Symbol;
use crate::types::cores;

pub fn is_type(ctx: &Ctx, r: &Renaming, inp: &Core) -> Result<Core> {
    use Core::*;
    match inp {
        PiStar(bindings, b) => match &bindings[..] {
            [] => unimplemented!(),
            [(x, a)] => {
                let y = fresh(ctx, x);
                let a_out = is_type(ctx, r, a)?;
                let a_outv = val_in_ctx(ctx, &a_out);
                let b_out = is_type(
                    &ctx.bind_free(y.clone(), a_outv)?,
                    &r.extend(x.clone(), y.clone()),
                    b,
                )?;
                Ok(cores::pi(y, a_out, b_out))
            }
            [(x, a), more @ ..] => {
                let z = fresh(ctx, x);
                let a_out = is_type(ctx, r, a)?;
                let a_outv = val_in_ctx(ctx, &a_out);
                let b_out = is_type(
                    &ctx.bind_free(z.clone(), a_outv)?,
                    &r.extend(x.clone(), z.clone()),
                    &PiStar(more.to_vec(), b.clone()),
                )?;
                Ok(cores::pi(z, a_out, b_out))
            }
        },

        LambdaStar(_, _) => Err(Error::NotAType(inp.clone())),

        Object(obj) => obj.is_type(ctx, r),
    }
}

pub fn synth(ctx: &Ctx, r: &Renaming, inp: &Core) -> Result<(Core, Core)> {
    use Core::*;
    match inp {
        PiStar(_, _) => todo!(),

        LambdaStar(_, _) => Err(Error::CantDetermineType(inp.clone())),

        Object(obj) => obj.synth(ctx, r),
    }
}

pub fn check(ctx: &Ctx, r: &Renaming, e: &Core, tv: &Value) -> Result<Core> {
    match e {
        Core::LambdaStar(params, b) => match &params[..] {
            [] => panic!("nullary lambda"),
            [x] => check(ctx, r, &Core::lambda(x.clone(), (**b).clone()), tv),
            [x, xs @ ..] => check(
                ctx,
                r,
                &Core::lambda(x.clone(), Core::LambdaStar(xs.to_vec(), b.clone())),
                tv,
            ),
        },

        Core::PiStar(_, _) => {
            let (t_out, e_out) = synth(ctx, r, e)?;
            same_type(ctx, &val_in_ctx(ctx, &t_out), tv)?;
            Ok(e_out)
        }

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
    use crate::types::values;

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
