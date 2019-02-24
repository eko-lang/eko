use eko_gc::{Gc, RefCell};

use super::ident::Ident;

#[derive(Trace)]
pub struct Fn<'gc>(Gc<'gc, RefCell<'gc, FnData<'gc>>>);

#[derive(Trace)]
pub struct FnData<'gc> {
    ident: Ident<'gc>,
    arity: u8,
    method: bool,
    proto: FnProto,
}

#[derive(Trace)]
pub enum FnProto {
    Chunk(Box<Chunk>),
    External(Box<std::ops::FnOnce()>),
}

#[derive(Trace)]
pub struct Chunk {
    instrs: Vec<Instr>,
}

pub enum Instr {}
