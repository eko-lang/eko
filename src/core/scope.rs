use eko_gc::{Arena, Gc, RefCell};

use super::value::Value;

#[derive(Trace)]
pub struct Scope<'gc>(Gc<'gc, RefCell<'gc, Vec<Value<'gc>>>>);

impl<'gc> Scope<'gc> {
    pub fn new(arena: &Arena<'gc>, vars_len: usize) -> Scope<'gc> {
        Scope(Gc::new(
            arena,
            // TODO: Figure out how to represent `None`.
            RefCell::new(arena, vec![Value::Boolean(false); vars_len]),
        ))
    }
}

#[derive(Clone, Trace)]
pub struct CapturedScope<'gc>(Gc<'gc, RefCell<'gc, CapturedScopeData<'gc>>>);

#[derive(Trace)]
pub struct CapturedScopeData<'gc> {
    parent_scope: Option<CapturedScope<'gc>>,
    vars_len: usize,
    scope: Scope<'gc>,
}
