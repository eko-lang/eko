use std::convert::{TryFrom, TryInto};

use eko_gc::Arena;

use crate::core::fun::{self, Chunk, Fun, FunProto};
use crate::core::instr::Instr;
use crate::core::modu::Modu;
use crate::core::value::Value;

use super::error::{Error, OperandKind, Result};
use super::frame::Frame;

pub struct Machine<'a, 'gc> {
    arena: &'a Arena<'gc>,
    operand_stack: OperandStack<'gc>,
}

impl<'a, 'gc> Machine<'a, 'gc> {
    pub fn new(arena: &'a Arena<'gc>) -> Machine<'a, 'gc> {
        Machine {
            arena,
            operand_stack: OperandStack::new(),
        }
    }

    pub fn call(&mut self, arity: u8) -> Result<'gc, ()> {
        let mut args = Vec::new();
        for _ in 0..arity {
            args.push(self.operand_stack.pop()?.try_into()?);
        }

        let fun = match self.operand_stack.pop()? {
            Operand::Fun(fun) => fun,
            Operand::Method(method) => {
                if method.0.is_method() {
                    method.0
                } else {
                    return Err(Error::MethodNotFound {
                        ident: method.0.ident().clone(),
                    });
                }
            }
            operand => {
                // TODO: Should `expected` be a `Vec`?
                return Err(Error::InvalidOperandKind {
                    expected: OperandKind::Fun,
                    received: operand.into(),
                });
            }
        };

        if fun.arity() != arity {
            return Err(Error::WrongArity {
                expected: fun.arity(),
                received: arity,
            });
        }

        match fun.proto() {
            FunProto::Chunk(chunk) => self.call_chunk(chunk.clone(), args),
            FunProto::External(external) => {
                self.call_external(external.clone(), args)
            }
        }
    }

    fn call_chunk(
        &mut self,
        chunk: Chunk<'gc>,
        mut args: Vec<Value<'gc>>,
    ) -> Result<'gc, ()> {
        use self::Instr::*;

        let mut frame = Frame::new(self.arena, chunk);

        for var in 0..args.len() {
            // TODO: Change to use `expect`.
            frame.local_scope().set(var, args.pop().unwrap())?;
        }

        while let Some(instr) = frame.step() {
            match instr {
                Value(value) => self.value(value),

                Load(var) => self.load(&frame, var)?,
                Store(var) => self.store(&frame, var)?,

                Pop => self.pop().map(|_| ())?,

                Add => self.add()?,
                Subtract => self.subtract()?,
                Multiply => self.multiply()?,
                Divide => self.divide()?,

                Call(arity) => self.call(arity)?,
            }
        }

        Ok(())
    }

    fn call_external(
        &mut self,
        external: fun::External<'gc>,
        args: Vec<Value<'gc>>,
    ) -> Result<'gc, ()> {
        let value = external.call(args);
        Ok(self.operand_stack.push(value.into()))
    }

    pub fn value(&mut self, value: Value<'gc>) {
        self.operand_stack.push(value.into());
    }

    pub fn load(&mut self, frame: &Frame<'gc>, var: usize) -> Result<'gc, ()> {
        let value = frame.local_scope().get(var)?.clone();
        Ok(self.operand_stack.push(value.into()))
    }

    pub fn store(&mut self, frame: &Frame<'gc>, var: usize) -> Result<'gc, ()> {
        let value = self.operand_stack.pop()?.try_into()?;
        frame.local_scope().set(var, value)
    }

    pub fn pop(&mut self) -> Result<'gc, ()> {
        let _value: Value<'_> = self.operand_stack.pop()?.try_into()?;
        Ok(())
    }

    pub fn add(&mut self) -> Result<'gc, ()> {
        let right_value = self.operand_stack.pop()?.try_into()?;
        let left_value = self.operand_stack.pop()?.try_into()?;

        let value = match (left_value, right_value) {
            (Value::Integer(left), Value::Integer(right)) => {
                Value::Integer(left + right)
            }
            (Value::Integer(left), Value::Float(right)) => {
                Value::Float(left as f64 + right)
            }
            (Value::Float(left), Value::Integer(right)) => {
                Value::Float(left + right as f64)
            }
            (Value::Float(left), Value::Float(right)) => {
                Value::Float(left + right)
            }
            _ => unimplemented!(),
        };

        Ok(self.operand_stack.push(value.into()))
    }

    pub fn subtract(&mut self) -> Result<'gc, ()> {
        let right_value = self.operand_stack.pop()?.try_into()?;
        let left_value = self.operand_stack.pop()?.try_into()?;

        let value = match (left_value, right_value) {
            (Value::Integer(left), Value::Integer(right)) => {
                Value::Integer(left - right)
            }
            (Value::Integer(left), Value::Float(right)) => {
                Value::Float(left as f64 - right)
            }
            (Value::Float(left), Value::Integer(right)) => {
                Value::Float(left - right as f64)
            }
            (Value::Float(left), Value::Float(right)) => {
                Value::Float(left - right)
            }
            _ => unimplemented!(),
        };

        Ok(self.operand_stack.push(value.into()))
    }

    pub fn multiply(&mut self) -> Result<'gc, ()> {
        let right_value = self.operand_stack.pop()?.try_into()?;
        let left_value = self.operand_stack.pop()?.try_into()?;

        let value = match (left_value, right_value) {
            (Value::Integer(left), Value::Integer(right)) => {
                Value::Integer(left * right)
            }
            (Value::Integer(left), Value::Float(right)) => {
                Value::Float(left as f64 - right)
            }
            (Value::Float(left), Value::Integer(right)) => {
                Value::Float(left * right as f64)
            }
            (Value::Float(left), Value::Float(right)) => {
                Value::Float(left * right)
            }
            _ => unimplemented!(),
        };

        Ok(self.operand_stack.push(value.into()))
    }

    pub fn divide(&mut self) -> Result<'gc, ()> {
        let right_value = self.operand_stack.pop()?.try_into()?;
        let left_value = self.operand_stack.pop()?.try_into()?;

        let value = match (left_value, right_value) {
            (Value::Integer(left), Value::Integer(right)) => {
                Value::Integer(left / right)
            }
            (Value::Integer(left), Value::Float(right)) => {
                Value::Float(left as f64 / right)
            }
            (Value::Float(left), Value::Integer(right)) => {
                Value::Float(left / right as f64)
            }
            (Value::Float(left), Value::Float(right)) => {
                Value::Float(left / right)
            }
            _ => unimplemented!(),
        };

        Ok(self.operand_stack.push(value.into()))
    }
}

