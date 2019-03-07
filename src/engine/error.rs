use std::marker::PhantomData;

use super::machine::OperandKind;

pub type Result<'gc, T> = std::result::Result<T, Error<'gc>>;

#[derive(Debug, Error)]
pub enum Error<'gc> {
    #[error(display = "empty operand stack")]
    EmptyOperandStack,

    #[error(
        display = "invalid operand kind: expected {}, received {}",
        expected,
        received
    )]
    InvalidOperandKind {
        expected: OperandKind,
        received: OperandKind,
    },

    #[error(display = "invalid parent")]
    InvalidParent,

    #[error(display = "invalid variable: {}", var)]
    InvalidVar { var: usize },

    // TODO: Display name of method.
    #[error(display = "method not found")]
    MethodNotFound,

    #[error(
        display = "wrong arity: expected {}, received {}",
        expected,
        received
    )]
    WrongArity { expected: u8, received: u8 },

    #[error(display = "{:?}", _0)]
    PhantomData(PhantomData<&'gc ()>),
}
