

use frontend::syntax::{*,token::*};
use frontend::lexer::*;

#[test]
fn tokenizer_operator_test(){
    //TODO Equal only if == so = is always an Assignment
    let src = "=";
    let equal = Lexer::tokenize(&src).unwrap();
    let expected = vec![Token::new(TokenType::OperatorEqual,"=".into(),0),eof()];
    assert_eq!(equal,expected);

    let src = "+";
    let plus = Lexer::tokenize(&src).unwrap();
    let expected = vec![Token::new(TokenType::OperatorPlus,"+".into(),0),eof()];
    assert_eq!(plus,expected);

    let src = "-";
    let minus = Lexer::tokenize(&src).unwrap();
    let expected = vec![Token::new(TokenType::OperatorMinus,"-".into(),0),eof()];
    assert_eq!(minus,expected);

    let src = "*";
    let multi = Lexer::tokenize(&src).unwrap();
    let expected = vec![Token::new(TokenType::OperatorMultiplication,"*".into(),0),eof()];
    assert_eq!(multi,expected);

    let src = "/";
    let divide = Lexer::tokenize(&src).unwrap();
    let expected = vec![Token::new(TokenType::OperatorDivide,"/".into(),0),eof()];
    assert_eq!(divide,expected);

    let src = " <   ";
    let less = Lexer::tokenize(src).unwrap();
    let expected = vec![Token::new(TokenType::OperatorLessThen,"<".into(),0),eof()];
    assert_eq!(less,expected);

    let src = "  > ";
    let greater_then = Lexer::tokenize(src).unwrap();
    let expected = vec![Token::new(TokenType::OperatorGreaterThen,">".into(),0),eof()];
    assert_eq!(greater_then,expected);
}


#[test]
fn tokenizer_separator_test(){

    let curved_bracket_open = "{";
    let tokens = Lexer::tokenize(&curved_bracket_open).unwrap();
    let expected = vec![Token::new(TokenType::SeparatorCurvedBracketOpen,curved_bracket_open.to_string(),0),eof()];
    assert_eq!(expected,tokens);

    let curved_bracket_closed = "}";
    let tokens = Lexer::tokenize(&curved_bracket_closed).unwrap();
    let expected = vec![Token::new(TokenType::SeparatorCurvedBracketClosed,curved_bracket_closed.to_string(),0),eof()];
    assert_eq!(expected,tokens);

    let bracket_open = "(";
    let tokens = Lexer::tokenize(&bracket_open).unwrap();
    let expected = vec![Token::new(TokenType::SeparatorBracketOpen,bracket_open.to_string(),0),eof()];
    assert_eq!(expected,tokens);

    let bracket_closed = ")";
    let tokens = Lexer::tokenize(&bracket_closed).unwrap();
    let expected = vec![Token::new(TokenType::SeparatorBracketClose,bracket_closed.to_string(),0),eof()];
    assert_eq!(expected,tokens);

    let semicolon = ";";
    let tokens = Lexer::tokenize(&semicolon).unwrap();
    let expected = vec![Token::new(TokenType::SeparatorSemiColon,semicolon.to_string(),0),eof()];
    assert_eq!(expected,tokens);

    let comma = ",";
    let tokens = Lexer::tokenize(&comma).unwrap();
    let expected = vec![Token::new(TokenType::SeparatorComma, comma.to_string(), 0),eof()];
    assert_eq!(expected,tokens);

    let colon = " : ";
    let tokens = Lexer::tokenize(colon).unwrap();
    let expected = vec![Token::new(TokenType::SeparatorColon,":".into(),0),eof()];
    assert_eq!(tokens,expected);
}

