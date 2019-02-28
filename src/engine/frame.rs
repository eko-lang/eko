use eko_gc::Arena;

use crate::core::{CapturedScope, Chunk, Instr, Scope};

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
            local_scope: Scope::new(arena, chunk.vars_len()),
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
            local_scope: Scope::new(arena, chunk.vars_len()),
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
}
