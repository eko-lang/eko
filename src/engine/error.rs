use std::fmt;

use crate::core::ident::Ident;

use super::machine::Operand;

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

#[derive(Debug)]
pub enum OperandKind {
    Modu,
    Fun,
    Method,
    Value,
}

impl<'gc> From<Operand<'gc>> for OperandKind {
    fn from(operand: Operand<'gc>) -> OperandKind {
        use self::OperandKind::*;

        match operand {
            Operand::Modu(_) => Modu,
            Operand::Fun(_) => Fun,
            Operand::Method(_) => Method,
            Operand::Value(_) => Value,
        }
    }
}

// TODO: Check the display strings.
impl fmt::Display for OperandKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::OperandKind::*;

        match self {
            Modu => write!(f, "module"),
            Fun => write!(f, "function"),
            Method => write!(f, "method"),
            Value => write!(f, "value"),
        }
    }
}
