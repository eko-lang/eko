use std::collections::BTreeMap;

use eko_gc::{Gc, RefCell};

use crate::ident::Ident;

#[derive(Trace)]
pub struct Mod<'gc>(Gc<'gc, RefCell<'gc, ModData<'gc>>>);

#[derive(Trace)]
pub struct ModData<'gc> {
    parent_module: Mod<'gc>,
    child_modules: BTreeMap<Ident<'gc>, Mod<'gc>>,
    structs: BTreeMap<Ident<'gc>, Struct<'gc>>,
}

#[derive(Trace)]
pub struct Struct<'gc>(Gc<'gc, RefCell<'gc, StructData<'gc>>>);

#[derive(Trace)]
pub struct StructData<'gc> {
    ident: Ident<'gc>,
    proto: StructProto<'gc>,
    fns: BTreeMap<Ident<'gc>, Fn<'gc>>,
}

#[derive(Trace)]
pub struct Enum<'gc>(Gc<'gc, RefCell<'gc, EnumData<'gc>>>);

#[derive(Trace)]
pub struct EnumData<'gc> {
    ident: Ident<'gc>,
    variants: Vec<EnumVariant<'gc>>,
    fns: BTreeMap<Ident<'gc>, Fn<'gc>>,
}

#[derive(Trace)]
pub struct EnumVariant<'gc> {
    ident: Ident<'gc>,
    proto: StructProto<'gc>,
}

#[derive(Trace)]
pub enum StructProto<'gc> {
    Tuple(u8),
    Map(MapData<'gc>),
}

pub struct MapData<'gc> {
    fields: BTreeMap<Ident<'gc>, ()>,
}

#[derive(Trace)]
pub struct Fn<'gc>(Gc<'gc, RefCell<'gc, FnData<'gc>>>);

#[derive(Trace)]
pub struct FnData<'gc> {
    ident: Ident<'gc>,
    arity: u8,
    method: bool,
    kind: FnProto,
}

#[derive(Trace)]
pub enum FnProto {
    Internal(Box<Chunk>),
    External(Box<std::ops::FnOnce()>),
}

#[derive(Trace)]
pub struct Chunk {
    instrs: Vec<Instr>,
}

pub enum Instr {}
