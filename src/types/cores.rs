use crate::basics::Core;
use crate::symbol::Symbol;
use crate::types::functions::{App, Lambda, Pi};
use crate::types::universe::Universe;

pub fn universe() -> Core {
    Core::new(Universe)
}

pub fn pi(x: impl Into<Symbol>, arg_type: Core, res_type: Core) -> Core {
    Core::new(Pi {
        arg_name: x.into(),
        arg_type,
        res_type,
    })
}

pub fn lambda(x: impl Into<Symbol>, body: Core) -> Core {
    Core::new(Lambda {
        arg_name: x.into(),
        body,
    })
}

pub fn app(fun: Core, arg: Core) -> Core {
    Core::new(App { fun, arg })
}
