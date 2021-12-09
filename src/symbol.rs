use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Symbol(Rc<str>);

impl Symbol {
    pub fn new(s: &str) -> Self {
        Symbol(s.into())
    }

    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl From<String> for Symbol {
    fn from(s: String) -> Self {
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
