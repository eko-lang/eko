use std::fmt;

use super::ident::Ident;

pub type Result<'gc, T> = std::result::Result<T, Error<'gc>>;

#[derive(Debug, Error)]
pub enum Error<'gc> {
    #[error(display = "function not found: {}", ident)]
    FunNotFound { ident: Ident<'gc> },

    #[error(display = "missing field: {}", ident)]
    MissingField { ident: Ident<'gc> },

    #[error(display = "invalid field: {}", ident)]
    InvalidField { ident: Ident<'gc> },

    #[error(
        display = "invalid type kind: expected {}, received {}",
        expected,
        received
    )]
    InvalidTypeKind {
        expected: TypeKind,
        received: TypeKind,
    },
}

#[derive(Debug)]
pub enum TypeKind {
    Tuple,
    Map,
}

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TypeKind::*;

        match self {
            Tuple => write!(f, "tuple"),
            Map => write!(f, "map"),
        }
    }
}
