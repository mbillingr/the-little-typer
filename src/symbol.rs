use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Symbol(Arc<str>);

impl Symbol {
    pub fn new(s: &str) -> Self {
        Symbol(s.into())
    }

    pub fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

impl From<String> for Symbol {
    fn from(s: String) -> Self {
        Symbol(s.into())
    }
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Self {
        Symbol(s.into())
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        &*self.0
    }
}

impl Deref for Symbol {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl Borrow<str> for Symbol {
    fn borrow(&self) -> &str {
        &*self.0
    }
}

impl PartialEq<&str> for Symbol {
    fn eq(&self, other: &&str) -> bool {
        *self.0 == **other
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
