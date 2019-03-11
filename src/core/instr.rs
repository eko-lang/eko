use super::value::Value;

#[derive(Debug, Clone)]
pub enum Instr<'gc> {
    Value(Value<'gc>),

    Load(usize),
    Store(usize),

    Pop,

    Add,
    Subtract,
    Multiply,
    Divide,

    Call(u8),
}
