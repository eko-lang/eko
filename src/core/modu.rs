use std::collections::BTreeMap;

use eko_gc::{Gc, RefCell};

use super::ident::Ident;
use super::typ::Type;

#[derive(Trace)]
pub struct Mod<'gc>(Gc<'gc, RefCell<'gc, ModData<'gc>>>);

#[derive(Trace)]
pub struct ModData<'gc> {
    parent_module: Mod<'gc>,
    child_modules: BTreeMap<Ident<'gc>, Mod<'gc>>,
    types: BTreeMap<Ident<'gc>, Type<'gc>>,
}
