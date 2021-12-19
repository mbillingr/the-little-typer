mod app;
mod fun;
mod lambda;
mod pi;

use crate::basics::{Closure, The, Value, ValueInterface};
use crate::normalize::now;
use crate::types::neutral::Neutral;
use crate::types::values::neutral;

pub use app::{App, AppStar, NeutralApp};
pub use fun::Fun;
pub use lambda::{Lambda, LambdaStar};
pub use pi::{Pi, PiStar};

pub fn do_ap(rator: &Value, rand: Value) -> Value {
    match now(rator).as_any().downcast_ref::<Lambda<Closure>>() {
        Some(Lambda { body, .. }) => return body.val_of(rand),
        None => {}
    }

    match now(rator).as_any().downcast_ref::<Neutral>() {
        Some(neu) => {
            if let Some(pi) = now(&neu.type_value)
                .as_any()
                .downcast_ref::<Pi<Value, Closure>>()
            {
                neutral(
                    pi.res_type.val_of(rand.clone()),
                    NeutralApp(neu.kind.clone(), The(pi.arg_type.clone(), rand)),
                )
            } else {
                todo!()
            }
        }
        None => todo!("{:?}", now(rator)),
    }
}
