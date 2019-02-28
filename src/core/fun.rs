use eko_gc::Gc;

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
