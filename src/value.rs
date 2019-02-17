use std::collections::BTreeMap;

use eko_gc::{Gc, RefCell};

use crate::ident::Ident;

#[derive(Trace)]
pub enum Value<'gc> {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String<'gc>),
    Tuple(Tuple<'gc>),
    Struct(Struct<'gc>),
    Enum(Enum<'gc>),
}

#[derive(Trace)]
pub struct String<'gc>(Gc<'gc, RefCell<'gc, std::string::String>>);

#[derive(Trace)]
pub struct Tuple<'gc>(Gc<'gc, RefCell<'gc, TupleData<'gc>>>);

#[derive(Trace)]
pub struct TupleData<'gc> {
    fields: Vec<Value<'gc>>,
}

#[derive(Trace)]
pub struct Struct<'gc>(Gc<'gc, RefCell<'gc, StructData<'gc>>>);

#[derive(Trace)]
pub struct StructData<'gc> {
    data: FieldData<'gc>,
}

#[derive(Trace)]
pub struct Enum<'gc>(Gc<'gc, RefCell<'gc, EnumData<'gc>>>);

#[derive(Trace)]
pub struct EnumData<'gc> {
    variant: u8,
    data: FieldData<'gc>,
}

#[derive(Trace)]
pub enum FieldData<'gc> {
    Tuple(TupleData<'gc>),
    Map(MapData<'gc>),
}

#[derive(Trace)]
pub struct MapData<'gc> {
    fields: BTreeMap<Ident<'gc>, Value<'gc>>,
}
