use super::fun::Fun;
use super::modu::Modu;
use super::value::Value;

#[derive(Debug, Clone)]
pub enum Instr<'gc> {
    PushValue { value: Value<'gc> },
    PushModu { modu: Modu<'gc> },
    PushFun { fun: Fun<'gc> },

    LoadVar { var: usize },
    StoreVar { var: usize },

    Pop,

    Add,
    Subtract,
    Multiply,
    Divide,

    Call { arity: u8, is_method: bool },
}
