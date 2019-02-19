use std::collections::BTreeMap;

use eko_gc::{Arena, Gc, RefCell};

use crate::ident::Ident;

#[derive(Clone, Trace)]
pub enum Value<'gc> {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String<'gc>),
    Tuple(Tuple<'gc>),
    Struct(Struct<'gc>),
    Enum(Enum<'gc>),
}

#[derive(Clone, Trace)]
pub struct String<'gc>(Gc<'gc, RefCell<'gc, std::string::String>>);

#[derive(Clone, Trace)]
pub struct Tuple<'gc>(Gc<'gc, RefCell<'gc, TupleData<'gc>>>);

impl<'gc> Tuple<'gc> {
    pub fn new(arena: &Arena<'gc>, fields: Vec<Value<'gc>>) -> Tuple<'gc> {
        Tuple(Gc::new(arena, RefCell::new(arena, TupleData { fields })))
    }

    pub fn set_field(&self, field: u8, value: Value<'gc>) -> bool {
        self.0.borrow_mut().set_field(field, value)
    }

    pub fn field(&self, field: u8) -> Option<Value<'gc>> {
        self.0.borrow().field(field)
    }
}

#[derive(Trace)]
pub struct TupleData<'gc> {
    fields: Vec<Value<'gc>>,
}

impl<'gc> TupleData<'gc> {
    pub fn field(&self, field: u8) -> Option<Value<'gc>> {
        self.fields.get(field as usize).cloned()
    }

    pub fn set_field(&mut self, field: u8, value: Value<'gc>) -> bool {
        if let Some(field) = self.fields.get_mut(field as usize) {
            *field = value;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Trace)]
pub struct Struct<'gc>(Gc<'gc, RefCell<'gc, StructData<'gc>>>);

impl<'gc> Struct<'gc> {
    pub fn new_tuple(arena: &Arena<'gc>, fields: Vec<Value<'gc>>) -> Struct<'gc> {
        Struct(Gc::new(
            arena,
            RefCell::new(
                arena,
                StructData {
                    proto: StructProto::Tuple(TupleData { fields }),
                },
            ),
        ))
    }

    pub fn new_map(arena: &Arena<'gc>, fields: BTreeMap<Ident<'gc>, Value<'gc>>) -> Struct<'gc> {
        Struct(Gc::new(
            arena,
            RefCell::new(
                arena,
                StructData {
                    proto: StructProto::Map(MapData { fields }),
                },
            ),
        ))
    }

    pub fn set_tuple_field(&self, field: u8, value: Value<'gc>) -> bool {
        self.0.borrow_mut().proto.set_tuple_field(field, value)
    }

    pub fn set_map_field(&self, field: Ident<'gc>, value: Value<'gc>) -> bool {
        self.0.borrow_mut().proto.set_map_field(field, value)
    }

    pub fn tuple_field(&self, field: u8) -> Option<Value<'gc>> {
        self.0.borrow().proto.tuple_field(field)
    }

    pub fn map_field(&self, field: &Ident<'gc>) -> Option<Value<'gc>> {
        self.0.borrow().proto.map_field(field)
    }
}

#[derive(Trace)]
pub struct StructData<'gc> {
    proto: StructProto<'gc>,
}

#[derive(Clone, Trace)]
pub struct Enum<'gc>(Gc<'gc, RefCell<'gc, EnumData<'gc>>>);

impl<'gc> Enum<'gc> {
    pub fn set_tuple_field(&self, field: u8, value: Value<'gc>) -> bool {
        self.0.borrow_mut().proto.set_tuple_field(field, value)
    }

    pub fn set_map_field(&self, field: Ident<'gc>, value: Value<'gc>) -> bool {
        self.0.borrow_mut().proto.set_map_field(field, value)
    }

    pub fn tuple_field(&self, field: u8) -> Option<Value<'gc>> {
        self.0.borrow().proto.tuple_field(field)
    }

    pub fn map_field(&self, field: &Ident<'gc>) -> Option<Value<'gc>> {
        self.0.borrow().proto.map_field(field)
    }
}

#[derive(Trace)]
pub struct EnumData<'gc> {
    variant: u8,
    proto: StructProto<'gc>,
}

#[derive(Trace)]
pub enum StructProto<'gc> {
    Tuple(TupleData<'gc>),
    Map(MapData<'gc>),
}

impl<'gc> StructProto<'gc> {
    pub fn set_tuple_field(&mut self, field: u8, value: Value<'gc>) -> bool {
        if let StructProto::Tuple(tuple) = self {
            tuple.set_field(field, value)
        } else {
            false
        }
    }

    pub fn set_map_field(&mut self, field: Ident<'gc>, value: Value<'gc>) -> bool {
        if let StructProto::Map(map) = self {
            map.set_field(field, value)
        } else {
            false
        }
    }

    pub fn tuple_field(&self, field: u8) -> Option<Value<'gc>> {
        if let StructProto::Tuple(tuple) = self {
            tuple.field(field)
        } else {
            None
        }
    }

    pub fn map_field(&self, field: &Ident<'gc>) -> Option<Value<'gc>> {
        if let StructProto::Map(map) = self {
            map.field(field)
        } else {
            None
        }
    }
}

#[derive(Trace)]
pub struct MapData<'gc> {
    fields: BTreeMap<Ident<'gc>, Value<'gc>>,
}

impl<'gc> MapData<'gc> {
    pub fn set_field(&mut self, field: Ident<'gc>, value: Value<'gc>) -> bool {
        if self.fields.get(&field).is_some() {
            self.fields.insert(field, value);
            true
        } else {
            false
        }
    }

    pub fn field(&self, field: &Ident<'gc>) -> Option<Value<'gc>> {
        self.fields.get(field).cloned()
    }
}
