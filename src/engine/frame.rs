use crate::core::Scope;

pub struct Frame<'gc> {
    scope: Scope<'gc>,
}