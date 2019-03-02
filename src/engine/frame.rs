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

// TODO: Avoid going through two layers of `Gc` and `RefCell`.
#[derive(Clone, Trace)]
pub struct CapturedScope<'gc>(Gc<'gc, CapturedScopeData<'gc>>);

impl<'gc> CapturedScope<'gc> {
    pub fn new(
        arena: &Arena<'gc>,
        scope: Scope<'gc>,
        captured_scope_len: usize,
        parent_scope: Option<CapturedScope<'gc>>,
    ) -> CapturedScope<'gc> {
        CapturedScope(Gc::new(
            arena,
            CapturedScopeData {
                parent_scope,
                captured_scope_len,
                scope,
            },
        ))
    }

    // TODO: Make this iterative.
    pub fn set(
        &self,
        parents: usize,
        var: usize,
        value: Value<'gc>,
    ) -> Result<'gc, ()> {
        if parents == 0 {
            if let Some(parent_scope) = &self.0.parent_scope {
                return parent_scope.set(parents - 1, var, value);
            } else {
                return Err(Error::InvalidParent);
            }
        }
        if var >= self.0.captured_scope_len {
            return Err(Error::InvalidVar { var });
        }
        self.0.scope.set(var, value)
    }

    // TODO: Make this iterative.
    pub fn get(&self, parents: usize, var: usize) -> Result<'gc, Value<'gc>> {
        if parents == 0 {
            if let Some(parent_scope) = &self.0.parent_scope {
                return parent_scope.get(parents - 1, var);
            } else {
                return Err(Error::InvalidParent);
            }
        }
        if var >= self.0.captured_scope_len {
            return Err(Error::InvalidVar { var });
        }
        self.0.scope.get(var)
    }
}

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
