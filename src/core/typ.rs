use std::collections::BTreeMap;

use eko_gc::{Gc, Ref, RefCell};

use super::error::{Error, Result};
use super::fun::Fun;
use super::ident::Ident;

#[derive(Debug, Trace)]
pub enum Typ<'gc> {
    Struct(Struct<'gc>),
    Enum(Enum<'gc>),
}

#[derive(Clone, Debug, Trace)]
pub struct Struct<'gc>(Gc<'gc, RefCell<'gc, StructData<'gc>>>);

impl<'gc> Struct<'gc> {
    pub fn proto(&self) -> Ref<StructProto<'gc>> {
        Ref::map(self.0.borrow(), |data| &data.proto)
    }
}

#[derive(Debug, Trace)]
pub struct StructData<'gc> {
    ident: Ident<'gc>,
    proto: StructProto<'gc>,
    funs: BTreeMap<Ident<'gc>, Fun<'gc>>,
}

impl<'gc> StructData<'gc> {
    pub fn define_fun(&mut self, ident: Ident<'gc>, fun: Fun<'gc>) {
        self.funs.insert(ident, fun);
    }

    pub fn fun(&self, ident: Ident<'gc>) -> Result<'gc, Fun<'gc>> {
        self.funs
            .get(&ident)
            .cloned()
            .ok_or_else(|| Error::FunNotFound { ident })
    }
}

#[derive(Debug, Trace)]
pub struct Enum<'gc>(Gc<'gc, RefCell<'gc, EnumData<'gc>>>);

#[derive(Debug, Trace)]
pub struct EnumData<'gc> {
    ident: Ident<'gc>,
    variants: Vec<EnumVariant<'gc>>,
    funs: BTreeMap<Ident<'gc>, Fun<'gc>>,
}

impl<'gc> EnumData<'gc> {
    pub fn define_fun(&mut self, ident: Ident<'gc>, fun: Fun<'gc>) {
        self.funs.insert(ident, fun);
    }

    pub fn fun(&self, ident: Ident<'gc>) -> Result<'gc, Fun<'gc>> {
        self.funs
            .get(&ident)
            .cloned()
            .ok_or_else(|| Error::FunNotFound { ident })
    }
}

#[derive(Debug, Trace)]
pub struct EnumVariant<'gc> {
    ident: Ident<'gc>,
    proto: StructProto<'gc>,
}

#[derive(Debug, Trace)]
pub enum StructProto<'gc> {
    Tuple(u8),
    Map(MapData<'gc>),
}

#[derive(Debug)]
pub struct MapData<'gc> {
    fields: BTreeMap<Ident<'gc>, ()>,
}

impl<'gc> MapData<'gc> {
    pub fn fields(&self) -> &BTreeMap<Ident<'gc>, ()> {
        &self.fields
    }
}
