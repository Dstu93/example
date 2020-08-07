use crate::frontend::syntax::token::{TokenType, Token};

#[derive(Eq, PartialEq,Debug,Hash,Clone)]
pub enum ParseError{
    /// Found, Expected
    WrongToken(Token,Vec<TokenType>),
    /// Language Mistake with description
    GrammarMistake(&'static str),
}
