use std::fmt;

use eko_gc::{Arena, Gc, Trace};

use super::ident::Ident;
use super::instr::Instr;
use super::modu::Mod;
use super::value::Value;

#[derive(Clone, Debug, Trace)]
pub struct Fn<'gc>(Gc<'gc, FnData<'gc>>);

impl<'gc> Fn<'gc> {
    // TODO: Add in all the required parameters.
    pub fn new_chunk(
        arena: &Arena<'gc>,
        arity: u8,
        chunk: Chunk<'gc>,
    ) -> Fn<'gc> {
        Fn(Gc::new(
            arena,
            FnData {
                modu: Mod::new(arena),
                ident: Ident::new_number(0),
                arity,
                is_method: false,
                proto: FnProto::Chunk(chunk),
            },
        ))
    }

    // TODO: Add in all the required parameters.
    pub fn new_external(
        arena: &Arena<'gc>,
        arity: u8,
        external: External<'gc>,
    ) -> Fn<'gc> {
        Fn(Gc::new(
            arena,
            FnData {
                modu: Mod::new(arena),
                ident: Ident::new_number(0),
                arity,
                is_method: false,
                proto: FnProto::External(external),
            },
        ))
    }

    pub fn ident(&self) -> &Ident<'gc> {
        &self.0.ident
    }

    pub fn arity(&self) -> u8 {
        self.0.arity
    }

    pub fn is_method(&self) -> bool {
        self.0.is_method
    }

    pub fn proto(&self) -> &FnProto<'gc> {
        &self.0.proto
    }
}

#[derive(Debug, Trace)]
pub struct FnData<'gc> {
    modu: Mod<'gc>,
    ident: Ident<'gc>,
    arity: u8,
    is_method: bool,
    proto: FnProto<'gc>,
}

#[derive(Trace)]
pub enum FnProto<'gc> {
    Chunk(Chunk<'gc>),
    External(External<'gc>),
}

impl<'gc> fmt::Debug for FnProto<'gc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FnProto::*;

        match self {
            Chunk(chunk) => fmt::Debug::fmt(chunk, f),
            // TODO: Move this onto `External`.
            External(_) => write!(f, "External"),
        }
    }
}

#[derive(Clone, Debug, Trace)]
pub struct Chunk<'gc>(Gc<'gc, ChunkData<'gc>>);

impl<'gc> Chunk<'gc> {
    pub fn new(
        arena: &Arena<'gc>,
        local_scope_len: usize,
        instrs: Vec<Instr<'gc>>,
    ) -> Chunk<'gc> {
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

    pub fn instr(&self, index: usize) -> Option<Instr<'gc>> {
        self.0.instrs.get(index).cloned()
    }
}

#[derive(Debug, Trace)]
pub struct ChunkData<'gc> {
    local_scope_len: usize,
    instrs: Vec<Instr<'gc>>,
}

#[derive(Clone, Trace)]
pub struct External<'gc>(Gc<'gc, ExternalFn<'gc>>);

impl<'gc> External<'gc> {
    pub fn new(
        arena: &Arena<'gc>,
        external: fn(Vec<Value<'gc>>) -> Value<'gc>,
    ) -> External<'gc> {
        External(Gc::new(arena, ExternalFn(external)))
    }

    pub fn call(&self, args: Vec<Value<'gc>>) -> Value<'gc> {
        ((self.0).0)(args)
    }
}

pub struct ExternalFn<'gc>(fn(Vec<Value<'gc>>) -> Value<'gc>);

unsafe impl<'gc> Trace for ExternalFn<'gc> {}
