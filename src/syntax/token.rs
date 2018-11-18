

/// Defines the Type of a Token.
/// Types means in this case its meaning
#[derive(Copy, Clone,Eq, PartialEq,Ord, PartialOrd,Hash,Debug)]
pub enum TokenType {
    Let,
    For,
    Loop,
    Break,
    Continue,
    Return,
    While,
    Fn,
    Assign,
    If,
    Else,
    Identifier,
    SeparatorCurvedBracketOpen,
    SeparatorCurvedBracketClosed,
    SeparatorBracketOpen,
    SeparatorBracketClose,
    SeparatorSemiColon,
    SeparatorColon,
    SeparatorComma,
    SeparatorDot,
    OperatorPlus,
    OperatorMinus,
    OperatorMultiplication,
    OperatorDivide,
    OperatorEqual,
    OperatorNegation,
    OperatorLessThen,
    OperatorGreaterThen,
    LiteralInteger,
    LiteralFloat,
    LiteralBoolean,
    LiteralString,
}

//TODO explain meaning of a token
/// Struct to represent an token in our language.
#[derive(Eq, PartialEq,Debug,Hash)]
pub struct Token{
    token_type: TokenType,
    value: String,
    start_position: usize,
}

impl Token {

    /// creates a new Token.
    pub fn new(token_type: TokenType, value: String, start_position: usize) -> Token{
        Token{token_type,value,start_position}
    }

    /// returns the position where this tokens begins.
    pub fn start(&self) -> usize {
        self.start_position
    }

    /// returns the position where this token ends.
    pub fn end(&self) -> usize{
        self.start_position + self.value.len()
    }

    pub fn token_type(&self) -> TokenType{
        self.token_type
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    /// Consumes this tokens and returns the owned value String
    pub fn move_value(self) -> String{
        self.value
    }
}
