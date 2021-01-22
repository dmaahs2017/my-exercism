#![feature(try_trait)]
mod parse;

use parse::*;
use std::collections::HashMap;
use std::fmt;

pub type Value = i32;
pub type ForthResult<T = ()> = Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl From<std::option::NoneError> for Error {
    fn from(_: std::option::NoneError) -> Self {
        Error::StackUnderflow
    }
}

#[derive(Debug)]
pub struct Forth {
    stack: Vec<Value>,
    user_words: HashMap<Operations, Vec<Primitives>>,
}

impl Forth {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            user_words: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut ops: Vec<Operations> = Vec::new();
        let mut defs: Vec<Definition> = Vec::new();
        for token in get_tokens(input)? {
            match token {
                Token::Definition(d) => defs.push(d),
                Token::Operation(op) => ops.push(op),
            }
        }

        dbg!(&self);
        dbg!(&defs);
        self.eval_defs(defs)?;
        self.eval_ops(ops)?;
        Ok(())
    }

    fn reduce(&self, ops: Vec<Operations>) -> ForthResult<Vec<Primitives>> {
        Ok(ops
            .iter()
            .map(|op| match op {
                Operations::UserOp(s) => self
                    .user_words
                    .get(&Operations::UserOp(s.clone()))
                    .ok_or(Error::UnknownWord)
                    .map(|v| v.clone()),
                Operations::Primitive(p) => Ok(vec![p.clone()]),
            })
            .flatten()
            .flatten()
            .collect::<Vec<_>>())
    }

    fn eval_defs(&mut self, defs: Vec<Definition>) -> ForthResult {
        for def in defs {
            self.user_words.insert(def.name, self.reduce(def.def)?);
        }

        Ok(())
    }

    fn eval_primitive(&mut self, primitive: Primitives) -> ForthResult {
        match primitive {
            Primitives::Operand(v) => self.stack.push(v),
            Primitives::Add => self.binary_operation(|a, b| Ok(a + b))?,
            Primitives::Sub => self.binary_operation(|a, b| Ok(a - b))?,
            Primitives::Mult => self.binary_operation(|a, b| Ok(a * b))?,
            Primitives::Div => {
                self.binary_operation(|a, b| Ok(a.checked_div(b).ok_or(Error::DivisionByZero)?))?;
            }
            Primitives::Over => {
                let v1 = self.stack.iter().rev().skip(1).next()?.clone();
                self.stack.push(v1);
            }
            Primitives::Dup => {
                let v1 = self.stack.last()?.clone();
                self.stack.push(v1);
            }
            Primitives::Swap => {
                let v1 = self.stack.pop()?;
                let v2 = self.stack.pop()?;
                self.stack.push(v1);
                self.stack.push(v2);
            }
            Primitives::Drop => {
                self.stack.pop()?;
            }
        }
        Ok(())
    }

    fn eval_ops(&mut self, operations: Vec<Operations>) -> ForthResult {
        for op in operations {
            if let Some(primitives) = self.user_words.get(&op) {
                for p in primitives.clone() {
                    self.eval_primitive(p)?;
                }
            } else {
                match op {
                    Operations::UserOp(word) => {
                        let definition = self
                            .user_words
                            .get(&Operations::UserOp(word))
                            .ok_or(Error::UnknownWord)?
                            .clone();
                        for p in definition {
                            self.eval_primitive(p)?;
                        }
                    }
                    Operations::Primitive(p) => {
                        self.eval_primitive(p)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn binary_operation<F>(&mut self, f: F) -> ForthResult
    where
        F: Fn(Value, Value) -> ForthResult<Value>,
    {
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        self.stack.push(f(a, b)?);
        Ok(())
    }
}

impl fmt::Display for Forth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_works() {
        let mut f = Forth::new();
        f.user_words.insert(
            Operations::UserOp(String::from("FOO")),
            vec![Primitives::Operand(1), Primitives::Add],
        );

        assert_eq!(
            f.reduce(vec![
                Operations::Primitive(Primitives::Mult),
                Operations::UserOp(String::from("FOO")),
            ])
            .expect("reduce is not ok"),
            vec![Primitives::Mult, Primitives::Operand(1), Primitives::Add]
        )
    }

    #[test]
    fn def_works() {
        let mut f = Forth::new();
        f.eval(": foo 10 ;").expect("Definition to work");
        assert_eq!(
            f.user_words.get(&Operations::UserOp(String::from("FOO"))),
            Some(&vec![Primitives::Operand(10)])
        )
    }
}
