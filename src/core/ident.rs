use std::fmt;

use eko_gc::{Arena, Gc};

#[derive(Clone, Debug, Eq, Trace, Ord, PartialEq, PartialOrd)]
pub struct Ident<'gc>(Gc<'gc, String>);

impl<'gc> Ident<'gc> {
    pub fn new(arena: &Arena<'gc>, data: String) -> Ident<'gc> {
        Ident(Gc::new(arena, data))
    }
}

impl<'gc> fmt::Display for Ident<'gc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &*self)
    }
}
