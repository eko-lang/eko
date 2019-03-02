use std::fmt;

use eko_gc::Arena;

use crate::core::fun::{Constant, Fn, FnProto};
use crate::core::modu::Mod;
use crate::core::value::Value;

use super::error::{Error, Result};

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

    pub fn call(&mut self, arity: u8) -> Result<'gc, Value<'gc>> {
        let mut args = Vec::new();
        for _ in 0..arity {
            args.push(self.operand_stack.pop_fn()?);
        }

        let fun = self.operand_stack.pop_fn()?;

        if fun.arity() != arity {
            return Err(Error::WrongArity {
                expected: fun.arity(),
                received: arity,
            });
        }

        // TODO: Run either the chunk or the external.

        Ok(Value::Boolean(false))
    }

    pub fn push_constant(&mut self, constant: Constant) {
        self.operand_stack
            .push_value(Value::from_constant(self.arena, constant));
    }

    pub fn pop(&mut self) -> Result<'gc, Value<'gc>> {
        self.operand_stack.pop_value()
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
