use crate::basics::{Core, Ctx, Renaming};
use crate::errors::{Error, Result};
use crate::symbol::Symbol;

pub fn is_type(_ctx: &Ctx, _renaming: &Renaming, _inp: &Core) -> Result<Core> {
    todo!()
}

pub fn synth(_ctx: &Ctx, _renaming: &Renaming, inp: &Core) -> Result<Core> {
    use Core::*;
    match inp {
        Atom => Ok(Core::the(U, Atom)),
        Quote(a) => {
            if atom_is_ok(a) {
                Ok(Core::the(Atom, Core::quote(a.clone())))
            } else {
                Err(Error::InvalidAtom(a.clone()))
            }
        }
        _ => todo!("{:?}", inp),
    }
}

pub fn atom_is_ok(_: &Symbol) -> bool {
    true
}
