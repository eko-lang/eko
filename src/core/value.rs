use std::collections::BTreeMap;

use eko_gc::{Arena, Gc, RefCell};

use crate::engine::frame::CapturedScope;

use super::error::{Error, Result};
use super::fun::Fn;
use super::ident::Ident;
use super::typ::{self, Kind};

#[derive(Clone, Debug, PartialEq, Trace)]
pub enum Value<'gc> {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String<'gc>),
    Tuple(Tuple<'gc>),
    Struct(Struct<'gc>),
    Enum(Enum<'gc>),
    Option(Option<'gc>),
    Closure(Closure<'gc>),
}

impl<'gc> Value<'gc> {
    pub fn empty(arena: &Arena<'gc>) -> Value<'gc> {
        Value::Option(Option::new_none(arena))
    }
}

#[derive(Clone, Debug, Trace)]
pub struct String<'gc>(Gc<'gc, RefCell<'gc, std::string::String>>);

impl<'gc> PartialEq for String<'gc> {
    fn eq(&self, other: &String<'gc>) -> bool {
        Gc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Clone, Debug, Trace)]
pub struct Tuple<'gc>(Gc<'gc, RefCell<'gc, TupleData<'gc>>>);

impl<'gc> Tuple<'gc> {
    pub fn new(arena: &Arena<'gc>, fields: Vec<Value<'gc>>) -> Tuple<'gc> {
        Tuple(Gc::new(&arena, RefCell::new(&arena, TupleData { fields })))
    }

    pub fn set_field(&self, field: u8, value: Value<'gc>) -> Result<'gc, ()> {
        self.0.borrow_mut().set_field(field, value)
    }

    pub fn field(&self, field: u8) -> Result<'gc, Value<'gc>> {
        self.0.borrow().field(field)
    }
}

impl<'gc> PartialEq for Tuple<'gc> {
    fn eq(&self, other: &Tuple<'gc>) -> bool {
        Gc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Debug, Trace)]
pub struct TupleData<'gc> {
    fields: Vec<Value<'gc>>,
}

impl<'gc> TupleData<'gc> {
    fn set_field(&mut self, field: u8, value: Value<'gc>) -> Result<'gc, ()> {
        if let Some(field) = self.fields.get_mut(field as usize) {
            *field = value;
            Ok(())
        } else {
            Err(Error::InvalidField {
                field: Ident::new_number(field),
            })
        }
    }

    fn field(&self, field: u8) -> Result<'gc, Value<'gc>> {
        self.fields.get(field as usize).cloned().ok_or_else(|| {
            Error::InvalidField {
                field: Ident::new_number(field),
            }
        })
    }
}

#[derive(Clone, Debug, Trace)]
pub struct Struct<'gc>(Gc<'gc, RefCell<'gc, StructData<'gc>>>);

impl<'gc> Struct<'gc> {
    pub fn new_tuple(
        arena: &Arena<'gc>,
        typ: typ::Struct<'gc>,
        fields: Vec<Value<'gc>>,
    ) -> Result<'gc, Struct<'gc>> {
        match *typ.proto() {
            typ::StructProto::Tuple(num_fields) => {
                let data = StructData {
                    typ: typ.clone(),
                    proto: StructProto::new_tuple(num_fields, fields)?,
                };
                Ok(Struct(Gc::new(&arena, RefCell::new(&arena, data))))
            }
            typ::StructProto::Map(_) => Err(Error::InvalidKind {
                expected: Kind::Tuple,
                received: Kind::Map,
            }),
        }
    }

    pub fn new_map(
        arena: &Arena<'gc>,
        typ: typ::Struct<'gc>,
        fields: BTreeMap<Ident<'gc>, Value<'gc>>,
    ) -> Result<'gc, Struct<'gc>> {
        match *typ.proto() {
            typ::StructProto::Tuple(_) => Err(Error::InvalidKind {
                expected: Kind::Map,
                received: Kind::Tuple,
            }),
            typ::StructProto::Map(ref map_data) => {
                let data = StructData {
                    typ: typ.clone(),
                    proto: StructProto::new_map(&map_data, fields)?,
                };
                Ok(Struct(Gc::new(&arena, RefCell::new(&arena, data))))
            }
        }
    }

    pub fn set_tuple_field(
        &self,
        field: u8,
        value: Value<'gc>,
    ) -> Result<'gc, ()> {
        self.0.borrow_mut().proto.set_tuple_field(field, value)
    }

    pub fn set_map_field(
        &self,
        field: Ident<'gc>,
        value: Value<'gc>,
    ) -> Result<'gc, ()> {
        self.0.borrow_mut().proto.set_map_field(field, value)
    }

    pub fn tuple_field(&self, field: u8) -> Result<'gc, Value<'gc>> {
        self.0.borrow().proto.tuple_field(field)
    }

    pub fn map_field(&self, field: Ident<'gc>) -> Result<'gc, Value<'gc>> {
        self.0.borrow().proto.map_field(field)
    }
}

impl<'gc> PartialEq for Struct<'gc> {
    fn eq(&self, other: &Struct<'gc>) -> bool {
        Gc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Debug, Trace)]
pub struct StructData<'gc> {
    typ: typ::Struct<'gc>,
    proto: StructProto<'gc>,
}

#[derive(Clone, Debug, Trace)]
pub struct Enum<'gc>(Gc<'gc, RefCell<'gc, EnumData<'gc>>>);

