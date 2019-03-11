use std::fmt;

use eko_gc::{Arena, Gc, Trace};

use super::ident::Ident;
use super::instr::Instr;
use super::modu::Modu;
use super::value::Value;

#[derive(Clone, Debug, Trace)]
pub struct Fun<'gc>(Gc<'gc, FunData<'gc>>);

impl<'gc> Fun<'gc> {
    // TODO: Add in all the required parameters.
    pub fn new_chunk(
        arena: &Arena<'gc>,
        arity: u8,
        chunk: Chunk<'gc>,
    ) -> Fun<'gc> {
        Fun(Gc::new(
            arena,
            FunData {
                modu: Modu::new(arena),
                ident: Ident::new_number(0),
                arity,
                is_method: false,
                proto: FunProto::Chunk(chunk),
            },
        ))
    }

    // TODO: Add in all the required parameters.
    pub fn new_external(
        arena: &Arena<'gc>,
        arity: u8,
        external: External<'gc>,
    ) -> Fun<'gc> {
        Fun(Gc::new(
            arena,
            FunData {
                modu: Modu::new(arena),
                ident: Ident::new_number(0),
                arity,
                is_method: false,
                proto: FunProto::External(external),
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

    pub fn proto(&self) -> &FunProto<'gc> {
        &self.0.proto
    }
}

#[derive(Debug, Trace)]
pub struct FunData<'gc> {
    modu: Modu<'gc>,
    ident: Ident<'gc>,
    arity: u8,
    is_method: bool,
    proto: FunProto<'gc>,
}

#[derive(Trace)]
pub enum FunProto<'gc> {
    Chunk(Chunk<'gc>),
    External(External<'gc>),
}

impl<'gc> fmt::Debug for FunProto<'gc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::FunProto::*;

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
pub struct External<'gc>(Gc<'gc, ExternalFun<'gc>>);

impl<'gc> External<'gc> {
    pub fn new(
        arena: &Arena<'gc>,
        external: fn(Vec<Value<'gc>>) -> Value<'gc>,
    ) -> External<'gc> {
        External(Gc::new(arena, ExternalFun(external)))
    }

    pub fn call(&self, args: Vec<Value<'gc>>) -> Value<'gc> {
        ((self.0).0)(args)
    }
}

pub struct ExternalFun<'gc>(fn(Vec<Value<'gc>>) -> Value<'gc>);

unsafe impl<'gc> Trace for ExternalFun<'gc> {}
