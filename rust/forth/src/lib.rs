use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;
type MyResult<T> = Result<T, Error>;

pub struct Forth {
    stack: Vec<Value>,
    operators: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        let mut operators = HashMap::new();
        operators.insert("DUP".to_string(), "DUP".to_string());
        operators.insert("OVER".to_string(), "OVER".to_string());
        operators.insert("DROP".to_string(), "DROP".to_string());
        operators.insert("SWAP".to_string(), "SWAP".to_string());
        operators.insert("SUM".to_string(), "SUM".to_string());
        operators.insert("PRINT".to_string(), "PRINT".to_string());
        operators.insert("LEN".to_string(), "LEN".to_string());
        operators.insert("+".to_string(), "+".to_string());
        operators.insert("-".to_string(), "-".to_string());
        operators.insert("*".to_string(), "*".to_string());
        operators.insert("/".to_string(), "/".to_string());
        Forth {
            stack: Vec::new(),
            operators,
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }
    fn eval_word(&mut self, word: Word) -> ForthResult {
        Ok(match word {
            Word::Operand(v) => self.stack.push(v),
            Word::Operator(s) => {
                let op = self.operators.get(&s).ok_or(Error::UnknownWord)?.clone();
                match op.as_ref() {
                    "DUP" => self
                        .stack
                        .push(*self.stack.last().ok_or(Error::StackUnderflow)?),
                    "OVER" => self.stack.push(
                        *self
                            .stack
                            .get(
                                self.stack
                                    .len()
                                    .checked_sub(2)
                                    .ok_or(Error::StackUnderflow)?,
                            )
                            .ok_or(Error::StackUnderflow)?,
                    ),
                    "DROP" => {
                        self.stack.pop().ok_or(Error::StackUnderflow)?;
                    }
                    "SWAP" => {
                        let (v1, v2) = self.get_binary_operands()?;
                        self.stack.push(v2);
                        self.stack.push(v1);
                    }
                    "SUM" => {
                        let sum = self.stack.drain(..).sum();
                        self.stack.push(sum)
                    }
                    "LEN" => {
                        self.stack.push(self.stack.len() as i32)
                    }
                    "PRINT" => {
                        println!("{}", &self)
                    }
                    "+" => {
                        let (v1, v2) = self.get_binary_operands()?;
                        self.stack.push(v1 + v2);
                    }
                    "-" => {
                        let (v1, v2) = self.get_binary_operands()?;
                        self.stack.push(v1 - v2);
                    }
                    "*" => {
                        let (v1, v2) = self.get_binary_operands()?;
                        self.stack.push(v1 * v2);
                    }
                    "/" => {
                        let (v1, v2) = self.get_binary_operands()?;
                        self.stack
                            .push(v1.checked_div(v2).ok_or(Error::DivisionByZero)?)
                    }
                    s => {
                        self.eval(s)?;
                    }
                }
            }
        })
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        for word in self.parse(input)? {
            self.eval_word(word)?
        }

        Ok(())
    }

    fn get_binary_operands(&mut self) -> MyResult<(Value, Value)> {
        let v1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let v2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
        Ok((v2, v1))
    }

    fn parse(&mut self, input: &str) -> MyResult<Vec<Word>> {
        if let Some(colon) = input.find(":") {
            let semi = input.find(";").ok_or(Error::InvalidWord)?;
            let user_def: Vec<&str> = input[colon + 1..semi].split_ascii_whitespace().collect();
            let key = user_def
                .first()
                .ok_or(Error::UnknownWord)?
                .to_ascii_uppercase();

            if key.parse::<Value>().is_ok() {
                return Err(Error::InvalidWord);
            }

            let value: String = user_def[1..]
                .join(" ")
                .to_ascii_uppercase()
                .split_ascii_whitespace()
                .map(|s| {
                    if let Some(value) = self.operators.get(s) {
                        value.clone()
                    } else {
                        s.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");

            self.operators.insert(key, value);

            self.parse(&format!("{} {}", &input[..colon], &input[semi + 1..]))
        } else {
            let mut v = Vec::new();
            for w in input
                .split_ascii_whitespace()
                .map(|w| w.to_ascii_uppercase())
            {
                if self.operators.contains_key(&w) {
                    v.push(Word::Operator(w))
                } else {
                    v.push(w.parse::<Word>()?);
                }
            }
            Ok(v)
        }
    }
}

impl fmt::Display for Forth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = String::new();
        for v in &self.stack {
            builder += &format!("{} ", v);
        }
        write!(f, "[{}]", builder)
    }
}

#[derive(Debug, Clone)]
enum Word {
    Operand(Value),
    Operator(String),
}

impl FromStr for Word {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse::<Value>() {
            Ok(Self::Operand(v))
        } else {
            Ok(Self::Operator(s.to_ascii_uppercase()))
        }
    }
}
