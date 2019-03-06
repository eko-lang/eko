use std::collections::BTreeMap;
use std::fmt;

use eko_gc::{Gc, Ref, RefCell};

use super::fun::Fn;
use super::ident::Ident;

#[derive(Debug, Trace)]
pub enum Type<'gc> {
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
    fns: BTreeMap<Ident<'gc>, Fn<'gc>>,
}

#[derive(Debug, Trace)]
pub struct Enum<'gc>(Gc<'gc, RefCell<'gc, EnumData<'gc>>>);

#[derive(Debug, Trace)]
pub struct EnumData<'gc> {
    ident: Ident<'gc>,
    variants: Vec<EnumVariant<'gc>>,
    fns: BTreeMap<Ident<'gc>, Fn<'gc>>,
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

#[derive(Debug)]
pub enum Kind {
    Tuple,
    Map,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Tuple => write!(f, "tuple"),
            Kind::Map => write!(f, "map"),
        }
    }
}
