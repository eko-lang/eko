use super::ident::Ident;
use super::typ::Kind;

pub type Result<'gc, T> = std::result::Result<T, Error<'gc>>;

#[derive(Debug, Error)]
pub enum Error<'gc> {
    #[error(display = "function not found: {}", ident)]
    FnNotFound { ident: Ident<'gc> },

    #[error(display = "missing field: {}", ident)]
    MissingField { ident: Ident<'gc> },

    #[error(display = "invalid field: {}", ident)]
    InvalidField { ident: Ident<'gc> },

    #[error(
        display = "invalid kind: expected {}, received {}",
        expected,
        received
    )]
    InvalidKind { expected: Kind, received: Kind },
}
