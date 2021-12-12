use crate::alpha::is_alpha_equiv;
use crate::basics::{fresh_binder, Core, Ctx, Renaming, Value, R};
use crate::errors::{Error, Result};
use crate::normalize::{read_back_type, val_in_ctx};
use crate::symbol::{Symbol as S, Symbol};

pub fn is_type(_ctx: &Ctx, _renaming: &Renaming, inp: &Core) -> Result<Core> {
    use Core::*;
    match inp {
        Nat => Ok(Nat),
        Atom => Ok(Atom),
        _ => todo!("{:?}", inp),
    }
}

pub fn synth(ctx: &Ctx, renaming: &Renaming, inp: &Core) -> Result<Core> {
    use Core::*;
    match inp {
        Fun(types) if types.len() == 2 => {
            // A -> B
            let a = &types[0];
            let b = &types[1];
            let z = fresh_binder(ctx, b, &S::new("x"));
            let a_out = check(ctx, renaming, a, &Value::Universe)?;
            let b_out = check(
                &ctx.bind_free(z.clone(), val_in_ctx(ctx, &a_out))?,
                renaming,
                b,
                &Value::Universe,
            )?;
            Ok(Core::the(
                Core::U,
                Core::Pi(z, R::new(a_out), R::new(b_out)),
            ))
        }
        Atom => Ok(Core::the(U, Atom)),
        Quote(a) => {
            if atom_is_ok(a) {
                Ok(Core::the(Atom, Core::quote(a.clone())))
            } else {
                Err(Error::InvalidAtom(a.clone()))
            }
        }
        The(t, e) => {
            let t_out = is_type(ctx, renaming, t)?;
            let e_out = check(ctx, renaming, e, &val_in_ctx(ctx, &t_out))?;
            Ok(Core::the(t_out, e_out))
        }
        _ => todo!("{:?}", inp),
    }
}

pub fn check(ctx: &Ctx, r: &Renaming, e: &Core, tv: &Value) -> Result<Core> {
    match e {
        Core::Atom | Core::Quote(_) | Core::Fun(_) => {
            if let Core::The(t_out, e_out) = synth(ctx, r, e)? {
                same_type(ctx, &val_in_ctx(ctx, &*t_out), tv)?;
                Ok((*e_out).clone())
            } else {
                unreachable!()
            }
        }
        _ => todo!("{:?}", e),
    }
}

pub fn same_type(ctx: &Ctx, given: &Value, expected: &Value) -> Result<()> {
    let given_e = read_back_type(ctx, given);
    let expected_e = read_back_type(ctx, expected);
    if is_alpha_equiv(&given_e, &expected_e) {
        Ok(())
    } else {
        Err(Error::UnexpectedType(given_e, expected_e))
    }
}

pub fn atom_is_ok(_: &Symbol) -> bool {
    true
}
