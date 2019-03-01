use eko_gc::Arena;

pub mod error;
pub mod frame;
pub mod machine;

pub struct Engine<'a, 'gc> {
    arena: &'a Arena<'gc>,
}

impl<'a, 'gc> Engine<'a, 'gc> {
    pub fn new(arena: &'a Arena<'gc>) -> Engine<'a, 'gc> {
        Engine { arena }
    }
}
