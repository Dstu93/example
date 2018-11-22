use std::str::Chars;
use std::iter::Peekable;
use frontend::syntax::{DataValue,DataType,token::*};

/// Lexer for splitting the source code into a vec of tokens
pub struct Lexer;

impl Lexer {

    /// splits src into Tokens.
    pub fn tokenize(src: &str) -> Result<Vec<Token>,LexerError> {
        let mut tokens = Vec::<Token>::new();
        let mut iter = src.chars().into_iter().peekable();

        loop {

            let c = iter.next();
            if c.is_none(){
                break;
            }
            let c = c.unwrap();
            if c.is_whitespace() {continue;}
            if c == '#' {
                Lexer::skip_comment(&mut iter);
                continue;
            }
            if is_separator(&c) {
                let ttype = separator_to_token_type(&c);
                tokens.push(Token::new(ttype,c.to_string(),0));
                continue;
            }
            if is_operator(&c) {
                let token = if c == '=' && iter.peek().eq(&Some(&'=')){
                    let mut equal = c.to_string();
                    equal.push(iter.next().unwrap());
                    Token::new(TokenType::OperatorEqual,equal,0)
                } else {
                    let kind = operator_to_token_type(&c);
                    Token::new(kind,c.to_string(),0)
                };
                tokens.push(token);
                continue;
            }
            if c == '"' {
                let result = Lexer::read_string(&mut iter);
                if result.is_ok(){tokens.push(result.unwrap())}
                else { return Err(result.unwrap_err()) }
                continue;
            }

            if c.is_numeric(){
                let mut s = c.to_string();
                loop{
                    let read_next = {
                        let peek = iter.peek();
                        if peek.is_none() { break; }
                        let next_char = peek.unwrap();
                        next_char.is_alphanumeric() || next_char == &'.'
                    };
                    if read_next{
                        s.push(iter.next().unwrap());
                    } else { break; }
                }
                if s.contains("."){
                    let token = Token::new(TokenType::LiteralFloat,s,0);
                    tokens.push(token);
                }else {
                    let token = Token::new(TokenType::LiteralInteger, s,0);
                    tokens.push(token);
                }
                continue;
            }

            if c.is_alphabetic() {
                let mut s = String::new();
                s.push(c);
                let result = Lexer::read_identifier(&mut iter, s);
                if result.is_err(){return Err(result.unwrap_err())}
                tokens.push(result.unwrap());
                continue;
            }

            //our char seems to be something else
           return Err(LexerError::UnknownCharacter(c));
        }

        tokens.push(Token::new(TokenType::EoF,"".into(),0));
        Ok(tokens)
    }

    fn read_string(iter: &mut Peekable<Chars>) -> Result<Token,LexerError> {
        let mut string = String::new();
        loop {
            let n = iter.next();
            if n.is_none(){return Err(LexerError::UnexpectedEndOfString);}
            let n = n.unwrap();
            if n == '"' { return Ok(Token::new(TokenType::LiteralString,string,0));}
            string.push(n);
        };
    }

    /// reads identifier or keyword Token from iterator
    fn read_identifier(iter: &mut Peekable<Chars>, mut s: String) -> Result<Token,LexerError> {
        loop {
            let read_next = {
                let peek = iter.peek();
                if peek.is_none() { break; }
                let c = peek.unwrap();
                c.is_alphanumeric()
            };
            if read_next{
                s.push(iter.next().unwrap());
            } else {
                break;
            }
        }
        let kind = match_keyword(&*s);
        return Ok(Token::new(kind,s,0));
    }

    fn skip_comment(iter: &mut Peekable<Chars>){
        loop{
            let c = iter.next();
            if c.is_none(){break;}
            let c = c.unwrap();
            if is_newline(&c){break;}
        }
    }
}

#[derive(Eq, PartialEq,Copy, Clone,Ord, PartialOrd,Hash,Debug)]
pub enum LexerError{
    UnexpectedEndOfString,
    NAN, //not a number
    UnknownCharacter(char),
}

/// language operators like +
fn is_operator(c: &char) -> bool {
    c == &'+' ||
        c == &'-' ||
        c == &'*' ||
        c == &'/' ||
        c == &'=' ||
        c == &'<' ||
        c == &'>' ||
        c == &'!'
}

fn operator_to_token_type(c: &char) -> TokenType{
    match c {
        '+' => TokenType::OperatorPlus,
        '-' => TokenType::OperatorMinus,
        '*' => TokenType::OperatorMultiplication,
        '/' => TokenType::OperatorDivide,
        '=' => TokenType::Assign,
        '!' => TokenType::OperatorNegation,
        '<' => TokenType::OperatorLessThen,
        '>' => TokenType::OperatorGreaterThen,
        _ => panic!("cant parse {} to an operator token",c),
    }
}

/// means brackes and semicolons
fn is_separator(c: &char) -> bool{
    c == &'{' ||
        c == &'}' ||
        c == &'(' ||
        c == &')' ||
        c == &';' ||
        c == &',' ||
        c == &'.' ||
        c == &':'
}

fn separator_to_token_type(c: &char) -> TokenType {
    match c {
        '{' => TokenType::SeparatorCurvedBracketOpen,
        '}' => TokenType::SeparatorCurvedBracketClosed,
        '(' => TokenType::SeparatorBracketOpen,
        ')' => TokenType::SeparatorBracketClose,
        ';' => TokenType::SeparatorSemiColon,
        ',' => TokenType::SeparatorComma,
        '.' => TokenType::SeparatorDot,
        ':' => TokenType::SeparatorColon,
        _ => panic!("cant parse {} to a separator token",c),
    }
}

/// matches an keyword and returns the TokenType, if no keyword matches it returns
/// identifier as TokenType
fn match_keyword(value: &str) -> TokenType{
    match value {
        "let" => TokenType::Let,
        "for" => TokenType::For,
        "loop" => TokenType::Loop,
        "break" => TokenType::Break,
        "continue" => TokenType::Continue,
        "return" => TokenType::Return,
        "while" => TokenType::While,
        "fn" => TokenType::Fn,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "boolean" => TokenType::Boolean,
        "true" => TokenType::BooleanTrue,
        "false" => TokenType::BooleanFalse,
        "integer" => TokenType::Integer,
        "float" => TokenType::Float,
        "string" => TokenType::String,
        _ => TokenType::Identifier
    }
}

fn is_newline(c: &char) -> bool {
    c == &'\n' || c == &'\r'
}
