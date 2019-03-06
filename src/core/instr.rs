use super::fun::Fn;
use super::modu::Mod;
use super::value::Value;

#[derive(Debug, Clone)]
pub enum Instr<'gc> {
    PushValue { value: Value<'gc> },
    PushMod { modu: Mod<'gc> },
    PushFn { fun: Fn<'gc> },
    Pop,

    PushVar { var: usize },
    PopVar { var: usize },

    Add,
    Subtract,
    Multiply,
    Divide,
}
