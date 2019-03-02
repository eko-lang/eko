use eko_gc::{Arena, Gc};

use super::ident::Ident;
use super::modu::Mod;

#[derive(Trace)]
pub struct Fn<'gc>(Gc<'gc, FnData<'gc>>);

impl<'gc> Fn<'gc> {
    pub fn arity(&self) -> u8 {
        self.0.arity
    }

    pub fn proto(&self) -> &FnProto<'gc> {
        &self.0.proto
    }
}

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
    pub fn new(arena: &Arena<'gc>, local_scope_len: usize, instrs: Vec<Instr>) -> Chunk<'gc> {
        Chunk(Gc::new(
            arena,
            ChunkData {
                local_scope_len,
                instrs,
            },
        ))
    }

    pub fn local_scope_len(&self) -> usize {
        self.0.local_scope_len
    }

    pub fn instr(&self, index: usize) -> Option<Instr> {
        self.0.instrs.get(index).cloned()
    }
}

#[derive(Trace)]
pub struct ChunkData {
    local_scope_len: usize,
    instrs: Vec<Instr>,
}

#[derive(Clone, Copy)]
pub enum Instr {
    PushConstant(Constant),
    Pop,
}

#[derive(Clone, Copy)]
pub enum Constant {
    Boolean(bool),
    Integer(i64),
    Float(f64),
}
