use crate::basics::{Core, Ctx, Env, Renaming, Value, ValueInterface, N};
use crate::errors::Result;
use crate::normalize::{now, val_of};
use std::any::Any;
use std::borrow::Cow;
use std::ops::Deref;
use std::sync::{Arc as R, Mutex, MutexGuard};

#[derive(Debug)]
pub struct Delay {
    value: SharedBox<Delayed>,
}

impl Delay {
    pub fn new(env: Env, exp: Core) -> Self {
        Delay {
            value: SharedBox::new(Delayed::Later(env, exp)),
        }
    }

    fn force(&self) -> Value {
        let mut dv = self.value.write_lock();

        if let Delayed::Value(x) = &*dv {
            return x.clone();
        }

        let the_value = undelay(&dv);
        dv.replace(Delayed::Value(the_value.clone()));
        the_value
    }
}

fn undelay(c: &SharedBoxGuard<Delayed>) -> Value {
    match &**c {
        Delayed::Later(env, exp) => now(&val_of(env, exp)).into_owned(),
        _ => unreachable!(),
    }
}

impl ValueInterface for Delay {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn ValueInterface) -> bool {
        unimplemented!()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        unimplemented!()
    }

    fn read_back(&self, _ctx: &Ctx, _tv: &Value, _v: &Value) -> Result<Core> {
        unimplemented!()
    }

    fn check(&self, _ctx: &Ctx, _r: &Renaming, _e: &Core, _tv: &Value) -> Result<Core> {
        unimplemented!()
    }

    fn now<'a>(&self, _v: &'a Value) -> Cow<'a, Value> {
        Cow::Owned(self.force())
    }

    fn as_neutral(&self) -> Option<(&Value, &N)> {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq)]
enum Delayed {
    Value(Value),
    Later(Env, Core),
}

#[derive(Debug)]
struct SharedBox<T>(R<Mutex<T>>);

struct SharedBoxGuard<'a, T: 'a>(MutexGuard<'a, T>);

impl<T> Clone for SharedBox<T> {
    fn clone(&self) -> Self {
        SharedBox(self.0.clone())
    }
}

impl<T> SharedBox<T> {
    pub fn new(inner: T) -> Self {
        SharedBox(R::new(Mutex::new(inner)))
    }

    pub fn write_lock(&self) -> SharedBoxGuard<T> {
        SharedBoxGuard(self.0.lock().unwrap())
    }

    pub fn read_lock(&self) -> SharedBoxGuard<T> {
        self.write_lock()
    }
}

impl<T: PartialEq> std::cmp::PartialEq for SharedBox<T> {
    fn eq(&self, other: &Self) -> bool {
        let a = self.read_lock();
        let b = other.read_lock();
        *a == *b
    }
}

impl<'a, T: 'a> Deref for SharedBoxGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'a, T: 'a> SharedBoxGuard<'a, T> {
    pub fn replace(&mut self, value: T) -> T {
        std::mem::replace(&mut *self.0, value)
    }
}
