use eko_gc::Arena;

use crate::core::fun::Constant;
use crate::core::value::Value;

use super::error::{Error, Result};

pub struct Machine<'gc> {
    operand_stack: Vec<Value<'gc>>,
}

impl<'gc> Machine<'gc> {
    pub fn new() -> Machine<'gc> {
        Machine {
            operand_stack: Vec::new(),
        }
    }

    pub fn push_constant(&mut self, arena: &Arena<'gc>, constant: Constant) {
        self.operand_stack
            .push(Value::from_constant(arena, constant));
    }

    pub fn pop(&mut self) -> Result<'gc, Value<'gc>> {
        self.operand_stack
            .pop()
            .ok_or_else(|| Error::EmptyOperandStack)
    }
}
