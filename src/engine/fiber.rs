use crate::core::Value;

use super::frame::Frame;

pub struct Fiber<'gc> {
    operand_stack: Vec<Value<'gc>>,
    frame_stack: Vec<Frame<'gc>>,
}