#![allow(dead_code)]
use super::{Error, ForthResult, Value};
use autumn::prelude::*;
use std::boxed::Box;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Token {
    Definition(Definition),
    Operation(Operations),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Definition {
    pub name: Operations,
    pub def: Vec<Operations>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Operations {
    Primitive(Primitives),
    UserOp(String),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Primitives {
    Add,
    Sub,
    Mult,
    Div,
    Over,
    Swap,
    Drop,
    Dup,
    Operand(Value),
}

impl FromStr for Primitives {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse::<Value>() {
            return Ok(Primitives::Operand(v));
        }

        let maybe_t = if let Ok(c) = s.parse::<char>() {
            match c {
                '+' => Some(Primitives::Add),
                '-' => Some(Primitives::Sub),
                '*' => Some(Primitives::Mult),
                '/' => Some(Primitives::Div),
                _ => None,
            }
        } else {
            None
        };

        if let Some(t) = maybe_t {
            return Ok(t);
        }

        let s = s.to_uppercase();
        if s == "DUP" {
            return Ok(Primitives::Dup);
        } else if s == "DROP" {
            return Ok(Primitives::Drop);
        } else if s == "OVER" {
            return Ok(Primitives::Over);
        } else if s == "SWAP" {
            return Ok(Primitives::Swap);
        }

        Err(Error::UnknownWord)
    }
}

impl FromStr for Operations {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(primitive) = s.parse::<Primitives>() {
            return Ok(Operations::Primitive(primitive));
        }

        Ok(Operations::UserOp(s.to_uppercase()))
    }
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        {
            let s = s.to_uppercase();
            let mut words_iter = s.split_whitespace();

            if let Some(key) = words_iter.next() {
                if words_iter.clone().count() >= 1 {
                    let key = key.parse::<Operations>()?;
                    if let Operations::Primitive(Primitives::Operand(_)) = key {
                        return Err(Error::InvalidWord);
                    }
                    let values = words_iter
                        .map(|w: &str| FromStr::from_str(w))
                        .collect::<Result<Vec<Operations>, Error>>()?;

                    return Ok(Token::Definition(Definition {
                        name: key,
                        def: values,
                    }));
                }
            }
        }

        Ok(Token::Operation(s.parse::<Operations>()?))
    }
}

pub fn get_tokens(source: &str) -> ForthResult<Vec<Token>> {
    let v: ForthResult<Vec<Token>> = parse(tokens, source)
        .take_values()
        .flat_map(|l| l.reverse().map(|s| FromStr::from_str(&s)))
        .collect();

    if let Ok(v) = &v {
        if v.is_empty() && !source.is_empty() {
            return Err(Error::InvalidWord);
        }
    }
    v
}

fn definitions(source: &str, location: Span) -> ParseResult<List<String>> {
    token
        .delimited_by(space, ..)
        .surrounded_by(":".maybe_space_after(), ";")
        .parse(source, location)
}

fn tokens(source: &str, location: Span) -> ParseResult<List<String>> {
    space.maybe().skip(definitions)
        .on_none(space.maybe().skip(token.delimited_by(space, ..)).end())
        .parse(source, location)
}

fn token(source: &str, location: Span) -> ParseResult<String> {
    value
        .or(operator.or(identifier))
        .copy_string()
        .parse(source, location)
}

fn operator(source: &str, location: Span) -> ParseResult<Span> {
    add.or(sub).or(div).or(mult).parse(source, location)
}

fn add(source: &str, location: Span) -> ParseResult<Span> {
    Parser::parse("+", source, location)
}

fn identifier(source: &str, location: Span) -> ParseResult<Span> {
    alphabetic
        .or("_")
        .and(alphanumeric.or("-").or("_").multiple().maybe())
        .parse(source, location)
}

fn sub(source: &str, location: Span) -> ParseResult<Span> {
    Parser::parse("-", source, location)
}

fn mult(source: &str, location: Span) -> ParseResult<Span> {
    Parser::parse("*", source, location)
}

fn div(source: &str, location: Span) -> ParseResult<Span> {
    Parser::parse("/", source, location)
}

