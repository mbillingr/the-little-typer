mod app;
mod lambda;
mod pi;

use crate::basics::{Closure, Value, ValueInterface, N};
use crate::normalize::now;
use crate::types::neutral::Neutral;
use crate::types::values::neutral;

pub use app::App;
pub use lambda::Lambda;
pub use pi::Pi;

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
                    N::app(neu.kind.clone(), pi.arg_type.clone(), rand),
                )
            } else {
                todo!()
            }
        }
        None => todo!("{:?}", now(rator)),
    }
}
