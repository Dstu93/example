use frontend::syntax::{DataType,DataValue,token::*,ast::*};

pub struct ASTParser;

impl ASTParser{

    pub fn new() -> Self{
        ASTParser{}
    }

    // mut is not currently necessary because we do not mutate inner state
    pub fn parse(self, stream: TokenStream) -> Result<AbstractSyntaxTree,ParseError>{

        //vom stream lesen bis }
        //Vec an Tokens an unterfunktion geben zum parsen
        //JoinHandle einsammeln
        //alle JoinHandle abwarten

        // we split the TokenStream in parts, in functions as small units
        // se we can parse them multiple threads
        let mut ob = 0; //number of open brackets {
        let mut cb = 0; //number of closed brackets }

        let mut fn_unit = Vec::with_capacity(20);
        loop {
            match stream.next() {
                Some(t) => {
                    if t.kind() == TokenType::SeparatorCurvedBracketOpen { ob += 1; }
                    else if t.kind() == TokenType::SeparatorCurvedBracketClosed{ cb += 1; }
                    fn_unit.push(t);
                    // if we closed a scope {} we break
                    if ob > 0 && cb > 0 && ob == cb { break; }
                },
                None => break
            };
        }
        println!("fn unit: {:#?}",fn_unit);

        Err(ParseError::Unknown)
    }

}


#[derive(Eq, PartialEq,Debug,Hash,Clone)]
pub enum ParseError{
    Unknown,
    /// This Number token cant get parsed to a number, its invalid
    NaN(Token),
    /// Found, Expected
    WrongToken(Token,Token),
    /// This Token is not allowed outside of a function
    OutOfFnScope(Token),
}