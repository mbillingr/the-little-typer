use crate::basics::Core;
use crate::symbol::Symbol;
use crate::types::annotation::The;
use crate::types::atom::{Atom, Quote};
use crate::types::functions::{App, AppStar, Fun, Lambda, Pi, PiStar};
use crate::types::natural::{Add1, IndNat, Nat, WhichNat, Zero};
use crate::types::pairs::{Cons, Pair, Sigma};
use crate::types::reference::Ref;
use crate::types::universe::Universe;

pub fn the(typ: Core, exp: Core) -> Core {
    Core::new(The { typ, exp })
}

pub fn universe() -> Core {
    Core::new(Universe)
}

pub fn nat() -> Core {
    Core::new(Nat)
}

pub fn zero() -> Core {
    Core::new(Zero)
}

pub fn add1(n: Core) -> Core {
    Core::new(Add1(n))
}

pub fn which_nat(target: Core, base: Core, step: Core) -> Core {
    Core::new(WhichNat::untyped(target, base, step))
}

pub fn ind_nat(target: Core, motive: Core, base: Core, step: Core) -> Core {
    Core::new(IndNat::new(target, motive, base, step))
}

pub fn which_nat_desugared(target: Core, base_type: Core, base: Core, step: Core) -> Core {
    Core::new(WhichNat::typed(target, base_type, base, step))
}

pub fn atom() -> Core {
    Core::new(Atom)
}

pub fn quote(s: impl Into<Symbol>) -> Core {
    Core::new(Quote(s.into()))
}

pub fn fun(ts: Vec<Core>) -> Core {
    Core::new(Fun(ts))
}

pub fn pi(x: impl Into<Symbol>, arg_type: Core, res_type: Core) -> Core {
    Core::new(Pi {
        arg_name: x.into(),
        arg_type,
        res_type,
    })
}

pub fn pi_star(binders: Vec<(Symbol, Core)>, res_type: Core) -> Core {
    Core::new(PiStar { binders, res_type })
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

pub fn app_star(fun: Core, args: Vec<Core>) -> Core {
    Core::new(AppStar { fun, args })
}

pub fn refer(s: impl Into<Symbol>) -> Core {
    Core::new(Ref::new(s))
}

pub fn sigma(x: impl Into<Symbol>, car_type: Core, cdr_type: Core) -> Core {
    Core::new(Sigma {
        arg_name: x.into(),
        car_type,
        cdr_type,
    })
}

pub fn pair(a: Core, d: Core) -> Core {
    Core::new(Pair(a, d))
}

pub fn cons(car: Core, cdr: Core) -> Core {
    Core::new(Cons(car, cdr))
}
