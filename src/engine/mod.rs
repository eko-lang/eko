use eko_gc::Arena;

use crate::core::{ChunkBuilder, Mod, Value};

use self::frame::Frame;

pub mod error;
pub mod frame;

pub struct Engine<'gc> {
    arena: &'gc Arena<'gc>,
    root_mod: Mod<'gc>,
    operand_stack: Vec<Value<'gc>>,
    frame_stack: Vec<Frame<'gc>>,
}

impl<'gc> Engine<'gc> {
    // TODO: Pass in a parameter to indicate how to load `root_mod`.
    pub fn new(arena: &'gc Arena<'gc>) -> Engine<'gc> {
        let root_mod = Mod::new(&arena);

        // TODO: This should be loaded from the `main` method.
        let chunk = ChunkBuilder::new().build(arena);

        let frame = Frame::new(arena, chunk);

        Engine {
            arena,
            root_mod,
            operand_stack: Vec::new(),
            frame_stack: vec![frame],
        }
    }
}
