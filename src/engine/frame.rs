use eko_gc::{Arena, Gc, RefCell};

use crate::core::fun::{Chunk, Instr};
use crate::core::value::Value;

use super::error::{Error, Result};

pub struct Frame<'gc> {
    cur_instr_index: usize,
    chunk: Chunk<'gc>,
    local_scope: Scope<'gc>,
    captured_scope: Option<CapturedScope<'gc>>,
}

impl<'gc> Frame<'gc> {
    pub fn new(arena: &Arena<'gc>, chunk: Chunk<'gc>) -> Frame<'gc> {
        Frame {
            cur_instr_index: 0,
            local_scope: Scope::new(arena, chunk.local_scope_len()),
            chunk,
            captured_scope: None,
        }
    }

    pub fn with_captured_scope(
        arena: &Arena<'gc>,
        chunk: Chunk<'gc>,
        captured_scope: CapturedScope<'gc>,
    ) -> Frame<'gc> {
        Frame {
            cur_instr_index: 0,
            local_scope: Scope::new(arena, chunk.local_scope_len()),
            chunk,
            captured_scope: Some(captured_scope),
        }
    }

    pub fn step(&mut self) -> Option<Instr> {
        if let Some(instr) = self.chunk.instr(self.cur_instr_index) {
            self.cur_instr_index += 1;
            Some(instr)
        } else {
            None
        }
    }

    pub fn local_scope(&self) -> &Scope<'gc> {
        &self.local_scope
    }
}

#[derive(Clone, Trace)]
pub struct CapturedScope<'gc>(Gc<'gc, RefCell<'gc, CapturedScopeData<'gc>>>);

#[derive(Trace)]
pub struct CapturedScopeData<'gc> {
    parent_scope: Option<CapturedScope<'gc>>,
    captured_scope_len: usize,
    scope: Scope<'gc>,
}

#[derive(Trace)]
pub struct Scope<'gc>(Gc<'gc, RefCell<'gc, Vec<Value<'gc>>>>);

impl<'gc> Scope<'gc> {
    pub fn new(arena: &Arena<'gc>, len: usize) -> Scope<'gc> {
        Scope(Gc::new(
            arena,
            // TODO: Figure out how to represent `None`.
            RefCell::new(arena, vec![Value::Boolean(false); len]),
        ))
    }

    pub fn set(&self, var: usize, value: Value<'gc>) -> Result<'gc, ()> {
        *self
            .0
            .borrow_mut()
            .get_mut(var)
            .ok_or_else(|| Error::InvalidVar { var })? = value;
        Ok(())
    }

    pub fn get(&self, var: usize) -> Result<'gc, Value<'gc>> {
        self.0
            .borrow()
            .get(var)
            .cloned()
            .ok_or_else(|| Error::InvalidVar { var })
    }
}
