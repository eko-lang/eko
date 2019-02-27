use eko_gc::{Gc, RefCell};

use super::value::Value;

#[derive(Trace)]
pub struct Scope<'gc>(Gc<'gc, RefCell<'gc, Vec<Value<'gc>>>>);

#[derive(Trace)]
pub struct CapturedScope<'gc> {
    parent: Option<Box<CapturedScope<'gc>>>,
    vars_len: usize,
    scope: Scope<'gc>,
}
