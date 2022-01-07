use crate::basics::{Core, CoreInterface, Ctx, Renaming};
use crate::normalize::val_in_ctx;
use crate::symbol::Symbol;
use crate::{alpha, errors};
use std::collections::HashSet;
macro_rules! pi_type {
    ((), $ret:expr) => {$ret};

    ((($x:ident, $arg_t:expr) $($b:tt)*), $ret:expr) => {
        values::pi(stringify!($x), $arg_t, crate::basics::Closure::higher(move |$x| pi_type!(($($b)*), $ret)))
    };
}

macro_rules! impl_core_defaults {
    ($fields:tt, as_any) => {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    };

    ($fields:tt, no_type) => {
        fn is_type(&self, _ctx: &crate::basics::Ctx, _r: &crate::basics::Renaming) -> crate::errors::Result<Core> {
            Err(crate::errors::Error::NotAType(Core::new(self.clone())))
        }
    };

    ($fields:tt, simple_type) => {
        fn is_type(&self, _ctx: &crate::basics::Ctx, _r: &crate::basics::Renaming) -> crate::errors::Result<Core> {
            Ok(Core::new(self.clone()))
        }
    };

    ($fields:tt, no_synth) => {
        fn synth(&self, _ctx: &crate::basics::Ctx, _r: &crate::basics::Renaming) -> crate::errors::Result<(Core, Core)> {
            Err(crate::errors::Error::CantDetermineType(Core::new(self.clone())))
        }
    };

    ($fields:tt, check_by_synth) => {
        fn check(&self, ctx: &crate::basics::Ctx, r: &crate::basics::Renaming, tv: &crate::basics::Value) -> crate::errors::Result<Core> {
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
        fn occurring_names(&self) -> std::collections::HashSet<crate::symbol::Symbol> {
            std::collections::HashSet::new()
        }
    };

    (($($field:tt),*), occurring_names) => {
        fn occurring_names(&self) -> std::collections::HashSet<crate::symbol::Symbol> {
            let names = std::collections::HashSet::new();
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

macro_rules! ternary_eliminator {
    ($name:ident, $do_func:ident, $synth_func:ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            target: Core,
            base: MaybeTyped,
            step: Core,
        }

        impl $name {
            pub fn typed(target: Core, base_t: Core, base: Core, step: Core) -> Self {
                $name {
                    target,
                    step,
                    base: MaybeTyped::The(base_t, base),
                }
            }

            pub fn untyped(target: Core, base: Core, step: Core) -> Self {
                $name {
                    target,
                    step,
                    base: MaybeTyped::Plain(base),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match &self.base {
                    MaybeTyped::Plain(base) => {
                        write!(
                            f,
                            concat!("(", stringify!($name), " {} {} {})"),
                            self.target, base, self.step
                        )
                    }
                    MaybeTyped::The(base_type, base) => write!(
                        f,
                        concat!("(", stringify!($name), " {} (the {} {}) {})"),
                        self.target, base_type, base, self.step
                    ),
                }
            }
        }

        impl CoreInterface for $name {
            impl_core_defaults!(
                (target, base, step),
                as_any,
                same,
                occurring_names,
                alpha_equiv,
                no_type,
                check_by_synth
            );

            fn val_of(&self, env: &Env) -> Value {
                match &self.base {
                    MaybeTyped::Plain(_) => {
                        unimplemented!(concat!(
                            "evaluate a desugared ",
                            stringify!($name),
                            " instead"
                        ))
                    }
                    MaybeTyped::The(bt, b) => $do_func(
                        later(env.clone(), self.target.clone()),
                        later(env.clone(), bt.clone()),
                        later(env.clone(), b.clone()),
                        later(env.clone(), self.step.clone()),
                    ),
                }
            }

            fn synth(&self, ctx: &Ctx, r: &Renaming) -> crate::errors::Result<(Core, Core)> {
                match &self.base {
                    MaybeTyped::The(_, _) => unimplemented!("already synth'ed"),
                    MaybeTyped::Plain(b) => $synth_func(self, ctx, r, b),
                }
            }

            fn resugar(&self) -> (HashSet<Symbol>, Core) {
                let tgt = self.target.resugar();
                let bas = self.base.resugar();
                let stp = self.step.resugar();
                (
                    &tgt.0 | &(&bas.0 | &stp.0),
                    Core::new($name {
                        target: tgt.1,
                        base: bas.1,
                        step: stp.1,
                    }),
                )
            }
        }
    };
}

mod annotation;
mod atom;
pub mod cores;
mod delay;
pub mod functions;
mod invalid;
mod lists;
pub mod natural;
mod neutral;
mod pairs;
pub mod reference;
mod universe;
pub mod values;
mod vec;

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

#[derive(Debug, Clone, PartialEq)]
enum MaybeTyped {
    Plain(Core),
    The(Core, Core),
}

impl MaybeTyped {
    pub fn occurring_names(&self) -> HashSet<Symbol> {
        match self {
            MaybeTyped::Plain(b) => b.occurring_names(),
            MaybeTyped::The(bt, b) => &bt.occurring_names() | &b.occurring_names(),
        }
    }

    fn alpha_equiv_aux(
        &self,
        other: &Self,
        lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool {
        match (self, other) {
            (MaybeTyped::Plain(bs1), MaybeTyped::Plain(bs2)) => {
                alpha::alpha_equiv_aux(lvl, b1, b2, bs1, bs2)
            }
            (MaybeTyped::The(bt1, bs1), MaybeTyped::The(bt2, bs2)) => {
                alpha::alpha_equiv_aux(lvl, b1, b2, bt1, bt2)
                    && alpha::alpha_equiv_aux(lvl, b1, b2, bs1, bs2)
            }
            _ => false,
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Self) {
        match self {
            MaybeTyped::Plain(b) => {
                let term = b;
                let b = term.resugar();
                (b.0, MaybeTyped::Plain(b.1))
            }
            MaybeTyped::The(bt, b) => {
                let term = bt;
                let bt = term.resugar();
                let term = b;
                let b = term.resugar();
                (&bt.0 | &b.0, MaybeTyped::The(bt.1, b.1))
            }
        }
    }
}
