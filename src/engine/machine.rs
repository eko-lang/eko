use std::fmt;

use eko_gc::Arena;

use crate::core::fun::{Chunk, Fn, FnProto};
use crate::core::instr::Instr;
use crate::core::modu::Mod;
use crate::core::value::Value;

use super::error::{Error, Result};
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
            args.push(self.operand_stack.pop_value()?);
        }

        let fun = self.operand_stack.pop_fn()?;

        if fun.arity() != arity {
            return Err(Error::WrongArity {
                expected: fun.arity(),
                received: arity,
            });
        }

        match fun.proto() {
            FnProto::Chunk(chunk) => self.call_chunk(chunk.clone(), args),
            FnProto::External(_) => {
                // TODO: Call the external function.
                unimplemented!();
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

        for variable in 0..args.len() {
            // TODO: Remove the `expect`.
            frame.local_scope().set(variable, args.pop().unwrap())?;
        }

        while let Some(instr) = frame.step() {
            match instr {
                PushValue { value } => self.push_value(value),
                PushMod { modu } => self.push_mod(modu),
                PushFn { fun } => self.push_fn(fun),
                Pop => self.pop().map(|_| ())?,

                PushVar { var } => self.push_var(&frame, var)?,
                PopVar { var } => self.pop_var(&frame, var)?,

                Add => self.add()?,
                Subtract => self.subtract()?,
                Multiply => self.multiply()?,
                Divide => self.divide()?,

                Call { arity } => self.call(arity)?,
            }
        }

        Ok(())
    }

    pub fn push_value(&mut self, value: Value<'gc>) {
        self.operand_stack.push_value(value);
    }

    pub fn push_mod(&mut self, modu: Mod<'gc>) {
        self.operand_stack.push_mod(modu);
    }

    pub fn push_fn(&mut self, fun: Fn<'gc>) {
        self.operand_stack.push_fn(fun);
    }

    pub fn pop(&mut self) -> Result<'gc, ()> {
        self.operand_stack.pop_value().map(|_| ())
    }

    pub fn push_var(
        &mut self,
        frame: &Frame<'gc>,
        var: usize,
    ) -> Result<'gc, ()> {
        let value = frame.local_scope().get(var)?.clone();
        Ok(self.operand_stack.push_value(value))
    }

    pub fn pop_var(
        &mut self,
        frame: &Frame<'gc>,
        var: usize,
    ) -> Result<'gc, ()> {
        let value = self.operand_stack.pop_value()?;
        frame.local_scope().set(var, value)
    }

    pub fn add(&mut self) -> Result<'gc, ()> {
        let right_value = self.operand_stack.pop_value()?;
        let left_value = self.operand_stack.pop_value()?;

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

        Ok(self.operand_stack.push_value(value))
    }

    pub fn subtract(&mut self) -> Result<'gc, ()> {
        let right_value = self.operand_stack.pop_value()?;
        let left_value = self.operand_stack.pop_value()?;

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

        Ok(self.operand_stack.push_value(value))
    }

    pub fn multiply(&mut self) -> Result<'gc, ()> {
        let right_value = self.operand_stack.pop_value()?;
        let left_value = self.operand_stack.pop_value()?;

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

        Ok(self.operand_stack.push_value(value))
    }

    pub fn divide(&mut self) -> Result<'gc, ()> {
        let right_value = self.operand_stack.pop_value()?;
        let left_value = self.operand_stack.pop_value()?;

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

        Ok(self.operand_stack.push_value(value))
    }
}

pub struct OperandStack<'gc>(Vec<Operand<'gc>>);

impl<'gc> OperandStack<'gc> {
    pub fn new() -> OperandStack<'gc> {
        OperandStack(Vec::new())
    }

    pub fn push_mod(&mut self, modu: Mod<'gc>) {
        self.0.push(Operand::Mod(modu))
    }

    pub fn pop_mod(&mut self) -> Result<'gc, Mod<'gc>> {
        use self::Operand::*;

        match self.0.pop() {
            Some(Mod(modu)) => Ok(modu),
            Some(operand) => Err(Error::InvalidOperandKind {
                expected: OperandKind::Mod,
                received: operand.into(),
            }),
            None => Err(Error::EmptyOperandStack),
        }
    }

    pub fn push_fn(&mut self, fun: Fn<'gc>) {
        self.0.push(Operand::Fn(fun))
    }

    pub fn pop_fn(&mut self) -> Result<'gc, Fn<'gc>> {
        use self::Operand::*;

        match self.0.pop() {
            Some(Fn(fun)) => Ok(fun),
            Some(operand) => Err(Error::InvalidOperandKind {
                expected: OperandKind::Fn,
                received: operand.into(),
            }),
            None => Err(Error::EmptyOperandStack),
        }
    }

    pub fn push_value(&mut self, value: Value<'gc>) {
        self.0.push(Operand::Value(value))
    }

    pub fn pop_value(&mut self) -> Result<'gc, Value<'gc>> {
        use self::Operand::*;

        match self.0.pop() {
            Some(Value(value)) => Ok(value),
            Some(operand) => Err(Error::InvalidOperandKind {
                expected: OperandKind::Value,
                received: operand.into(),
            }),
            None => Err(Error::EmptyOperandStack),
        }
    }
}

