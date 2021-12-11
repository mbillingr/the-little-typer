use crate::symbol::Symbol;
use sexpr_parser::SexprFactory;

#[derive(Debug)]
pub enum Sexpr {
    Symbol(Symbol),
    List(Vec<Sexpr>),
}

impl SexprFactory for Sexpr {
    type Sexpr = Sexpr;
    type Integer = i64;
    type Float = f64;

    fn int(_: i64) -> Self::Sexpr {
        unimplemented!()
    }

    fn float(_: f64) -> Self::Sexpr {
        unimplemented!()
    }

    fn symbol(s: &str) -> Self::Sexpr {
        Sexpr::Symbol(Symbol::new(s))
    }

    fn string(_: &str) -> Self::Sexpr {
        unimplemented!()
    }

    fn list(items: Vec<Self::Sexpr>) -> Self::Sexpr {
        Sexpr::List(items)
    }

    fn pair(_: Self::Sexpr, _: Self::Sexpr) -> Self::Sexpr {
        unimplemented!()
    }
}
