use std::collections::BTreeMap;

use eko_gc::{Arena, Gc, RefCell};

use super::ident::Ident;
use super::typ::Type;

#[derive(Clone, Debug, Trace)]
pub struct Mod<'gc>(Gc<'gc, RefCell<'gc, ModData<'gc>>>);

impl<'gc> Mod<'gc> {
    // TODO: Add all the required parameters.
    pub fn new(arena: &Arena<'gc>) -> Mod<'gc> {
        Mod(Gc::new(
            arena,
            RefCell::new(arena, ModData::new(Ident::new_number(0))),
        ))
    }

    // TODO: Add all the required parameters.
    pub fn with_parent_mod(
        arena: &Arena<'gc>,
        parent_mod: Mod<'gc>,
    ) -> Mod<'gc> {
        Mod(Gc::new(
            arena,
            RefCell::new(
                arena,
                ModData::with_parent_mod(Ident::new_number(0), parent_mod),
            ),
        ))
    }
}

#[derive(Debug, Trace)]
pub struct ModData<'gc> {
    ident: Ident<'gc>,
    parent_mod: Option<Mod<'gc>>,
    child_mods: BTreeMap<Ident<'gc>, Mod<'gc>>,
    types: BTreeMap<Ident<'gc>, Type<'gc>>,
}

impl<'gc> ModData<'gc> {
    fn new(ident: Ident<'gc>) -> ModData<'gc> {
        ModData {
            ident,
            parent_mod: None,
            child_mods: BTreeMap::new(),
            types: BTreeMap::new(),
        }
    }

    fn with_parent_mod(
        ident: Ident<'gc>,
        parent_mod: Mod<'gc>,
    ) -> ModData<'gc> {
        ModData {
            ident,
            parent_mod: Some(parent_mod),
            child_mods: BTreeMap::new(),
            types: BTreeMap::new(),
        }
    }
}
