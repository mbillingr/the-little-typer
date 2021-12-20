use crate::symbol::Symbol;
use sexpr_parser::SexprFactory;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Sexpr {
    Invalid(String),
    SmallNat(u64),
    Symbol(Symbol),
    List(Vec<Sexpr>),
}

impl Sexpr {
    pub fn as_symbol(&self) -> Option<&Symbol> {
        match self {
            Sexpr::Symbol(s) => Some(s),
            _ => None,
        }
    }
}

impl SexprFactory for Sexpr {
    type Sexpr = Sexpr;
    type Integer = u64;
    type Float = f64;

    fn int(x: u64) -> Self::Sexpr {
        Sexpr::SmallNat(x)
    }

    fn float(x: f64) -> Self::Sexpr {
        Sexpr::Invalid(x.to_string())
    }

    fn symbol(s: &str) -> Self::Sexpr {
        Sexpr::Symbol(Symbol::new(s))
    }

    fn string(s: &str) -> Self::Sexpr {
        Sexpr::Invalid(format!("\"{}\"", s))
    }

    fn list(items: Vec<Self::Sexpr>) -> Self::Sexpr {
        Sexpr::List(items)
    }

    fn pair(a: Self::Sexpr, d: Self::Sexpr) -> Self::Sexpr {
        Sexpr::Invalid(format!("({} . {})", a, d))
    }
}

impl Display for Sexpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sexpr::Invalid(s) => write!(f, "{}", s),
            Sexpr::SmallNat(x) => write!(f, "{}", x),
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
