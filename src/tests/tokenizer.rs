

use frontend::syntax::{*,token::*};
use frontend::lexer::*;

#[test]
fn tokenizer_operator_test(){
    let src = "==";
    let equal = Lexer::tokenize(src.into()).0.collect();
    let expected = vec![Token::new(TokenType::OperatorEqual,"==".into(),0),eof()];
    assert_eq!(equal,expected);

    let src = " =";
    let assign = Lexer::tokenize(src.into()).0.collect();
    let expected = vec![Token::new(TokenType::Assign,"=".into(),0),eof()];
    assert_eq!(expected,assign);

    let src = "+";
    let plus = Lexer::tokenize(src.into()).0.collect();
    let expected = vec![Token::new(TokenType::OperatorPlus,"+".into(),0),eof()];
    assert_eq!(plus,expected);

    let src = "-";
    let minus = Lexer::tokenize(src.into()).0.collect();
    let expected = vec![Token::new(TokenType::OperatorMinus,"-".into(),0),eof()];
    assert_eq!(minus,expected);

    let src = "*";
    let multi = Lexer::tokenize(src.into()).0.collect();
    let expected = vec![Token::new(TokenType::OperatorMultiplication,"*".into(),0),eof()];
    assert_eq!(multi,expected);

    let src = "/";
    let divide = Lexer::tokenize(src.into()).0.collect();
    let expected = vec![Token::new(TokenType::OperatorDivide,"/".into(),0),eof()];
    assert_eq!(divide,expected);

    let src = " <   ";
    let less = Lexer::tokenize(src.into()).0.collect();
    let expected = vec![Token::new(TokenType::OperatorLessThen,"<".into(),0),eof()];
    assert_eq!(less,expected);

    let src = "  > ";
    let greater_then = Lexer::tokenize(src.into()).0.collect();
    let expected = vec![Token::new(TokenType::OperatorGreaterThen,">".into(),0),eof()];
    assert_eq!(greater_then,expected);
}


#[test]
fn tokenizer_separator_test(){

    let curved_bracket_open = "{";
    let tokens = Lexer::tokenize(curved_bracket_open.into()).0.collect();
    let expected = vec![Token::new(TokenType::SeparatorCurvedBracketOpen,curved_bracket_open.to_string(),0),eof()];
    assert_eq!(expected,tokens);

    let curved_bracket_closed = "}";
    let tokens = Lexer::tokenize(curved_bracket_closed.into()).0.collect();
    let expected = vec![Token::new(TokenType::SeparatorCurvedBracketClosed,curved_bracket_closed.to_string(),0),eof()];
    assert_eq!(expected,tokens);

    let bracket_open = "(";
    let tokens = Lexer::tokenize(bracket_open.into()).0.collect();
    let expected = vec![Token::new(TokenType::SeparatorBracketOpen,bracket_open.to_string(),0),eof()];
    assert_eq!(expected,tokens);

    let bracket_closed = ")";
    let tokens = Lexer::tokenize(bracket_closed.into()).0.collect();
    let expected = vec![Token::new(TokenType::SeparatorBracketClose,bracket_closed.to_string(),0),eof()];
    assert_eq!(expected,tokens);

    let semicolon = ";";
    let tokens = Lexer::tokenize(semicolon.into()).0.collect();
    let expected = vec![Token::new(TokenType::SeparatorSemiColon,semicolon.to_string(),0),eof()];
    assert_eq!(expected,tokens);

    let comma = ",";
    let tokens = Lexer::tokenize(comma.into()).0.collect();
    let expected = vec![Token::new(TokenType::SeparatorComma, comma.to_string(), 0),eof()];
    assert_eq!(expected,tokens);

    let colon = " : ";
    let tokens = Lexer::tokenize(colon.into()).0.collect();
    let expected = vec![Token::new(TokenType::SeparatorColon,":".into(),0),eof()];
    assert_eq!(tokens,expected);
}

