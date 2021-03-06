use crate::frontend::syntax::token::{TokenType, TokenStream, Token};
use crate::frontend::syntax::ast::Expression;

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
    fn parse_to_expression(&mut self, ts: TokenStream) -> Result<Expression,ParseError>;
}


#[derive(Eq, PartialEq,Debug,Hash,Clone)]
pub enum ParseError{
    /// Found, Expected
    WrongToken(Token,Vec<TokenType>),
    /// Language Mistake with description
    GrammarMistake(&'static str),
}