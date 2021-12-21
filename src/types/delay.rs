use crate::basics::{Core, CoreInterface, Ctx, Env, Value, ValueInterface, N};
use crate::errors::Result;
use crate::normalize::now;
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
        now(&exp.val_of(&env)).clone()
    }
}

impl Debug for Delay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.value.get() {
            Some(x) => write!(f, "{:?}", x),
            None => write!(f, "<DELAYED VALUE>"),
        }
    }
}

impl ValueInterface for Delay {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn same(&self, _other: &dyn ValueInterface) -> bool {
        unimplemented!()
    }

    fn read_back_type(&self, ctx: &Ctx) -> Result<Core> {
        self.now().unwrap().read_back_type(ctx)
    }

    fn read_back(&self, _ctx: &Ctx, _tv: &Value, _v: &Value) -> Result<Core> {
        unimplemented!()
    }

    fn now(&self) -> Option<&Value> {
        Some(self.force())
    }

    fn as_neutral(&self) -> Option<(&Value, &N)> {
        unimplemented!()
    }
}