fn value(source: &str, location: Span) -> ParseResult<Span> {
    "-".maybe().and(digit.multiple()).parse(source, location)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -1 2 + / 3 * -
    const EXPECTED: &[Token] = &[
        Token::Operation(Operations::Primitive(Primitives::Operand(-1))),
        Token::Operation(Operations::Primitive(Primitives::Operand(2))),
        Token::Operation(Operations::Primitive(Primitives::Add)),
        Token::Operation(Operations::Primitive(Primitives::Div)),
        Token::Operation(Operations::Primitive(Primitives::Operand(3))),
        Token::Operation(Operations::Primitive(Primitives::Mult)),
        Token::Operation(Operations::Primitive(Primitives::Sub)),
    ];

    #[test]
    fn def_with_other_before_and_after() {
        assert_eq!(
            get_tokens("1 2 : foo + ; foo").expect("get tokens is not ok"),
            &[
                Token::Operation(Operations::Primitive(Primitives::Operand(1))),
                Token::Operation(Operations::Primitive(Primitives::Operand(2))),
                Token::Definition(Definition {
                    name: Operations::UserOp(String::from("FOO")),
                    def: vec![Operations::Primitive(Primitives::Add)]
                }),
                Token::Operation(Operations::UserOp("FOO".to_string()))
            ]
        )
    }

    #[test]
    fn def_word() {
        assert_eq!(
            get_tokens(": foo 1 2 ;").expect("get tokens is not ok"),
            &[Token::Definition(Definition {
                name: Operations::UserOp(String::from("FOO")),
                def: vec![
                    Operations::Primitive(Primitives::Operand(1)),
                    Operations::Primitive(Primitives::Operand(2)),
                ]
            })]
        )
    }

    #[test]
    fn all_works() {
        assert_eq!(
            get_tokens("1 + 2 foo / *").expect("get tokens is not ok"),
            &[
                Token::Operation(Operations::Primitive(Primitives::Operand(1))),
                Token::Operation(Operations::Primitive(Primitives::Add)),
                Token::Operation(Operations::Primitive(Primitives::Operand(2))),
                Token::Operation(Operations::UserOp(String::from("FOO"))),
                Token::Operation(Operations::Primitive(Primitives::Div)),
                Token::Operation(Operations::Primitive(Primitives::Mult)),
            ]
        )
    }
    #[test]
    fn user_defined_works() {
        assert_eq!(
            get_tokens("foo bar").expect("get tokens is not ok"),
            &[
                Token::Operation(Operations::UserOp("FOO".to_string())),
                Token::Operation(Operations::UserOp("BAR".to_string())),
            ]
        )
    }

    #[test]
    fn mix_ident_and_other_works() {
        assert_eq!(
            get_tokens("1 2 swap + 3 *").expect("get tokens is not ok"),
            &[
                Token::Operation(Operations::Primitive(Primitives::Operand(1))),
                Token::Operation(Operations::Primitive(Primitives::Operand(2))),
                Token::Operation(Operations::Primitive(Primitives::Swap)),
                Token::Operation(Operations::Primitive(Primitives::Add)),
                Token::Operation(Operations::Primitive(Primitives::Operand(3))),
                Token::Operation(Operations::Primitive(Primitives::Mult)),
            ]
        )
    }

    #[test]
    fn with_identifier_token_works() {
        assert_eq!(
            get_tokens("SWAP over DuP drop").expect("get tokens is not ok"),
            &[
                Token::Operation(Operations::Primitive(Primitives::Swap)),
                Token::Operation(Operations::Primitive(Primitives::Over)),
                Token::Operation(Operations::Primitive(Primitives::Dup)),
                Token::Operation(Operations::Primitive(Primitives::Drop)),
            ]
        )
    }

    #[test]
    fn normal_case_works() {
        assert_eq!(
            get_tokens("-1 2 + / 3 * -").expect("get tokens is not ok"),
            EXPECTED
        )
    }

    #[test]
    fn preceding_space_works() {
        assert_eq!(
            get_tokens(" -1 2 + / 3 * -").expect("get tokens is not ok"),
            EXPECTED
        )
    }

    #[test]
    fn trailing_space_works() {
        assert_eq!(
            get_tokens("-1 2 + / 3 * - ").expect("get tokens is not ok"),
            EXPECTED
        )
    }

    #[test]
    fn large_spaces_works() {
        assert_eq!(
            get_tokens("-1  2   +  /  3     *         -                 ")
                .expect("get tokens is not ok"),
            EXPECTED
        )
    }

    #[test]
    fn identifiers_can_contain_dash() {
        assert_eq!(
            get_tokens("1 dup-twice").expect("get tokens failed"),
            &[
                Token::Operation(Operations::Primitive(Primitives::Operand(1))),
                Token::Operation(Operations::UserOp(String::from("DUP-TWICE")))
            ]
        )
    }

    #[test]
    fn identifiers_can_contain_underscore() {
        assert_eq!(
            get_tokens("1 _dup_twice").expect("get tokens failed"),
            &[
                Token::Operation(Operations::Primitive(Primitives::Operand(1))),
                Token::Operation(Operations::UserOp(String::from("_DUP_TWICE")))
            ]
        )
    }

    #[test]
    fn identifiers_can_contain_dash_and_underscore() {
        assert_eq!(
            get_tokens("1 _dup-twice-now_-").expect("get tokens failed"),
            &[
                Token::Operation(Operations::Primitive(Primitives::Operand(1))),
                Token::Operation(Operations::UserOp(String::from("_DUP-TWICE-NOW_-")))
            ]
        )
    }
}
