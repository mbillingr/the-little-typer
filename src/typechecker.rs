use crate::alpha::is_alpha_equiv;
use crate::basics::{fresh, fresh_binder, is_var_name, Core, Ctx, Renaming, Value, N};
use crate::errors::{Error, Result};
use crate::normalize::{now, read_back, read_back_type, val_in_ctx};
use crate::symbol::{Symbol as S, Symbol};

pub fn is_type(ctx: &Ctx, r: &Renaming, inp: &Core) -> Result<Core> {
    use Core::*;
    match inp {
        Nat => Ok(Nat),
        Fun(params) => match &params[..] {
            [a, b] => {
                let x = fresh_binder(ctx, b, &S::new("x"));
                let a_out = is_type(ctx, r, a)?;
                let b_out = is_type(&ctx.bind_free(x.clone(), val_in_ctx(ctx, &a_out))?, r, b)?;
                Ok(Core::pi(x, a_out, b_out))
            }
            _ => todo!("{:?}", inp),
        },
        Atom => Ok(Atom),
        _ => todo!("{:?}", inp),
    }
}

pub fn synth(ctx: &Ctx, r: &Renaming, inp: &Core) -> Result<Core> {
    use Core::*;
    match inp {
        U => Err(Error::UhasNoType),
        Fun(types) if types.len() == 2 => {
            // A -> B
            let a = &types[0];
            let b = &types[1];
            let z = fresh_binder(ctx, b, &S::new("x"));
            let a_out = check(ctx, r, a, &Value::Universe)?;
            let b_out = check(
                &ctx.bind_free(z.clone(), val_in_ctx(ctx, &a_out))?,
                r,
                b,
                &Value::Universe,
            )?;
            Ok(Core::the(Core::U, Core::pi(z, a_out, b_out)))
        }
        Nat => Ok(Core::the(U, Nat)),
        Zero => Ok(Core::the(Nat, Zero)),
        Add1(n) => check(ctx, r, n, &Value::Nat).map(|n_out| Core::the(Nat, Core::add1(n_out))),
        Atom => Ok(Core::the(U, Atom)),
        Quote(a) => {
            if atom_is_ok(a) {
                Ok(Core::the(Atom, Core::quote(a.clone())))
            } else {
                Err(Error::InvalidAtom(a.clone()))
            }
        }
        The(t, e) => {
            let t_out = is_type(ctx, r, t)?;
            let e_out = check(ctx, r, e, &val_in_ctx(ctx, &t_out))?;
            Ok(Core::the(t_out, e_out))
        }
        Symbol(x) if is_var_name(x) => {
            let real_x = r.rename(x);
            let xtv = ctx.var_type(&real_x)?;
            Ok(Core::the(read_back_type(ctx, &xtv), inp.clone()))
        }
        _ => todo!("{:?}", inp),
    }
}

pub fn check(ctx: &Ctx, r: &Renaming, e: &Core, tv: &Value) -> Result<Core> {
    match e {
        Core::Lambda(x, b) => match &*now(tv) {
            Value::Pi {
                arg_type: a,
                result_type: c,
                ..
            } => {
                let x_hat = fresh(ctx, x);
                let b_out = check(
                    &ctx.bind_free(x_hat.clone(), (**a).clone())?,
                    &r.extend(x.clone(), x_hat.clone()),
                    b,
                    &c.val_of(Value::neu(a.clone(), N::Var(x_hat.clone()))),
                )?;
                Ok(Core::lambda(x_hat, b_out))
            }
            non_pi => Err(Error::NotAFunctionType(read_back_type(ctx, non_pi))),
        },
        Core::Atom
        | Core::Quote(_)
        | Core::Fun(_)
        | Core::Symbol(_)
        | Core::Nat
        | Core::Zero
        | Core::Add1(_) => {
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
