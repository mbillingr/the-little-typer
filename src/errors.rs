use crate::symbol::Symbol;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidAtom(Symbol),
}
