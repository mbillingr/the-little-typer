use crate::basics::Core;
use crate::symbol::Symbol;
use crate::types::annotation::The;
use crate::types::atom::{Atom, Quote};
use crate::types::functions::{App, AppStar, Fun, Lambda, LambdaStar, Pi, PiStar};
use crate::types::invalid::Invalid;
use crate::types::lists::{List, ListCons, Nil, RecList};
use crate::types::natural::{Add1, IndNat, IterNat, Nat, RecNat, WhichNat, Zero};
use crate::types::pairs::{Car, Cdr, Cons, Pair, Sigma, SigmaStar};
use crate::types::reference::Ref;
use crate::types::todo::ToDo;
use crate::types::universe::Universe;
use crate::types::vec::{Head, Tail, VecNil, Vector, VectorCons};

pub fn invalid_syntax(s: &str) -> Core {
    Core::new(Invalid(s.into()))
}

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

pub fn the_nat(n: u64) -> Core {
    let mut out = zero();
    for _ in 0..n {
        out = add1(out);
    }
    out
}

pub fn which_nat(target: Core, base: Core, step: Core) -> Core {
    Core::new(WhichNat::untyped(target, base, step))
}

pub fn which_nat_desugared(target: Core, base_type: Core, base: Core, step: Core) -> Core {
    Core::new(WhichNat::typed(target, base_type, base, step))
}

pub fn iter_nat(target: Core, base: Core, step: Core) -> Core {
    Core::new(IterNat::untyped(target, base, step))
}

pub fn iter_nat_desugared(target: Core, base_type: Core, base: Core, step: Core) -> Core {
    Core::new(IterNat::typed(target, base_type, base, step))
}

pub fn rec_nat(target: Core, base: Core, step: Core) -> Core {
    Core::new(RecNat::untyped(target, base, step))
}

pub fn rec_nat_desugared(target: Core, base_type: Core, base: Core, step: Core) -> Core {
    Core::new(RecNat::typed(target, base_type, base, step))
}

pub fn ind_nat(target: Core, motive: Core, base: Core, step: Core) -> Core {
    Core::new(IndNat::new(target, motive, base, step))
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

pub fn lambda_star(params: Vec<Symbol>, body: Core) -> Core {
    Core::new(LambdaStar { params, body })
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

pub fn sigma_star(binders: Vec<(Symbol, Core)>, res_type: Core) -> Core {
    Core::new(SigmaStar {
        binders,
        cdr_type: res_type,
    })
}

pub fn pair(a: Core, d: Core) -> Core {
    Core::new(Pair(a, d))
}

pub fn cons(car: Core, cdr: Core) -> Core {
    Core::new(Cons(car, cdr))
}

pub fn car(cons: Core) -> Core {
    Core::new(Car(cons))
}

pub fn cdr(cons: Core) -> Core {
    Core::new(Cdr(cons))
}

pub fn list(t: Core) -> Core {
    Core::new(List(t))
}

pub fn nil() -> Core {
    Core::new(Nil)
}

pub fn list_cons(head: Core, tail: Core) -> Core {
    Core::new(ListCons(head, tail))
}

pub fn rec_list(target: Core, base: Core, step: Core) -> Core {
    Core::new(RecList::untyped(target, base, step))
}

pub fn rec_list_desugared(target: Core, base_type: Core, base: Core, step: Core) -> Core {
    Core::new(RecList::typed(target, base_type, base, step))
}

pub fn vec(t: Core, n: Core) -> Core {
    Core::new(Vector(t, n))
}

pub fn vecnil() -> Core {
    Core::new(VecNil)
}

pub fn vec_cons(head: Core, tail: Core) -> Core {
    Core::new(VectorCons(head, tail))
}

pub fn head(vec: Core) -> Core {
    Core::new(Head(vec))
}

pub fn tail(vec: Core) -> Core {
    Core::new(Tail(vec))
}

pub fn todo(name: impl Into<Symbol>) -> Core {
    Core::new(ToDo::new(name))
}

pub fn annotated_todo(name: impl Into<Symbol>, typ: Core) -> Core {
    Core::new(ToDo::annotated(name, typ))
}
