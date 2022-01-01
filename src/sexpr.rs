use crate::symbol::Symbol;
use sexpr_parser::SexprFactory;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
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

impl PartialEq<&str> for Sexpr {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Sexpr::Symbol(s) => s.name() == *other,
            _ => false,
        }
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

#[macro_export]
macro_rules! match_sexpr {
    ($expr:expr, case _ => $then:expr,) => {$then};
    ($expr:expr, else => $then:expr,) => {$then};

    ($expr:expr, case $var:ident => $then:expr,) => {{
        let $var = $expr;
        $then
    }};

    ($expr:expr, case $literal:expr => $then:expr, $($rest:tt)*) => {
        if $expr == $literal {
            $then
        } else {
            match_sexpr! { $expr, $($rest)* }
        }
    };

    ($expr:expr, case $pat:pat => $then:expr, $($rest:tt)*) => {false};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_anything() {
        assert!(match_sexpr! {
            Sexpr::symbol("foo"),
            case _ => true,
        })
    }

    #[test]
    fn match_literal() {
        assert!(match_sexpr! {
            Sexpr::int(42),
            case Sexpr::int(42) => true,
            else => false,
        });

        assert!(match_sexpr! {
            Sexpr::symbol("foo"),
            case Sexpr::symbol("bar") => false,
            else => true,
        });
    }

    #[test]
    fn match_literal_symbol() {
        assert!(match_sexpr! {
            Sexpr::symbol("foo"),
            case "foo" => true,
            else => false,
        });

        assert!(match_sexpr! {
            Sexpr::symbol("foo"),
            case "bar" => false,
            else => true,
        });
    }

    #[test]
    fn match_binds_identifier() {
        assert!(match_sexpr! {
            Sexpr::symbol("foo"),
            case x => x == "foo",
        })
    }
}
