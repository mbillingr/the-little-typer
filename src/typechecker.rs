use crate::alpha::is_alpha_equiv;
use crate::basics::{fresh_binder, is_var_name, Core, Ctx, Renaming, Value, ValueInterface};
use crate::errors::{Error, Result};
use crate::normalize::{read_back, read_back_type, val_in_ctx};
use crate::symbol::{Symbol as S, Symbol};
use crate::types::cores;

pub fn is_type(ctx: &Ctx, r: &Renaming, inp: &Core) -> Result<Core> {
    use crate::types::values;
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
            [a, b, cs @ ..] => {
                let x = fresh_binder(ctx, &make_app(b, cs), &S::new("x"));
                let a_out = is_type(ctx, r, a)?;
                let mut rest = vec![b.clone()];
                rest.extend(cs.iter().cloned());
                let t_out = is_type(
                    &ctx.bind_free(x.clone(), val_in_ctx(ctx, &a_out))?,
                    r,
                    &Core::Fun(rest),
                )?;
                Ok(Core::pi(x, a_out, t_out))
            }
            _ => panic!("invalid fun types {:?}", params),
        },
        PiStar(_, _) => todo!(),

        The(_, _) | AppStar(_, _) => match check(ctx, r, inp, &values::universe()) {
            Ok(t_out) => Ok(t_out),
            Err(_) => Err(Error::NotAType(inp.clone())),
        },

        Symbol(s) => match check(ctx, r, inp, &values::universe()) {
            Ok(t_out) => Ok(t_out),
            Err(_) if is_var_name(s) => ctx.var_type(s).and_then(|other_tv| {
                Err(Error::WrongType(
                    read_back_type(ctx, &other_tv),
                    cores::universe(),
                ))
            }),
            Err(_) => Err(Error::NotAType(inp.clone())),
        },

        Zero | Add1(_) | LambdaStar(_, _) => Err(Error::NotAType(inp.clone())),

        Object(obj) => obj.is_type(ctx, r),
    }
}

pub fn synth(ctx: &Ctx, r: &Renaming, inp: &Core) -> Result<Core> {
    use crate::types::values;
    use Core::*;
    match inp {
        Fun(types) => match &types[..] {
            [a, b] => {
                let z = fresh_binder(ctx, b, &S::new("x"));
                let a_out = check(ctx, r, a, &values::universe())?;
                let b_out = check(
                    &ctx.bind_free(z.clone(), val_in_ctx(ctx, &a_out))?,
                    r,
                    b,
                    &values::universe(),
                )?;
                Ok(Core::the(cores::universe(), Core::pi(z, a_out, b_out)))
            }
            [a, b, cs @ ..] => {
                let z = fresh_binder(ctx, &make_app(b, cs), &S::new("x"));
                let a_out = check(ctx, r, a, &values::universe())?;
                let mut out_args = vec![b.clone()];
                out_args.extend(cs.iter().cloned());
                let t_out = check(
                    &ctx.bind_free(z.clone(), val_in_ctx(ctx, &a_out))?,
                    r,
                    &Core::Fun(out_args),
                    &values::universe(),
                )?;
                Ok(Core::the(cores::universe(), Core::pi(z, a_out, t_out)))
            }
            _ => todo!(),
        },
        PiStar(_, _) => todo!(),
        Nat => Ok(Core::the(cores::universe(), Nat)),
        Zero => Ok(Core::the(Nat, Zero)),
        Add1(n) => check(ctx, r, n, &values::nat()).map(|n_out| Core::the(Nat, Core::add1(n_out))),
        The(t, e) => {
            let t_out = is_type(ctx, r, t)?;
            let e_out = check(ctx, r, e, &val_in_ctx(ctx, &t_out))?;
            Ok(Core::the(t_out, e_out))
        }
        AppStar(rator, args) => match &args[..] {
            [] => panic!("nullary application"),
            [rand] => match synth(ctx, r, rator)? {
                The(rator_t, rator_out) => {
                    val_in_ctx(ctx, &rator_t).apply(ctx, r, &rator_out, rand)
                }
                _ => unreachable!(),
            },
            [_rand0, _rands @ ..] => todo!(),
        },
        Symbol(x) if is_var_name(x) => {
            let real_x = r.rename(x);
            let xtv = ctx.var_type(&real_x)?;
            Ok(Core::the(read_back_type(ctx, &xtv), inp.clone()))
        }
        Symbol(_) | LambdaStar(_, _) => Err(Error::CantDetermineType(inp.clone())),

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

        Core::The(_, _)
        | Core::Fun(_)
        | Core::PiStar(_, _)
        | Core::AppStar(_, _)
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

fn make_app(a: &Core, cs: &[Core]) -> Core {
    Core::app_star(a.clone(), cs)
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
            &Core::pi(Symbol::new("x"), Core::Nat, Core::Nat),
            &values::universe()
        )
        .is_ok());
    }
}
