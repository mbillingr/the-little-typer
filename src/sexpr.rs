use crate::symbol::Symbol;
use sexpr_parser::SexprFactory;
use std::fmt::{Display, Formatter};

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

impl Display for Sexpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sexpr::Symbol(s) => write!(f, "{}", s.name()),
            Sexpr::List(l) => {
                write!(f, "(")?;
                if !l.is_empty() {
                    write!(f, "{}", l[0])?;
                    for x in &l[1..] {
                        write!(f, " {}", x)?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}
