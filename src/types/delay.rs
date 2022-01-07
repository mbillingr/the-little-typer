use crate::basics::{Core, CoreInterface, Ctx, Env, Value, ValueInterface, N};
use crate::errors::Result;
use lazy_init::LazyTransform;
use std::any::Any;
use std::fmt::{Debug, Formatter};

pub struct Delay {
    value: LazyTransform<(Env, Core), Value>,
}

impl Delay {
    pub fn new(env: Env, exp: Core) -> Self {
        Delay {
            value: LazyTransform::new((env, exp)),
        }
    }

    fn force(&self) -> &Value {
        self.value.get_or_create(Self::eval_closure)
    }

    fn eval_closure((env, exp): (Env, Core)) -> Value {
        exp.val_of(&env)
    }
}

impl Debug for Delay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.force())
        /*match self.value.get() {
            Some(x) => write!(f, "{:?}", x),
            None => write!(f, "<DELAYED VALUE>"),
        }*/
    }
}

impl ValueInterface for Delay {
    fn as_any(&self) -> &dyn Any {
        self.force().as_any()
    }

    fn same(&self, other: &dyn ValueInterface) -> bool {
        self.force().same(other)
    }

    fn read_back_type(&self, ctx: &Ctx) -> Result<Core> {
        self.force().read_back_type(ctx)
    }

    fn read_back(&self, ctx: &Ctx, _tv: &Value, v: &Value) -> Result<Core> {
        self.force().read_back(ctx, v)
    }

    fn as_neutral(&self) -> Option<(&Value, &N)> {
        self.force().as_neutral()
    }
}