pub enum Operand<'gc> {
    Mod(Mod<'gc>),
    Fn(Fn<'gc>),
    Value(Value<'gc>),
}

#[derive(Debug)]
pub enum OperandKind {
    Mod,
    Fn,
    Value,
}

impl<'gc> From<Operand<'gc>> for OperandKind {
    fn from(operand: Operand<'gc>) -> OperandKind {
        use self::OperandKind::*;

        match operand {
            Operand::Mod(_) => Mod,
            Operand::Fn(_) => Fn,
            Operand::Value(_) => Value,
        }
    }
}

impl fmt::Display for OperandKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::OperandKind::*;

        match self {
            Mod => write!(f, "mod"),
            Fn => write!(f, "fn"),
            Value => write!(f, "value"),
        }
    }
}

// TODO: Remove all the `unwrap`s.
#[cfg(test)]
mod tests {
    use eko_gc::Arena;

    use crate::compiler::generator::ChunkBuilder;
    use crate::core::fun::Fn;
    use crate::core::instr::Instr;
    use crate::core::value::Value;
    use crate::engine::frame::Frame;

    use super::Machine;

    #[test]
    fn push_value() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        machine.push_value(Value::Integer(2));

        assert_eq!(
            machine.operand_stack.pop_value().unwrap(),
            Value::Integer(2),
        );
    }

    #[test]
    fn pop_push_var() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        let mut chunk = ChunkBuilder::new();
        let var = chunk.next_var();
        let chunk = chunk.build(&arena);

        let frame = Frame::new(&arena, chunk);

        machine.push_value(Value::Integer(2));
        machine.pop_var(&frame, var).unwrap();
        machine.push_var(&frame, var).unwrap();

        assert_eq!(
            machine.operand_stack.pop_value().unwrap(),
            Value::Integer(2),
        );
    }

    #[test]
    fn call_chunk() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        let mut chunk = ChunkBuilder::new();
        chunk.instr(Instr::Pop);
        chunk.instr(Instr::PushValue {
            value: Value::Integer(3),
        });
        let chunk = chunk.build(&arena);

        machine.push_value(Value::Integer(2));
        machine.push_fn(Fn::new_chunk(&arena, 0, chunk));
        machine.call(0).unwrap();

        assert_eq!(
            machine.operand_stack.pop_value().unwrap(),
            Value::Integer(3),
        );
    }

    #[test]
    fn call_external() {}

    #[test]
    fn add() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        machine.push_value(Value::Integer(2));
        machine.push_value(Value::Integer(5));
        machine.add().unwrap();

        assert_eq!(
            machine.operand_stack.pop_value().unwrap(),
            Value::Integer(7),
        );
    }

    #[test]
    fn subtract() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        machine.push_value(Value::Integer(2));
        machine.push_value(Value::Float(5.0));
        machine.subtract().unwrap();

        assert_eq!(
            machine.operand_stack.pop_value().unwrap(),
            Value::Float(-3.0),
        );
    }

    #[test]
    fn multiply() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        machine.push_value(Value::Float(2.5));
        machine.push_value(Value::Integer(4));
        machine.multiply().unwrap();

        assert_eq!(
            machine.operand_stack.pop_value().unwrap(),
            Value::Float(10.0),
        );
    }

    #[test]
    fn divide() {
        let arena = Arena::new();
        let mut machine = Machine::new(&arena);

        machine.push_value(Value::Integer(20));
        machine.push_value(Value::Integer(6));
        machine.divide().unwrap();

        assert_eq!(
            machine.operand_stack.pop_value().unwrap(),
            Value::Integer(3),
        );
    }
}
