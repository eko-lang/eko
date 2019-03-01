use std::marker::PhantomData;

pub type Result<'gc, T> = std::result::Result<T, Error<'gc>>;

#[derive(Debug, Error)]
pub enum Error<'gc> {
    #[error(display = "empty operand stack")]
    EmptyOperandStack,
    #[error(display = "{:?}", _0)]
    PhantomData(PhantomData<&'gc ()>),
}
