use std::any::Any;
use crate::basics::{Core, Ctx, Value, ValueInterface};
use crate::errors::Result;
use crate::normalize::read_back_type;

#[derive(Debug)]
pub struct Universe;

impl ValueInterface for Universe {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        other.as_any().is::<Self>()
    }

    fn read_back_type(&self, _ctx: &Ctx) -> Result<Core> {
        Ok(Core::U)
    }

    fn read_back(&self, ctx: &Ctx, v: &Value) -> Result<Core> {
        Ok(read_back_type(ctx, v))
    }
}
