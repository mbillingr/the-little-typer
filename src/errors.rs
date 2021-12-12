use crate::basics::{Core, Ctx};
use crate::symbol::Symbol;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidAtom(Symbol),
    UnexpectedType(Core, Core),
    AlreadyBound(Symbol, Ctx),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidAtom(s) => write!(f, "Invalid atom: {}", s.name()),
            Error::UnexpectedType(actual, expected) => {
                write!(f, "Expected type {} but got {}", expected, actual)
            }
            Error::AlreadyBound(s, ctx) => {
                write!(f, "Name {} is already bound in context {:?}", s.name(), ctx)
            }
        }
    }
}
