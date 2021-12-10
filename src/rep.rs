use crate::basics::{Core, Ctx, Renaming};
use crate::errors::Result;
use crate::normalize::{read_back, read_back_type, val_in_ctx};
use crate::typechecker::synth;

pub fn rep(ctx: &Ctx, e: &Core) -> Result<Core> {
    if let Core::The(t_out, e_out) = synth(ctx, &Renaming::new(), e)? {
        let tv = val_in_ctx(ctx, &t_out);
        let v = val_in_ctx(ctx, &e_out);
        Ok(Core::the(read_back_type(ctx, &tv), read_back(ctx, &tv, &v)))
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Core::*;

    static CTX: Ctx = Ctx::new();

    #[test]
    fn just_an_atom() {
        assert_eq!(
            rep(&CTX, &Core::quote("atom")),
            Ok(Core::the(Atom, Core::quote("atom")))
        );
    }

    #[test]
    fn just_a_type() {
        assert_eq!(rep(&CTX, &Atom), Ok(Core::the(U, Atom)));
    }
}