#[test]
fn tokenizer_numbers_test(){

    let float = "5.3";
    let tokens = Lexer::tokenize(float.into()).0.collect();
    let expected = vec![Token::new(TokenType::LiteralFloat,float.to_string(),0),eof()];
    assert_eq!(expected,tokens);

    let invalid_float = ".9";
    let tokens = Lexer::tokenize(invalid_float.into()).0.collect();
    let dot = Token::new(TokenType::SeparatorDot,".".into(),0);
    let nine = Token::new(TokenType::LiteralInteger,"9".into(),0);
    let expected = vec![dot,nine,eof()];
    assert_eq!(expected,tokens);

    let invalid_float2 = "9.";
    let tokens = Lexer::tokenize(invalid_float2.into()).0.collect();
    let expected = vec![
        Token::new(TokenType::LiteralFloat,"9.".into(),0),
        eof()
    ];
    assert_eq!(expected,tokens);

    let invalid_float3 = "9,0";
    let tokens = Lexer::tokenize(invalid_float3.into()).0.collect();
    let t9 = Token::new(TokenType::LiteralInteger,"9".into(),0);
    let t_sep = Token::new(TokenType::SeparatorComma, ",".into(), 0);
    let t0 = Token::new(TokenType::LiteralInteger,"0".into(),0);
    let expected = vec![t9,t_sep,t0,eof()];
    assert_eq!(expected,tokens);


    let valid_float2 = "1.23421323";
    let tokens = Lexer::tokenize(valid_float2.into()).0.collect();
    let expected = vec![Token::new(TokenType::LiteralFloat,valid_float2.into(),0),eof()];
    assert_eq!(expected,tokens);

    let integer = "6";
    let tokens = Lexer::tokenize(integer.into()).0.collect();
    let expected = vec![Token::new(TokenType::LiteralInteger,integer.into(),0),eof()];
    assert_eq!(expected,tokens);

    let identifier = "b6";
    let tokens = Lexer::tokenize(identifier.into()).0.collect();
    let expected = vec![Token::new(TokenType::Identifier,identifier.into(),0),eof()];
    assert_eq!(expected,tokens);

    let nan = "6b";
    let tokens = Lexer::tokenize(nan.into()).0.collect();
    let expected = vec![Token::new(TokenType::LiteralInteger,"6b".into(),0),eof()];
    assert_eq!(expected,tokens);
}

#[test]
fn tokenizer_literal_strings_test(){

    let invalid_string_not_closed = "\"this is an invalid string ";
    let tokens = Lexer::tokenize(invalid_string_not_closed.into()).1.join().unwrap_or_else(|_| panic!("should never happen"));
    let expected = Err(LexerError::UnexpectedEndOfString);
    assert_eq!(expected,tokens);

    let new_string = wrap_with_quotes("this is a string");
    let tokens = Lexer::tokenize(new_string.into()).0.collect();
    let expected = vec![Token::new(TokenType::LiteralString, String::from("this is a string"),0),eof()];
    assert_eq!(expected,tokens);

    let sentence = "this is a penguin emoticon 🐧 \n \n \n \t whdazhwsihawdhasiudhuiawuidh a;;;;;; ,, ++++ ++///";
    let s = wrap_with_quotes(sentence);
    let tokens = Lexer::tokenize(s).0.collect();
    let expected = vec![Token::new(TokenType::LiteralString, String::from(sentence),0),eof()];
    assert_eq!(expected,tokens);
}

