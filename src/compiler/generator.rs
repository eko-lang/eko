use eko_gc::Arena;

use crate::core::fun::{Chunk, Instr};

pub struct ChunkBuilder {
    local_scope_len: usize,
    instrs: Vec<Instr>,
}

impl ChunkBuilder {
    pub fn new() -> ChunkBuilder {
        ChunkBuilder {
            local_scope_len: 0,
            instrs: Vec::new(),
        }
    }

    pub fn next_var(&mut self) -> usize {
        let local_scope_len = self.local_scope_len;
        self.local_scope_len += 1;
        local_scope_len
    }

    pub fn instr(&mut self, instr: Instr) {
        self.instrs.push(instr);
    }

    pub fn build<'gc>(self, arena: &Arena<'gc>) -> Chunk<'gc> {
        Chunk::new(arena, self.local_scope_len, self.instrs)
    }
}
