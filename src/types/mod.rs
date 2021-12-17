macro_rules! pi_type {
    ((), $ret:expr) => {$ret};

    ((($x:ident, $arg_t:expr) $($b:tt)*), $ret:expr) => {
        values::pi(stringify!($x), $arg_t, Closure::higher(move |$x| pi_type!(($($b)*), $ret)))
    };
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
