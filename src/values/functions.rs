use crate::basics::{fresh, Closure, Core, Ctx, Renaming, Value, ValueInterface, N};
use crate::errors::{Error, Result};
use crate::normalize::{now, read_back, read_back_type, val_in_ctx};
use crate::symbol::Symbol;
use crate::typechecker::check;
use crate::values;
use crate::values::lambda;
use std::any::Any;

/// The dependent product type
#[derive(Debug)]
pub struct Pi {
    pub arg_name: Symbol,
    pub arg_type: Value,
    pub res_type: Closure,
}

/// An actual Function
#[derive(Debug)]
pub struct Lambda {
    pub arg_name: Symbol,
    pub body: Closure,
}

impl ValueInterface for Pi {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn ValueInterface) -> bool {
        unimplemented!()
    }

    fn read_back_type(&self, ctx: &Ctx) -> Result<Core> {
        let ae = read_back_type(ctx, &self.arg_type);
        let x_hat = fresh(ctx, &self.arg_name);

        let ctx_hat = ctx.bind_free(x_hat.clone(), self.arg_type.clone()).unwrap();
        let r = read_back_type(
            &ctx_hat,
            &self.res_type.val_of(values::neutral(
                self.arg_type.clone(),
                N::Var(x_hat.clone()),
            )),
        );
        Ok(Core::pi(x_hat, ae, r))
    }

    fn read_back(&self, ctx: &Ctx, _tv: &Value, f: &Value) -> Result<Core> {
        let y = match f.as_any().downcast_ref::<Lambda>() {
            Some(lam) => &lam.arg_name,
            None => &self.arg_name,
        };

        let x_hat = fresh(ctx, y);
        Ok(Core::lambda(
            x_hat.clone(),
            read_back(
                &ctx.bind_free(x_hat.clone(), self.arg_type.clone()).unwrap(),
                &self.res_type.val_of(values::neutral(
                    self.arg_type.clone(),
                    N::Var(x_hat.clone()),
                )),
                &do_ap(f, values::neutral(self.arg_type.clone(), N::Var(x_hat))),
            ),
        ))
    }

    fn apply(&self, _ctx: &Ctx, _r: &Renaming, rator_out: &Core, _rand: &Core) -> Result<Core> {
        let rand_out = check(_ctx, _r, _rand, &self.arg_type)?;
        Ok(Core::the(
            read_back_type(_ctx, &self.res_type.val_of(val_in_ctx(_ctx, &rand_out))),
            Core::app((*rator_out).clone(), rand_out),
        ))
    }

    fn check(&self, ctx: &Ctx, r: &Renaming, e: &Core, _tv: &Value) -> Result<Core> {
        match e {
            Core::Lambda(x, b) => {
                let x_hat = fresh(ctx, x);
                let b_out = check(
                    &ctx.bind_free(x_hat.clone(), self.arg_type.clone())?,
                    &r.extend(x.clone(), x_hat.clone()),
                    b,
                    &self.res_type.val_of(values::neutral(
                        self.arg_type.clone(),
                        N::Var(x_hat.clone()),
                    )),
                )?;
                Ok(Core::lambda(x_hat, b_out))
            }
            _ => Err(Error::NotAFunction(e.clone())),
        }
    }
}

impl ValueInterface for Lambda {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn ValueInterface) -> bool {
        unimplemented!()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Err(Error::NotATypeVar(lambda(
            self.arg_name.clone(),
            self.body.clone(),
        )))
    }
}

pub fn do_ap(rator: &Value, rand: Value) -> Value {
    match now(rator).as_any().downcast_ref::<Lambda>() {
        Some(Lambda { body, .. }) => body.val_of(rand),
        None => todo!("{:?}", rator),
    }
}
