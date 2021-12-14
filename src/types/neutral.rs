use crate::basics::{Core, Ctx, Value, ValueInterface, N};
use crate::errors::Result;
use crate::normalize::read_back_neutral;
use std::any::Any;

pub fn neutral(type_value: Value, kind: N) -> Value {
    Value::new(Neutral { type_value, kind })
}

#[derive(Debug)]
pub struct Neutral {
    pub type_value: Value,
    pub kind: N,
}

impl ValueInterface for Neutral {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn ValueInterface) -> bool {
        unimplemented!()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        todo!()
    }

    fn read_back(&self, ctx: &Ctx, _tv: &Value, _v: &Value) -> Result<Core> {
        Ok(read_back_neutral(ctx, &self.kind))
    }

    fn as_neutral(&self) -> Option<(&Value, &N)> {
        Some((&self.type_value, &self.kind))
    }
}