pub struct OperandStack<'gc>(Vec<Operand<'gc>>);

impl<'gc> OperandStack<'gc> {
    pub fn new() -> OperandStack<'gc> {
        OperandStack(Vec::new())
    }

    pub fn push(&mut self, operand: Operand<'gc>) {
        self.0.push(operand)
    }

    pub fn pop(&mut self) -> Result<'gc, Operand<'gc>> {
        self.0.pop().ok_or_else(|| Error::EmptyOperandStack)
    }
}

pub enum Operand<'gc> {
    Modu(Modu<'gc>),

    Fun(Fun<'gc>),
    Method(Method<'gc>),

    Value(Value<'gc>),
}

impl<'gc> From<Modu<'gc>> for Operand<'gc> {
    fn from(modu: Modu<'gc>) -> Operand<'gc> {
        Operand::Modu(modu)
    }
}

impl<'gc> TryFrom<Operand<'gc>> for Modu<'gc> {
    type Error = Error<'gc>;

    fn try_from(operand: Operand<'gc>) -> Result<'gc, Modu<'gc>> {
        match operand {
            Operand::Modu(modu) => Ok(modu),
            operand => Err(Error::InvalidOperandKind {
                expected: OperandKind::Modu,
                received: operand.into(),
            }),
        }
    }
}

impl<'gc> From<Method<'gc>> for Operand<'gc> {
    fn from(method: Method<'gc>) -> Operand<'gc> {
        Operand::Method(method)
    }
}

impl<'gc> TryFrom<Operand<'gc>> for Method<'gc> {
    type Error = Error<'gc>;

    fn try_from(operand: Operand<'gc>) -> Result<'gc, Method<'gc>> {
        match operand {
            Operand::Method(method) => Ok(method),
            operand => Err(Error::InvalidOperandKind {
                expected: OperandKind::Method,
                received: operand.into(),
            }),
        }
    }
}

impl<'gc> From<Fun<'gc>> for Operand<'gc> {
    fn from(fun: Fun<'gc>) -> Operand<'gc> {
        Operand::Fun(fun)
    }
}

impl<'gc> TryFrom<Operand<'gc>> for Fun<'gc> {
    type Error = Error<'gc>;

    fn try_from(operand: Operand<'gc>) -> Result<'gc, Fun<'gc>> {
        match operand {
            Operand::Fun(fun) => Ok(fun),
            operand => Err(Error::InvalidOperandKind {
                expected: OperandKind::Fun,
                received: operand.into(),
            }),
        }
    }
}