impl<'gc> Enum<'gc> {
    pub fn set_tuple_field(
        &self,
        field: u8,
        value: Value<'gc>,
    ) -> Result<'gc, ()> {
        self.0.borrow_mut().proto.set_tuple_field(field, value)
    }

    pub fn set_map_field(
        &self,
        field: Ident<'gc>,
        value: Value<'gc>,
    ) -> Result<'gc, ()> {
        self.0.borrow_mut().proto.set_map_field(field, value)
    }

    pub fn tuple_field(&self, field: u8) -> Result<'gc, Value<'gc>> {
        self.0.borrow().proto.tuple_field(field)
    }

    pub fn map_field(&self, field: Ident<'gc>) -> Result<'gc, Value<'gc>> {
        self.0.borrow().proto.map_field(field)
    }
}

impl<'gc> PartialEq for Enum<'gc> {
    fn eq(&self, other: &Enum<'gc>) -> bool {
        Gc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Debug, Trace)]
pub struct EnumData<'gc> {
    typ: typ::Enum<'gc>,
    variant: u8,
    proto: StructProto<'gc>,
}

#[derive(Debug, Trace)]
pub enum StructProto<'gc> {
    Tuple(TupleData<'gc>),
    Map(MapData<'gc>),
}

impl<'gc> StructProto<'gc> {
    fn new_tuple(
        num_fields: u8,
        fields: Vec<Value<'gc>>,
    ) -> Result<'gc, StructProto<'gc>> {
        if num_fields > fields.len() as u8 {
            Err(Error::MissingField {
                field: Ident::new_number(fields.len() as u8),
            })
        } else if num_fields < fields.len() as u8 {
            Err(Error::InvalidField {
                field: Ident::new_number(num_fields),
            })
        } else {
            Ok(StructProto::Tuple(TupleData { fields }))
        }
    }

    fn new_map(
        map_data: &typ::MapData<'gc>,
        fields: BTreeMap<Ident<'gc>, Value<'gc>>,
    ) -> Result<'gc, StructProto<'gc>> {
        let map_data_fields = map_data.fields();

        for (ident, _) in map_data_fields.iter() {
            if fields.get(ident).is_none() {
                return Err(Error::MissingField {
                    field: ident.clone(),
                });
            }
        }

        for (ident, _) in fields.iter() {
            if map_data_fields.get(ident).is_none() {
                return Err(Error::InvalidField {
                    field: ident.clone(),
                });
            }
        }

        Ok(StructProto::Map(MapData { fields }))
    }

    fn set_tuple_field(
        &mut self,
        field: u8,
        value: Value<'gc>,
    ) -> Result<'gc, ()> {
        match self {
            StructProto::Tuple(tuple_data) => {
                tuple_data.set_field(field, value)
            }
            StructProto::Map(_) => Err(Error::InvalidKind {
                expected: Kind::Map,
                received: Kind::Tuple,
            }),
        }
    }

    fn set_map_field(
        &mut self,
        field: Ident<'gc>,
        value: Value<'gc>,
    ) -> Result<'gc, ()> {
        match self {
            StructProto::Tuple(_) => Err(Error::InvalidKind {
                expected: Kind::Tuple,
                received: Kind::Map,
            }),
            StructProto::Map(map_data) => map_data.set_field(field, value),
        }
    }

    fn tuple_field(&self, field: u8) -> Result<'gc, Value<'gc>> {
        match self {
            StructProto::Tuple(tuple_data) => tuple_data.field(field),
            StructProto::Map(_) => Err(Error::InvalidKind {
                expected: Kind::Map,
                received: Kind::Tuple,
            }),
        }
    }

    fn map_field(&self, field: Ident<'gc>) -> Result<'gc, Value<'gc>> {
        match self {
            StructProto::Tuple(_) => Err(Error::InvalidKind {
                expected: Kind::Tuple,
                received: Kind::Map,
            }),
            StructProto::Map(map_data) => map_data.field(field),
        }
    }
}

#[derive(Debug, Trace)]
pub struct MapData<'gc> {
    fields: BTreeMap<Ident<'gc>, Value<'gc>>,
}

impl<'gc> MapData<'gc> {
    pub fn set_field(
        &mut self,
        field: Ident<'gc>,
        value: Value<'gc>,
    ) -> Result<'gc, ()> {
        if self.fields.get(&field).is_some() {
            self.fields.insert(field, value);
            Ok(())
        } else {
            Err(Error::InvalidField { field })
        }
    }

    pub fn field(&self, field: Ident<'gc>) -> Result<'gc, Value<'gc>> {
        self.fields
            .get(&field)
            .cloned()
            .ok_or_else(|| Error::InvalidField { field })
    }
}

#[derive(Clone, Debug, Trace)]
pub struct Option<'gc>(Gc<'gc, RefCell<'gc, OptionData<'gc>>>);

impl<'gc> Option<'gc> {
    pub fn new_none(arena: &Arena<'gc>) -> Option<'gc> {
        Option(Gc::new(&arena, RefCell::new(&arena, OptionData::None)))
    }
}

// TODO: Determine if this is intended behaviour.
impl<'gc> PartialEq for Option<'gc> {
    fn eq(&self, other: &Option<'gc>) -> bool {
        Gc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Debug, Trace)]
pub enum OptionData<'gc> {
    Some(Value<'gc>),
    None,
}

#[derive(Clone, Debug, Trace)]
pub struct Closure<'gc>(Gc<'gc, ClosureData<'gc>>);

impl<'gc> Closure<'gc> {
    pub fn captured_scope(&self) -> CapturedScope<'gc> {
        self.0.captured_scope.clone()
    }
}

impl<'gc> PartialEq for Closure<'gc> {
    fn eq(&self, other: &Closure<'gc>) -> bool {
        Gc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Debug, Trace)]
pub struct ClosureData<'gc> {
    captured_scope: CapturedScope<'gc>,
    data: Fn<'gc>,
}
