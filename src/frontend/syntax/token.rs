use std::sync::mpsc::Receiver;

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
    Boolean,
    Integer,
    Float,
    String,
    BooleanTrue,
    BooleanFalse,
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
    /// End of File
    EoF,
}

/// Struct to represent an token in our language.
/// A Token is the smallest unit of our language, its
/// represents keywords, names of variables (Identifier) or punctuation like ';' ',' '{'
#[derive(Eq, PartialEq,Debug,Hash,Clone)]
pub struct Token {
    kind: TokenType,
    value: String,
    start_position: usize,
}

impl Token {

    /// creates a new Token.
    pub fn new(kind: TokenType, value: String, start_position: usize) -> Token{
        Token{ kind,value,start_position}
    }

    /// returns the position where this tokens begins.
    pub fn start(&self) -> usize {
        self.start_position
    }

    /// returns the position where this token ends.
    pub fn end(&self) -> usize{
        self.start_position + self.value.len()
    }

    pub fn kind(&self) -> TokenType{
        self.kind
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    /// Consumes this tokens and returns the owned value String
    pub fn move_value(self) -> String{
        self.value
    }
}


/// Stream of Tokens.
pub struct TokenStream {
    rx: Receiver<Token>,
}

impl TokenStream{

    /// creates a new empty stream with a receiver to fill this stream
    pub fn new(rx: Receiver<Token>) -> Self{
        TokenStream{rx}
    }

    /// read next token from this stream and blocks the calling thread till a token is received.
    /// otherwise it will returns None if the stream closed and will never send a next token.
    /// The last token is always an EOF-Token, except the producer fails
    pub fn next(&self) -> Option<Token>{
        match self.rx.recv() {
            Ok(t) => Some(t),
            Err(_) => None,
        }
    }

    /// consumes this stream and collects every token.
    /// this function blocks the calling thread till every token is received
    pub fn collect(self) -> Vec<Token>{
        self.rx.iter().collect()
    }

}