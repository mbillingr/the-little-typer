use crate::basics::{Core, CoreInterface, Ctx, Renaming};
use crate::errors;
use crate::normalize::val_in_ctx;
use crate::symbol::Symbol;
macro_rules! pi_type {
    ((), $ret:expr) => {$ret};

    ((($x:ident, $arg_t:expr) $($b:tt)*), $ret:expr) => {
        values::pi(stringify!($x), $arg_t, Closure::higher(move |$x| pi_type!(($($b)*), $ret)))
    };
}

macro_rules! impl_core_defaults {
    ($fields:tt, as_any) => {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    };

    ($fields:tt, no_type) => {
        fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> crate::errors::Result<Core> {
            Err(Error::NotAType(Core::new(self.clone())))
        }
    };

    ($fields:tt, simple_type) => {
        fn is_type(&self, _ctx: &Ctx, _r: &Renaming) -> crate::errors::Result<Core> {
            Ok(Core::new(self.clone()))
        }
    };

    ($fields:tt, no_synth) => {
        fn synth(&self, _ctx: &Ctx, _r: &Renaming) -> crate::errors::Result<(Core, Core)> {
            Err(Error::CantDetermineType(Core::new(self.clone())))
        }
    };

    ($fields:tt, check_by_synth) => {
        fn check(&self, ctx: &Ctx, r: &Renaming, tv: &Value) -> crate::errors::Result<Core> {
            let (t_out, e_out) = self.synth(ctx, r)?;
            crate::typechecker::same_type(ctx, &crate::normalize::val_in_ctx(ctx, &t_out), tv)?;
            Ok(e_out)
        }
    };

    // no fields at all - every instance is the same
    (_, same) => {
        fn same(&self, other: &dyn CoreInterface) -> bool {
            other.as_any().is::<Self>()
        }
    };

    ($fields:tt, same) => {
        fn same(&self, other: &dyn CoreInterface) -> bool {
            other
                .as_any()
                .downcast_ref::<Self>()
                .map(|o| self == o)
                .unwrap_or(false)
        }
    };

    (_, occurring_names) => {
        fn occurring_names(&self) -> HashSet<Symbol> {
            HashSet::new()
        }
    };

    (($($field:tt),*), occurring_names) => {
        fn occurring_names(&self) -> HashSet<Symbol> {
            let names = HashSet::new();
            $(let names = &names | &self.$field.occurring_names();)*
            names
        }
    };

    (_, alpha_equiv) => {
        fn alpha_equiv_aux(&self,
                           other: &dyn CoreInterface,
                           _lvl: usize,
                           _b1: &crate::alpha::Bindings,
                           _b2: &crate::alpha::Bindings)
                        -> bool {
            CoreInterface::same(self, other)
        }
    };

    (($($field:tt),*), alpha_equiv) => {
        fn alpha_equiv_aux(&self,
                           other: &dyn CoreInterface,
                           lvl: usize,
                           b1: &crate::alpha::Bindings,
                           b2: &crate::alpha::Bindings)
                        -> bool {
            if let Some(other) = other.as_any().downcast_ref::<Self>() {
                let eq = true;
                $(let eq = eq && self.$field.alpha_equiv_aux(&other.$field, lvl, b1, b2);)*
                eq
            } else {
                false
            }
        }
    };

    ($_:tt, no_alpha_equiv) => {
        fn alpha_equiv_aux(&self,
                           _other: &dyn CoreInterface,
                           _lvl: usize,
                           _b1: &crate::alpha::Bindings,
                           _b2: &crate::alpha::Bindings)
                        -> bool {
            unimplemented!()
        }
    };

    ($fields:tt, $unknown:tt) => {
        fn $unknown() { 0 }  // cheap way to raise an error
    };

    ($fields:tt, $($more:tt),+) => {
        $(impl_core_defaults!($fields, $more);)+
    }
}

mod annotation;
mod atom;
pub mod cores;
mod delay;
pub mod functions;
pub mod natural;
mod neutral;
mod pairs;
pub mod reference;
mod universe;
pub mod values;
mod invalid;

fn is_type_with_fresh_binding<T: CoreInterface>(
    ctx: &Ctx,
    r: &Renaming,
    x: &Symbol,
    x_type: &Core,
    body: &T,
) -> errors::Result<(Symbol, Core, Core)> {
    let x_hat = ctx.fresh(x);
    let a_out = x_type.is_type(ctx, r)?;
    let a_outv = val_in_ctx(ctx, &a_out);
    let b_out = body.is_type(
        &ctx.bind_free(x_hat.clone(), a_outv)?,
        &r.extend(x.clone(), x_hat.clone()),
    )?;
    Ok((x_hat, a_out, b_out))
}

fn check_with_fresh_binding<T: CoreInterface>(
    ctx: &Ctx,
    r: &Renaming,
    x: &Symbol,
    x_type: &Core,
    body: &T,
) -> errors::Result<(Symbol, Core, Core)> {
    let x_hat = ctx.fresh(x);
    let a_out = x_type.check(ctx, r, &values::universe())?;
    let ctx_hat = ctx.bind_free(x_hat.clone(), val_in_ctx(ctx, &a_out))?;
    let r_hat = r.extend(x.clone(), x_hat.clone());
    let b_out = body.check(&ctx_hat, &r_hat, &values::universe())?;
    Ok((x_hat, a_out, b_out))
}
