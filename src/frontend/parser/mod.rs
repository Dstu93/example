pub mod ast_parser;
pub mod error;

use crate::frontend::parser::error::ParseError;
use crate::frontend::syntax::token::{Token, TokenType};
use std::str::FromStr;

/// Trait for Parsing the implementing struct into I which can fail with a ParseError
pub trait ParseInto<I> {
    /// parse this Object into I.
    /// In failing cases a ParseError will returned
    fn parse(&self) -> Result<I,ParseError>;
}

impl ParseInto<i64> for Token {
    fn parse(&self) -> Result<i64, ParseError> {
        match i64::from_str(self.value()) {
            Ok(int) => Ok(int),
            Err(_) => Err(ParseError::WrongToken(self.clone(), vec![TokenType::LiteralInteger])),
        }
    }
}

impl ParseInto<f64> for Token {
    fn parse(&self) -> Result<f64, ParseError> {
        match f64::from_str(self.value()) {
            Ok(float) => Ok(float),
            Err(_) => Err(ParseError::WrongToken(self.clone(),vec![TokenType::LiteralFloat])),
        }
    }
}
