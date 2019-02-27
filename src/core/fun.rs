use eko_gc::{Gc, RefCell};

use super::ident::Ident;
use super::modu::Mod;
use super::value::Value;

#[derive(Trace)]
pub struct Fn<'gc>(Gc<'gc, FnData<'gc>>);

#[derive(Trace)]
pub struct FnData<'gc> {
    modu: Mod<'gc>,
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
    vars_len: usize,
    instrs: Vec<Instr>,
}

pub enum Instr {}
