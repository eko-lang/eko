use std::fmt;

use eko_gc::{Arena, Gc};

#[derive(Clone, Debug, Eq, Trace, Ord, PartialEq, PartialOrd)]
pub enum Ident<'gc> {
    String(Gc<'gc, String>),
    Number(u8),
}

impl<'gc> Ident<'gc> {
    pub fn new_string(arena: &Arena<'gc>, string: String) -> Ident<'gc> {
        Ident::String(Gc::new(arena, string))
    }

    pub fn new_number(number: u8) -> Ident<'gc> {
        Ident::Number(number)
    }
}

impl<'gc> fmt::Display for Ident<'gc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ident::String(string) => write!(f, "{}", &**string),
            Ident::Number(number) => write!(f, "{}", number),
        }
    }
}