impl<'gc> From<Value<'gc>> for Operand<'gc> {
    fn from(value: Value<'gc>) -> Operand<'gc> {
        Operand::Value(value)
    }
}

impl<'gc> TryFrom<Operand<'gc>> for Value<'gc> {
    type Error = Error<'gc>;

    fn try_from(operand: Operand<'gc>) -> Result<'gc, Value<'gc>> {
        match operand {
            Operand::Value(value) => Ok(value),
            operand => Err(Error::InvalidOperandKind {
                expected: OperandKind::Value,
                received: operand.into(),
            }),
        }
    }
}

#[derive(Debug, Trace)]
pub struct Method<'gc>(Fun<'gc>);

// TODO: Remove all the `unwrap`s.
#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use eko_gc::Arena;

    use crate::compiler::generator::ChunkBuilder;
    use crate::core::fun::{External, Fun};
    use crate::core::instr::Instr;
    use crate::core::value::Value;
    use crate::engine::frame::Frame;

    use super::Machine;

    macro_rules! assert_ret {
        ($machine:ident, $value:expr) => {
            let value: Value<'_> =
                $machine.operand_stack.pop().unwrap().try_into().unwrap();
            assert_eq!(value, $value);
        };
    }

    #[test]
    fn value() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        machine.value(Value::Integer(2));

        assert_ret!(machine, Value::Integer(2));
    }

    #[test]
    fn store_load() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        let mut chunk = ChunkBuilder::new();
        let var = chunk.next_var();
        let chunk = chunk.build(&arena);

        let frame = Frame::new(&arena, chunk);

        machine.value(Value::Integer(2));
        machine.store(&frame, var).unwrap();
        machine.load(&frame, var).unwrap();

        assert_ret!(machine, Value::Integer(2));
    }

    #[test]
    fn call_chunk() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        let mut chunk = ChunkBuilder::new();
        chunk.instr(Instr::Pop);
        chunk.instr(Instr::Value(Value::Integer(3)));
        let chunk = chunk.build(&arena);

        machine.value(Value::Integer(2));
        machine
            .operand_stack
            .push(Fun::new_chunk(&arena, 0, chunk).into());
        machine.call(0).unwrap();

        assert_ret!(machine, Value::Integer(3));
    }

    #[test]
    fn call_external() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        let external = External::new(&arena, |_| Value::Integer(7));

        machine
            .operand_stack
            .push(Fun::new_external(&arena, 0, external).into());
        machine.call(0).unwrap();

        assert_ret!(machine, Value::Integer(7));
    }

    #[test]
    fn add() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        machine.value(Value::Integer(2));
        machine.value(Value::Integer(5));
        machine.add().unwrap();

        assert_ret!(machine, Value::Integer(7));
    }

    #[test]
    fn subtract() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        machine.value(Value::Integer(2));
        machine.value(Value::Float(5.0));
        machine.subtract().unwrap();

        assert_ret!(machine, Value::Float(-3.0));
    }

    #[test]
    fn multiply() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        machine.value(Value::Float(2.5));
        machine.value(Value::Integer(4));
        machine.multiply().unwrap();

        assert_ret!(machine, Value::Float(10.0));
    }

    #[test]
    fn divide() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        machine.value(Value::Integer(20));
        machine.value(Value::Integer(6));
        machine.divide().unwrap();

        assert_ret!(machine, Value::Integer(3));
    }
}
