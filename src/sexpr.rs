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

impl PartialEq<u64> for Sexpr {
    fn eq(&self, other: &u64) -> bool {
        match self {
            Sexpr::SmallNat(n) => n == other,
            _ => false,
        }
    }
}

impl PartialEq<u64> for &Sexpr {
    fn eq(&self, other: &u64) -> bool {
        (*self) == other
    }
}

impl PartialEq<str> for Sexpr {
    fn eq(&self, other: &str) -> bool {
        match self {
            Sexpr::Symbol(s) => s.name() == other,
            _ => false,
        }
    }
}

impl PartialEq<&str> for Sexpr {
    fn eq(&self, other: &&str) -> bool {
        self == *other
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

pub trait MaybeList {
    type Head;
    type Tail: MaybeList + ?Sized;
    fn is_list(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn head(&self) -> Option<&Self::Head>;
    fn tail(&self) -> Option<&Self::Tail>;

    fn decons(&self) -> Option<(&Self::Head, &Self::Tail)> {
        self.head().and_then(|h| self.tail().map(|t| (h, t)))
    }
}

impl<T: MaybeList> MaybeList for &T {
    type Head = T::Head;
    type Tail = T::Tail;

    fn is_list(&self) -> bool {
        (*self).is_list()
    }

    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }

    fn head(&self) -> Option<&Self::Head> {
        todo!()
    }

    fn tail(&self) -> Option<&Self::Tail> {
        todo!()
    }
}

impl MaybeList for Sexpr {
    type Head = Self;
    type Tail = [Self];

    fn is_list(&self) -> bool {
        match self {
            Sexpr::List(_) => true,
            _ => false,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Sexpr::List(l) => l.is_empty(),
            _ => false,
        }
    }

    fn head(&self) -> Option<&Self::Head> {
        match self {
            Sexpr::List(l) => l.first(),
            _ => None,
        }
    }

    fn tail(&self) -> Option<&Self::Tail> {
        match self {
            Sexpr::List(l) => Some(&l[1..]),
            _ => None,
        }
    }
}

impl MaybeList for [Sexpr] {
    type Head = Sexpr;
    type Tail = Self;

    fn is_list(&self) -> bool {
        true
    }

    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }

    fn head(&self) -> Option<&Self::Head> {
        self.first()
    }

    fn tail(&self) -> Option<&Self::Tail> {
        match self {
            [_, rest @ ..] => Some(rest),
            [] => None,
        }
    }
}

#[macro_export]
macro_rules! match_sexpr {
    ($expr:expr, else => $then:expr,) => {$then};

    ($expr:expr, case _ => $then:expr, $($rest:tt)*) => {$then};

    ($expr:expr, case $var:ident => $then:expr, $($rest:tt)*) => {{
        let $var = $expr;
        $then
    }};

    ($expr:expr, case () => $then:expr, $($rest:tt)*) => {
        if $crate::sexpr::MaybeList::is_empty($expr) {
            $then
        } else {
            match_sexpr! { $expr, $($rest)* }
        }
    };

    ($expr:expr, case ($item:tt) => $then:expr, $($rest:tt)*) => {{
        let result = if let Some((_h, _t)) = $crate::sexpr::MaybeList::decons($expr) {
            match_sexpr!(
                _h,
                case $item => match_sexpr!(
                    _t,
                    case () => Some($then),
                    else => None,
                ),
                else => None,
            )
        } else {
            None
        };
        match result {
            Some(r) => r,
            None => match_sexpr! { $expr, $($rest)* },
        }
    }};

    ($expr:expr, case ($item:tt, $($more:tt)*) => $then:expr, $($rest:tt)*) => {{
        let result = if let Some((_h, _t)) = $crate::sexpr::MaybeList::decons($expr) {
            match_sexpr!(
                _h,
                case $item => match_sexpr!(
                    _t,
                    case ($($more)*) => Some($then),
                    else => None,
                ),
                else => None,
            )
        } else {
            None
        };
        match result {
            Some(r) => r,
            None => match_sexpr! { $expr, $($rest)* },
        }
    }};

    ($expr:expr, case [$pat:pat] => $then:expr, $($rest:tt)*) => {
        if let $pat = $expr {
            $then
        }else {
            match_sexpr! { $expr, $($rest)* }
        }
    };

    ($expr:expr, case $literal:expr => $then:expr, $($rest:tt)*) => {
        if $expr == $literal {
            $then
        } else {
            match_sexpr! { $expr, $($rest)* }
        }
    };
}

pub fn match_empty<R>(exp: &(impl MaybeList + ?Sized), body: impl Fn() -> R) -> Option<R> {
    if exp.is_empty() {
        Some(body())
    } else {
        None
    }
}

pub fn match_list<'l, L: MaybeList + ?Sized, R>(
    exp: &'l L,
    body: impl Fn(&'l L::Head, &'l L::Tail) -> Option<R>,
) -> Option<R> {
    exp.decons().and_then(|(h, t)| body(h, t))
}

#[cfg(test)]
mod tests {
    use super::Sexpr;
    use sexpr_parser::SexprFactory;

    #[test]
    fn match_anything() {
        assert!(match_sexpr! {
            Sexpr::symbol("foo"),
            case _ => true,
        })
    }

    #[test]
    fn match_pattern() {
        assert!(match_sexpr! {
            Sexpr::symbol("foo"),
            case [Sexpr::Symbol(_)] => true,
            else => false,
        })
    }

    #[test]
    fn match_literal() {
        assert!(match_sexpr! {
            &Sexpr::int(42),
            case &Sexpr::int(42) => true,
            else => false,
        });

        assert!(match_sexpr! {
            &Sexpr::symbol("foo"),
            case &Sexpr::symbol("bar") => false,
            else => true,
        });
    }

    #[test]
    fn match_literal_symbol() {
        assert!(match_sexpr! {
            &Sexpr::symbol("foo"),
            case "foo" => true,
            else => false,
        });

        assert!(match_sexpr! {
            &Sexpr::symbol("foo"),
            case "bar" => false,
            else => true,
        });
    }

    #[test]
    fn match_binds_identifier() {
        assert!(match_sexpr! {
            &Sexpr::symbol("foo"),
            case x => x == "foo",
        })
    }

    #[test]
    fn match_empty_list() {
        assert!(match_sexpr! {
            &Sexpr::list(vec![]),
            case () => true,
            else => false,
        });

        assert!(match_sexpr! {
            &Sexpr::symbol("foo"),
            case () => false,
            else => true,
        });

        assert!(match_sexpr! {
            &Sexpr::list(vec![Sexpr::symbol("foo")]),
            case () => false,
            else => true,
        });
    }

    #[test]
    fn match_exact_list() {
        assert!(match_sexpr! {
            &Sexpr::list(vec![Sexpr::int(1)]),
            case (1) => true,
            else => false,
        });

        assert!(match_sexpr! {
            &Sexpr::list(vec![Sexpr::symbol("foo"), Sexpr::int(1)]),
            case ("foo", 1) => true,
            else => false,
        });
    }

    #[test]
    fn match_bind_list_items() {
        let expr = Sexpr::list(vec![Sexpr::int(1), Sexpr::int(2), Sexpr::int(3)]);

        assert!(match_sexpr! {
            &expr,
            case (_) => panic!("unexpected match"),
            else => true,
        });

        /*assert_eq!(
            match_sexpr! {
                &expr,
                case (_, 2, y) => y,
                else => panic!(""),
            },
            &Sexpr::int(3)
        );*/
    }
}
