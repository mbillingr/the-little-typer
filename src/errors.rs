use crate::basics::{Core, Ctx, Value, R};
use crate::symbol::Symbol;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidSyntax(R<str>),

    CantDetermineType(Core),
    InvalidAtom(Symbol),
    WrongType(Core, Core),
    AlreadyBound(Symbol, Ctx),
    UhasNoType,
    NotAType(Core),
    NotAFunctionType(Core),
    NotAFunction(Core),
    NotASigmaType(Core),
    UnknownVariable(Symbol),
    NotTheSame(Core, Core, Core),
    WrongArity(Core),

    TypeMismatchVar(Value, Value),
    NotATypeVar(Value),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidSyntax(x) => write!(f, "Invalid syntax: {}", x),
            Error::CantDetermineType(e) => write!(f, "Can't determine type of {}", e),
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
            Error::NotAFunction(e) => write!(f, "Not a function type: {}", e),
            Error::NotASigmaType(t) => write!(f, "Not a pair or sigma type: {}", t),
            Error::UnknownVariable(name) => write!(f, "Unknown variable {}", name.name()),
            Error::NotTheSame(t, a, b) => {
                write!(f, "The expressions {} and {} are not the same {}", a, b, t)
            }
            Error::WrongArity(expr) => {
                write!(f, "Wrong number of arguments: {}", expr)
            }
            Error::TypeMismatchVar(v, t) => {
                write!(f, "The value {:?} is not a {:?}", v, t)
            }
            Error::NotATypeVar(tv) => write!(f, "Not a type: {:?}", tv),
        }
    }
}
