use crate::basics::{R, Value};

mod universe;

use universe::Universe;

pub fn universe() -> Value {
    Value::Obj(R::new(Universe))
}
