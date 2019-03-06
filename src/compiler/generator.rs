use eko_gc::Arena;

use crate::core::fun::Chunk;
use crate::core::instr::Instr;

pub struct ChunkBuilder<'gc> {
    local_scope_len: usize,
    instrs: Vec<Instr<'gc>>,
}

impl<'gc> ChunkBuilder<'gc> {
    pub fn new() -> ChunkBuilder<'gc> {
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

    pub fn instr(&mut self, instr: Instr<'gc>) {
        self.instrs.push(instr);
    }

    pub fn build(self, arena: &Arena<'gc>) -> Chunk<'gc> {
        Chunk::new(arena, self.local_scope_len, self.instrs)
    }
}
