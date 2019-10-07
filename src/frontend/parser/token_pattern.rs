use crate::frontend::syntax::token::{TokenType, TokenStream, Token};
use crate::frontend::syntax::ast::ExpressionKind;

/// Small parser template which matches an pattern of Tokens
/// and parse them to a expression
pub trait TokenPatternParser{
    /// returns the first
    fn first_token(&self) -> TokenType;

    /// checks if this stream of tokens matches the pattern and can might be parsed
    /// to an expression
    fn matches(&self, tokens: &[&Token]) -> bool;

    /// reads from the tokenstream and parse the tokens to an ExpressionKind.
    /// returns parseError if cant parse correctly. Check with matches() if this
    /// token stream could be parsed
    fn parse_to_expression(&mut self, ts: TokenStream) -> Result<ExpressionKind,ParseError>;
}


#[derive(Eq, PartialEq,Debug,Hash,Clone)]
pub enum ParseError{
    Unknown,
    /// This Number token cant get parsed to a number, its invalid
    NaN(Token),
    /// Found, Expected
    WrongToken(Token,TokenType),
    /// This Token is not allowed outside of a function
    OutOfFnScope(Token),
    Missing(TokenType),
}