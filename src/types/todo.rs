use crate::alpha;
use crate::basics::{Core, CoreInterface, Ctx, Env, NeutralInterface, Renaming, Value};
use crate::errors::Result;
use crate::symbol::Symbol;
use crate::types::values::later;
use crate::types::{cores, values};
use std::collections::HashSet;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub struct ToDo {
    name: Symbol, // in this implementation we name our todos
    typ: Option<Core>,
}

#[derive(Debug)]
pub struct NeutralTodo(Symbol, Value);

impl ToDo {
    pub fn new(name: impl Into<Symbol>) -> Self {
        ToDo {
            name: name.into(),
            typ: None,
        }
    }

    pub fn annotated(name: impl Into<Symbol>, typ: Core) -> Self {
        ToDo {
            name: name.into(),
            typ: Some(typ),
        }
    }
}

impl CoreInterface for ToDo {
    impl_core_defaults!((0, 1), as_any, same, no_synth);

    fn occurring_names(&self) -> HashSet<Symbol> {
        todo!()
    }

    fn val_of(&self, env: &Env) -> Value {
        let delayed_type = later(env.clone(), self.typ.clone().unwrap());
        values::neutral(
            delayed_type.clone(),
            NeutralTodo(self.name.clone(), delayed_type),
        )
    }

    fn check(&self, ctx: &Ctx, _r: &Renaming, tv: &Value) -> Result<Core> {
        let ty = tv.read_back_type(ctx)?;
        let todo_out = ToDo::annotated(self.name.clone(), ty);
        // super simple hacky way to report types of todos
        eprintln!("{}", todo_out);
        Ok(Core::new(todo_out))
    }

    fn alpha_equiv_aux(
        &self,
        _other: &dyn CoreInterface,
        _lvl: usize,
        _b1: &alpha::Bindings,
        _b2: &alpha::Bindings,
    ) -> bool {
        todo!()
    }

    fn resugar(&self) -> (HashSet<Symbol>, Core) {
        (
            HashSet::new(),
            cores::annotated_todo(self.name.clone(), self.typ.clone().unwrap()),
        )
    }
}

impl std::fmt::Display for ToDo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(TODO {}: {})",
            self.name.name(),
            self.typ.as_ref().unwrap()
        )
    }
}

impl NeutralInterface for NeutralTodo {
    fn read_back_neutral(&self, ctx: &Ctx) -> Result<Core> {
        Ok(cores::annotated_todo(
            self.0.clone(),
            self.1.read_back_type(ctx)?,
        ))
    }
}
