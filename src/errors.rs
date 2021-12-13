use crate::basics::{Core, Ctx};
use crate::symbol::Symbol;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidAtom(Symbol),
    WrongType(Core, Core),
    AlreadyBound(Symbol, Ctx),
    UhasNoType,
    NotAType(Core),
    NotAFunctionType(Core),
    UnknownVariable(Symbol),
    NotTheSame(Core, Core, Core),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidAtom(s) => write!(f, "Invalid atom: {}", s.name()),
            Error::WrongType(actual, expected) => {
                write!(f, "Expected type {} but got {}", expected, actual)
            }
            Error::AlreadyBound(s, ctx) => {
                write!(f, "Name {} is already bound in context {:?}", s.name(), ctx)
            }
            Error::UhasNoType => write!(f, "U is a type, but does not have a type."),
            Error::NotAType(t) => write!(f, "Not a type: {}", t),
            Error::NotAFunctionType(t) => write!(f, "Not a function type: {}", t),
            Error::UnknownVariable(name) => write!(f, "Unknown variable {}", name.name()),
            Error::NotTheSame(t, a, b) => {
                write!(f, "The expressions {} and {} are not the same {}", a, b, t)
            }
        }
    }
}
