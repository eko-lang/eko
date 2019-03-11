use std::collections::BTreeMap;

use eko_gc::{Arena, Gc, RefCell};

use super::error::{Error, Result};
use super::fun::Fun;
use super::ident::Ident;
use super::typ::Typ;

#[derive(Clone, Debug, Trace)]
pub struct Modu<'gc>(Gc<'gc, RefCell<'gc, ModuData<'gc>>>);

impl<'gc> Modu<'gc> {
    // TODO: Add all the required parameters.
    pub fn new(arena: &Arena<'gc>) -> Modu<'gc> {
        Modu(Gc::new(
            arena,
            RefCell::new(arena, ModuData::new(Ident::new_number(0))),
        ))
    }

    // TODO: Add all the required parameters.
    pub fn with_parent_modu(
        arena: &Arena<'gc>,
        parent_modu: Modu<'gc>,
    ) -> Modu<'gc> {
        Modu(Gc::new(
            arena,
            RefCell::new(
                arena,
                ModuData::with_parent_modu(Ident::new_number(0), parent_modu),
            ),
        ))
    }

    pub fn define_fun(&self, ident: Ident<'gc>, fun: Fun<'gc>) {
        self.0.borrow_mut().define_fun(ident, fun);
    }

    pub fn fun(&self, ident: Ident<'gc>) -> Result<'gc, Fun<'gc>> {
        self.0.borrow().fun(ident)
    }
}

#[derive(Debug, Trace)]
pub struct ModuData<'gc> {
    ident: Ident<'gc>,
    parent_modu: Option<Modu<'gc>>,
    child_modus: BTreeMap<Ident<'gc>, Modu<'gc>>,
    typs: BTreeMap<Ident<'gc>, Typ<'gc>>,
    funs: BTreeMap<Ident<'gc>, Fun<'gc>>,
}

impl<'gc> ModuData<'gc> {
    fn new(ident: Ident<'gc>) -> ModuData<'gc> {
        ModuData {
            ident,
            parent_modu: None,
            child_modus: BTreeMap::new(),
            typs: BTreeMap::new(),
            funs: BTreeMap::new(),
        }
    }

    fn with_parent_modu(
        ident: Ident<'gc>,
        parent_modu: Modu<'gc>,
    ) -> ModuData<'gc> {
        ModuData {
            ident,
            parent_modu: Some(parent_modu),
            child_modus: BTreeMap::new(),
            typs: BTreeMap::new(),
            funs: BTreeMap::new(),
        }
    }

    fn define_fun(&mut self, ident: Ident<'gc>, fun: Fun<'gc>) {
        self.funs.insert(ident, fun);
    }

    fn fun(&self, ident: Ident<'gc>) -> Result<'gc, Fun<'gc>> {
        self.funs
            .get(&ident)
            .cloned()
            .ok_or_else(|| Error::FunNotFound { ident })
    }
}
