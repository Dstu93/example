use frontend::syntax::{DataType,DataValue,token::*,ast::*};

pub struct ASTParser{
    stack: Vec<Token>,
    stream: TokenStream,
}

impl ASTParser{

    pub fn new(stream: TokenStream) -> Self{
        ASTParser{stack: Vec::with_capacity(1), stream}
    }

    // mut is not currently necessary because we do not mutate inner state
    pub fn parse(self) -> Result<AbstractSyntaxTree,ParseError>{

        //vom stream lesen bis }
        //Vec an Tokens an unterfunktion geben zum parsen
        //JoinHandle einsammeln
        //alle JoinHandle abwarten

        // we split the TokenStream in parts, in functions as small units
        // se we can parse them multiple threads
        let fn_unit = ASTParser::separate_function(stream);

        println!("fn unit: {:#?}",fn_unit);

        Err(ParseError::Unknown)
    }

    fn separate_function(stream: TokenStream) -> Result<Vec<Token>,ParseError> {
        let mut ob = 0;
        //number of open brackets {
        let mut cb = 0;
        //number of closed brackets }
        let mut fn_unit = Vec::with_capacity(20);
        loop {
            match stream.next() {
                Some(t) => {
                    if t.kind() == TokenType::SeparatorCurvedBracketOpen { ob += 1; } else if t.kind() == TokenType::SeparatorCurvedBracketClosed { cb += 1; }
                    fn_unit.push(t);
                    // if we closed a scope {} we break
                    if p_balanced(ob, cb) { break; }
                },
                None => {
                    //check if parenthesis are balanced
                    if !p_balanced(ob, cb) {
                        let kind = if ob > cb {
                            TokenType::SeparatorCurvedBracketClosed
                        } else {
                            TokenType::SeparatorCurvedBracketOpen
                        };
                        return Err(ParseError::Missing(kind));
                    }
                    break;
                }
            };
        }
        Ok(fn_unit)
    }
}

/// checks if parenthesis are balanced
fn p_balanced(ob: usize, cb: usize) -> bool{
    ob > 0 && cb > 0 && ob == cb
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