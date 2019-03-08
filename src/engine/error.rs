use crate::core::ident::Ident;

use super::machine::OperandKind;

pub type Result<'gc, T> = std::result::Result<T, Error<'gc>>;

#[derive(Debug, Error)]
pub enum Error<'gc> {
    #[error(display = "method not found: {}", ident)]
    MethodNotFound { ident: Ident<'gc> },

    #[error(
        display = "wrong arity: expected {}, received {}",
        expected,
        received
    )]
    WrongArity { expected: u8, received: u8 },

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

    #[error(display = "parent not found")]
    ParentNotFound,

    #[error(display = "variable not found: {}", var)]
    VarNotFound { var: usize },
}
