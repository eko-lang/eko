use super::ident::Ident;
use super::typ::Kind;

pub type Result<'gc, T> = std::result::Result<T, Error<'gc>>;

#[derive(Debug, Error)]
pub enum Error<'gc> {
    #[error(display = "missing field: {}", _0)]
    MissingField { field: Ident<'gc> },

    #[error(display = "invalid field: {}", _0)]
    InvalidField { field: Ident<'gc> },

    #[error(display = "invalid kind: expected {}, received {}", expected, received)]
    InvalidKind { expected: Kind, received: Kind },
}
