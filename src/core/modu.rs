use std::collections::BTreeMap;

use eko_gc::{Gc, RefCell};

use super::ident::Ident;
use super::typ::{Enum, Struct};

#[derive(Trace)]
pub struct Mod<'gc>(Gc<'gc, RefCell<'gc, ModData<'gc>>>);

#[derive(Trace)]
pub struct ModData<'gc> {
    parent_module: Mod<'gc>,
    child_modules: BTreeMap<Ident<'gc>, Mod<'gc>>,
    structs: BTreeMap<Ident<'gc>, Struct<'gc>>,
    enums: BTreeMap<Ident<'gc>, Enum<'gc>>,
}
