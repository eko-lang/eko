use eko_gc::{Arena, Gc};

use super::ident::Ident;
use super::modu::Mod;

#[derive(Trace)]
pub struct Fn<'gc>(Gc<'gc, FnData<'gc>>);

#[derive(Trace)]
pub struct FnData<'gc> {
    modu: Mod<'gc>,
    ident: Ident<'gc>,
    arity: u8,
    method: bool,
    proto: FnProto<'gc>,
}

#[derive(Trace)]
pub enum FnProto<'gc> {
    Chunk(Chunk<'gc>),
    External(Box<dyn std::ops::FnOnce()>),
}

#[derive(Trace)]
pub struct Chunk<'gc>(Gc<'gc, ChunkData>);

impl<'gc> Chunk<'gc> {
    pub fn vars_len(&self) -> usize {
        self.0.vars_len
    }

    pub fn instr(&self, index: usize) -> Option<Instr> {
        self.0.instrs.get(index).cloned()
    }
}

#[derive(Trace)]
pub struct ChunkData {
    vars_len: usize,
    instrs: Vec<Instr>,
}

#[derive(Clone, Copy)]
pub enum Instr {}

// TODO: Move this into `compiler`.
pub struct ChunkBuilder {
    vars_len: usize,
    instrs: Vec<Instr>,
}

impl ChunkBuilder {
    pub fn new() -> ChunkBuilder {
        ChunkBuilder {
            vars_len: 0,
            instrs: Vec::new(),
        }
    }

    pub fn vars_len(mut self, vars_len: usize) -> ChunkBuilder {
        self.vars_len = vars_len;
        self
    }

    pub fn instr(mut self, instr: Instr) -> ChunkBuilder {
        self.instrs.push(instr);
        self
    }

    pub fn build<'gc>(self, arena: &Arena<'gc>) -> Chunk<'gc> {
        Chunk(Gc::new(
            arena,
            ChunkData {
                vars_len: self.vars_len,
                instrs: self.instrs,
            },
        ))
    }
}
