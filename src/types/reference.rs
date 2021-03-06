use crate::alpha;
use crate::basics::{
    is_var_name, Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, Value,
};
use crate::errors::{Error, Result};
use crate::symbol::Symbol;
use crate::types::{cores, values};
use maplit::hashset;
use std::collections::HashSet;
use std::fmt::Formatter;

/// Quotations are atoms
#[derive(Debug, Clone, PartialEq)]
pub struct Ref(Symbol);

#[derive(Debug)]
pub struct NeutralVar(pub Symbol);

impl Ref {
    pub fn new(s: impl Into<Symbol>) -> Self {
        let s = s.into();
        assert!(is_var_name(&s));
        Ref(s)
    }
}

impl CoreInterface for Ref {
    impl_core_defaults!((0), as_any, same, check_by_synth);

    fn occurring_names(&self) -> HashSet<Symbol> {
        hashset![self.0.clone()]
    }

    fn val_of(&self, env: &Env) -> Value {
        env.var_val(&self.0).unwrap()
    }

    fn is_type(&self, ctx: &Ctx, r: &Renaming) -> Result<Core> {
        match self.check(ctx, r, &values::universe()) {
            Ok(t_out) => Ok(t_out),
            Err(_) => ctx.var_type(&self.0).and_then(|other_tv| {
                Err(Error::WrongType(
                    other_tv.read_back_type(ctx)?,
                    cores::universe(),
                ))
            }),
        }
    }

    fn synth(&self, ctx: &Ctx, r: &Renaming) -> Result<(Core, Core)> {
        let real_x = r.rename(&self.0);
        let xtv = ctx.var_type(&real_x)?;
        Ok((xtv.read_back_type(ctx)?, cores::refer(real_x)))
    }

    fn alpha_equiv_aux(
        &self,
        other: &dyn CoreInterface,
        _lvl: usize,
        b1: &alpha::Bindings,
        b2: &alpha::Bindings,
    ) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let x_binding = b1.assv(&self.0);
            let y_binding = b2.assv(&other.0);
            match (x_binding, y_binding) {
                // both bound
                (Some((_, lvlx)), Some((_, lvly))) => lvlx == lvly,
                // both free
                (None, None) => self.0 == other.0,
                // one bound, one free
                (_, _) => false,
            }
        } else {
            false
        }
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        (hashset![self.0.clone()], Core::new(self.clone()))
    }
}

impl std::fmt::Display for Ref {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.name())
    }
}

impl NeutralInterface for NeutralVar {
    fn read_back_neutral(&self, _ctx: &Ctx) -> Result<Core> {
        Ok(cores::refer(self.0.clone()))
    }
}
