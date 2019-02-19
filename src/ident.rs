use eko_gc::{Arena, Gc};

#[derive(Eq, Trace, Ord, PartialEq, PartialOrd)]
pub struct Ident<'gc>(Gc<'gc, String>);

impl<'gc> Ident<'gc> {
    pub fn new(arena: &Arena<'gc>, data: String) -> Ident<'gc> {
        Ident(Gc::new(arena, data))
    }
}
