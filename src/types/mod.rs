macro_rules! pi_type {
    ((), $ret:expr) => {$ret};

    ((($x:ident, $arg_t:expr) $($b:tt)*), $ret:expr) => {
        values::pi(stringify!($x), $arg_t, Closure::higher(move |$x| pi_type!(($($b)*), $ret)))
    };
}

macro_rules! impl_core_defaults {
    (as_any) => {
        fn as_any(&self) -> &dyn Any {
            self
        }
    };

    (same) => {
        fn same(&self, other: &dyn CoreInterface) -> bool {
            other
                .as_any()
                .downcast_ref::<Self>()
                .map(|o| self == o)
                .unwrap_or(false)
        }
    };

    ((same unique)) => {
        fn same(&self, other: &dyn CoreInterface) -> bool {
            other.as_any().is::<Self>()
        }
    };

    ($($more:tt),*) => {
        $(impl_core_defaults!($more);)*
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
