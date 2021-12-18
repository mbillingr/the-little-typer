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
                           _b1: &alpha::Bindings,
                           _b2: &alpha::Bindings)
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
mod natural;
mod neutral;
mod pairs;
mod reference;
mod universe;
pub mod values;
