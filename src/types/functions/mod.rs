mod app;
mod fun;
mod lambda;
mod pi;

use crate::basics::{Closure, The, Value};
use crate::types::neutral::Neutral;
use crate::types::values::neutral;

pub use app::{App, AppStar, NeutralApp};
pub use fun::Fun;
pub use lambda::{Lambda, LambdaStar};
pub use pi::{Pi, PiStar};

pub fn do_ap(rator: &Value, rand: Value) -> Value {
    match rator.try_as::<Lambda<Closure>>() {
        Some(Lambda { body, .. }) => return body.val_of(rand),
        None => {}
    }

    match rator.try_as::<Neutral>() {
        Some(neu) => {
            if let Some(pi) = neu.type_value.try_as::<Pi<Value, Closure>>() {
                neutral(
                    pi.res_type.val_of(rand.clone()),
                    NeutralApp(neu.kind.clone(), The(pi.arg_type.clone(), rand)),
                )
            } else {
                todo!()
            }
        }
        None => todo!("{:?}", rator),
    }
}