#[test]
fn tokenizer_function_test(){

    let function_call = "fn onKey(CTRL){\n  exit(); \n}";
    let tokens = Lexer::tokenize(function_call.into()).0.collect();

    let keyword_fn = Token::new(TokenType::Fn,"fn".into(),0);
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

    let function = "fn test(){ \
    loop{ \
        let a: boolean = false; \
        if !a {\
           break;\
        } else {\
            continue;\
        } return;\
    } }";
    let tokens = Lexer::tokenize(function.into()).0.collect();

    let expected = vec![
        Token::new(TokenType::Fn,"fn".into(),0),
        Token::new(TokenType::Identifier,"test".into(),0),
        Token::new(TokenType::SeparatorBracketOpen,"(".into(),0),
        Token::new(TokenType::SeparatorBracketClose,")".into(),0),
        Token::new(TokenType::SeparatorCurvedBracketOpen,"{".into(),0),
        Token::new(TokenType::Loop,"loop".into(),0),
        Token::new(TokenType::SeparatorCurvedBracketOpen,"{".into(),0),
        Token::new(TokenType::Let,"let".into(),0),
        Token::new(TokenType::Identifier,"a".into(),0),
        Token::new(TokenType::SeparatorColon,":".into(),0),
        Token::new(TokenType::Boolean,"boolean".into(),0),
        Token::new(TokenType::Assign,"=".into(),0),
        Token::new(TokenType::BooleanFalse,"false".into(),0),
        Token::new(TokenType::SeparatorSemiColon,";".into(),0),
        Token::new(TokenType::If,"if".into(),0),
        Token::new(TokenType::OperatorNegation,"!".into(),0),
        Token::new(TokenType::Identifier,"a".into(),0),
        Token::new(TokenType::SeparatorCurvedBracketOpen,"{".into(),0),
        Token::new(TokenType::Break,"break".into(),0),
        Token::new(TokenType::SeparatorSemiColon,";".into(),0),
        Token::new(TokenType::SeparatorCurvedBracketClosed,"}".into(),0),
        Token::new(TokenType::Else,"else".into(),0),
        Token::new(TokenType::SeparatorCurvedBracketOpen,"{".into(),0),
        Token::new(TokenType::Continue,"continue".into(),0),
        Token::new(TokenType::SeparatorSemiColon,";".into(),0),
        Token::new(TokenType::SeparatorCurvedBracketClosed,"}".into(),0),
        Token::new(TokenType::Return,"return".into(),0),
        Token::new(TokenType::SeparatorSemiColon,";".into(),0),
        Token::new(TokenType::SeparatorCurvedBracketClosed,"}".into(),0),
        Token::new(TokenType::SeparatorCurvedBracketClosed,"}".into(),0),
        eof()
    ];

    assert_eq!(expected,tokens);
}

#[test]
fn identifier_test() {
    let src = "let xðłð@łðæſ = 5;";
    let tokens = Lexer::tokenize(src.into()).1.join().unwrap_or_else(|_| panic!("should never happen"));
    let expected = Err(LexerError::UnknownCharacter('@'));
    assert_eq!(expected,tokens);

    let src = "let ä = 5;";
    let tokens = Lexer::tokenize(src.into()).0.collect();
    let expected = vec![
        Token::new(TokenType::Let,"let".into(),0),
        Token::new(TokenType::Identifier,"ä".into(),0),
        Token::new(TokenType::Assign,"=".into(),0),
        Token::new(TokenType::LiteralInteger,"5".into(),0),
        Token::new(TokenType::SeparatorSemiColon,";".into(),0),
        eof()
    ];
    assert_eq!(expected,tokens);
}

#[test]
fn keywords_test(){
    expect_token("fn",TokenType::Fn);
    expect_token("Fn",TokenType::Identifier);

    expect_token("while",TokenType::While);
    expect_token("While",TokenType::Identifier);

    expect_token("return",TokenType::Return);
    expect_token("Return",TokenType::Identifier);

    expect_token("for",TokenType::For);
    expect_token("For",TokenType::Identifier);

    expect_token("loop",TokenType::Loop);
    expect_token("Loop",TokenType::Identifier);

    expect_token("break",TokenType::Break);
    expect_token("Break",TokenType::Identifier);

    expect_token("continue",TokenType::Continue);
    expect_token("Continue",TokenType::Identifier);

    expect_token("if",TokenType::If);
    expect_token("If",TokenType::Identifier);

    expect_token("else",TokenType::Else);
    expect_token("Else",TokenType::Identifier);

    expect_token("boolean",TokenType::Boolean);
    expect_token("Boolean",TokenType::Identifier);

    expect_token("int",TokenType::Integer);
    expect_token("Int",TokenType::Identifier);

    expect_token("float",TokenType::Float);
    expect_token("Float",TokenType::Identifier);

    expect_token("string",TokenType::String);
    expect_token("String",TokenType::Identifier);

    expect_token("false",TokenType::BooleanFalse);
    expect_token("False",TokenType::Identifier);

    expect_token("true",TokenType::BooleanTrue);
    expect_token("True",TokenType::Identifier);

}

fn expect_token(value: &str,kind: TokenType) {
    let result = Lexer::tokenize(value.into()).0.collect();
    let expected = vec![Token::new(kind, value.into(), 0),eof()];
    assert_eq!(expected, result);
}


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