#[test]
fn tokenizer_numbers_test(){

    let float = "5.3";
    let tokens = Lexer::tokenize(&float).unwrap();
    let expected = vec![Token::new(TokenType::LiteralFloat,float.to_string(),0),eof()];
    assert_eq!(expected,tokens);

    let invalid_float = ".9";
    let tokens = Lexer::tokenize(&invalid_float);
    let dot = Token::new(TokenType::SeparatorDot,".".into(),0);
    let nine = Token::new(TokenType::LiteralInteger,"9".into(),0);
    let expected = Ok(vec![dot,nine,eof()]);
    assert_eq!(expected,tokens);

    let invalid_float2 = "9.";
    let tokens = Lexer::tokenize(&invalid_float2);
    let expected = Ok(vec![
        Token::new(TokenType::LiteralFloat,"9.".into(),0),
        eof()
    ]);
    assert_eq!(expected,tokens);

    let invalid_float3 = "9,0";
    let tokens = Lexer::tokenize(&invalid_float3);
    let t9 = Token::new(TokenType::LiteralInteger,"9".into(),0);
    let t_sep = Token::new(TokenType::SeparatorComma, ",".into(), 0);
    let t0 = Token::new(TokenType::LiteralInteger,"0".into(),0);
    let expected = Ok(vec![t9,t_sep,t0,eof()]);
    assert_eq!(expected,tokens);


    let valid_float2 = "1.23421323";
    let tokens = Lexer::tokenize(&valid_float2);
    let expected = Ok(vec![Token::new(TokenType::LiteralFloat,valid_float2.into(),0),eof()]);
    assert_eq!(expected,tokens);

    let integer = "6";
    let tokens = Lexer::tokenize(&integer);
    let expected = Ok(vec![Token::new(TokenType::LiteralInteger,integer.into(),0),eof()]);
    assert_eq!(expected,tokens);

    let identifier = "b6";
    let tokens = Lexer::tokenize(&identifier);
    let expected = Ok(vec![Token::new(TokenType::Identifier,identifier.into(),0),eof()]);
    assert_eq!(expected,tokens);

    let nan = "6b";
    let tokens = Lexer::tokenize(&nan);
    let expected = Ok(vec![Token::new(TokenType::LiteralInteger,"6b".into(),0),eof()]);
    assert_eq!(expected,tokens);
}

#[test]
fn tokenizer_literal_strings_test(){

    let invalid_string_not_closed = "\"this is an invalid string ";
    let tokens = Lexer::tokenize(&invalid_string_not_closed);
    let expected = Err(LexerError::UnexpectedEndOfString);
    assert_eq!(expected,tokens);

    let new_string = wrap_with_quotes("this is a string");
    let tokens = Lexer::tokenize(&new_string);
    let expected = Ok(vec![Token::new(TokenType::LiteralString, String::from("this is a string"),0),eof()]);
    assert_eq!(expected,tokens);

    let sentence = "this is a penguin emoticon 🐧 \n \n \n \t whdazhwsihawdhasiudhuiawuidh a;;;;;; ,, ++++ ++///";
    let s = wrap_with_quotes(sentence);
    let tokens = Lexer::tokenize(&*s);
    let expected = Ok(vec![Token::new(TokenType::LiteralString, String::from(sentence),0),eof()]);
    assert_eq!(expected,tokens);
}

#[test]
fn tokenizer_function_test(){

    let function_call = "fn onKey(CTRL){\n  exit(); \n}";
    let tokens = Lexer::tokenize(&function_call).unwrap();

    let keyword_fn = Token::new(TokenType::Identifier,"fn".into(),0);
    let function_name = Token::new(TokenType::Identifier,"onKey".into(),0);
    let bracket_open = Token::new(TokenType::SeparatorBracketOpen,"(".into(),0);
    let ctrl = Token::new(TokenType::Identifier,"CTRL".into(),0);
    let bracket_closed = Token::new(TokenType::SeparatorBracketClose,")".into(),0);
    let c_bracket_open = Token::new(TokenType::SeparatorCurvedBracketOpen,"{".into(),0);
    let exit_fn = Token::new(TokenType::Identifier,"exit".into(),0);
    let bracket_open_inner = Token::new(TokenType::SeparatorBracketOpen,"(".into(),0);
    let bracket_closed_inner = Token::new(TokenType::SeparatorBracketClose,")".into(),0);
    let semicolon = Token::new(TokenType::SeparatorSemiColon,";".into(),0);
    let c_bracket_closed = Token::new(TokenType::SeparatorCurvedBracketClosed,"}".into(),0);

    let expected = vec![
        keyword_fn,
        function_name,
        bracket_open,
        ctrl,
        bracket_closed,
        c_bracket_open,
        exit_fn,
        bracket_open_inner,
        bracket_closed_inner,
        semicolon,
        c_bracket_closed,
        eof()
    ];

    assert_eq!(expected,tokens);
}

//TODO add test where Emoticon is in identifier


/// Utility method for wrapping a string with quotes
fn wrap_with_quotes(s: &str) -> String {
    let quote = '"';
    let mut string = quote.to_string();
    string.push_str(s);
    string.push(quote);

    string
}

fn eof() -> Token{
    Token::new(TokenType::EoF,"".into(),0)
}
