use crate::frontend::syntax::token::{TokenStream, Token, TokenType};
use crate::frontend::syntax::ast::{AbstractSyntaxTree, Expression, VariableBinding, Statement, Block, StatementKind, ExpressionKind};
use crate::frontend::parser::token_pattern::ParseError;
use crate::frontend::syntax::DataType;

const TOKEN_STACK_SIZE: usize = 3;

pub struct ASTParser{
    stack: Vec<Token>,
    stream: TokenStream,
}

impl ASTParser {

    pub fn new(stream: TokenStream) -> Self {
        ASTParser {
            stack: Vec::with_capacity(TOKEN_STACK_SIZE),
            stream,
        }
    }

    pub fn parse(mut self) -> Result<AbstractSyntaxTree, ParseError> {
        self.init_stack();

        let mut statements = Vec::new();
        loop {
            if self.lookup_next().kind() == TokenType::EoF {
                break;
            }
            let function = self.parse_fn()?;
            statements.push(function);
        }

        let ast = AbstractSyntaxTree::new(statements);
        Ok(ast)
    }

    /// we initialise our stack with the next 3 tokens
    fn init_stack(&mut self){
        for _ in 0..TOKEN_STACK_SIZE {
            match self.stream.next() {
                None => {break;},
                Some(t) => { self.stack.push(t); },
            };
        }
    }

    /// Returns next token from the stack, panics if read after EOF
    fn next(&mut self) -> Token {
        match self.stream.next() {
            None => {},
            Some(token) => {
                self.stack.push(token);
            },
        };

        self.stack.pop().expect("called next after EOF")
    }

    /// lookahead for the next token on the stack.
    /// panics if look after EOF
    fn lookup_next(&mut self) -> &Token {
        self.stack.last().expect("called lookup_next after EOF")
    }

    /// parses a single function to an Statement
    fn parse_fn(&mut self) -> Result<Statement,ParseError> {
        let token = self.next();
        if token.kind() != TokenType::Fn {
            return Err(ParseError::WrongToken(token,vec![TokenType::Fn]));
        }

        if self.lookup_next().kind() != TokenType::Identifier {
            return Err(ParseError::WrongToken(self.next(),vec![TokenType::Identifier]));
        }

        let fn_name = self.next().move_value();
        //expecting parenthesis
        if self.lookup_next().kind() != TokenType::SeparatorBracketOpen {
            return Err(ParseError::WrongToken(self.next(), vec![TokenType::SeparatorBracketOpen]));
        }
        let args = self.parse_arg_list()?;

        if self.lookup_next().kind() != TokenType::SeparatorCurvedBracketOpen {
            return Err(ParseError::WrongToken(self.next(), vec![TokenType::SeparatorCurvedBracketOpen]));
        }

        let return_type = self.parse_return_type()?;

        //parsing the function body
        let block = self.parse_block_stmt()?;
        let opt_args = if args.is_empty() { None} else { Some(args) };
        let fn_stmt_kind = ExpressionKind::FnDecl(fn_name,block,opt_args,return_type);
        let fn_stmt_expr = Expression::new(fn_stmt_kind);
        let fn_stmt = Statement::new(StatementKind::Expression(fn_stmt_expr));
        Ok(fn_stmt)
    }

    /// reads from the Tokenstream to read the argument list from a function signature
    fn parse_arg_list(&mut self) -> Result<Vec<VariableBinding>,ParseError>{
        let mut args = Vec::new();
        while self.lookup_next().kind() != TokenType::SeparatorBracketClose {
            let arg = self.parse_argument()?;
            args.push(arg);
            let next = self.lookup_next().kind();
            if next != TokenType::SeparatorSemiColon || next != TokenType::SeparatorBracketClose {
                return Err(ParseError::WrongToken(self.next(), vec![TokenType::SeparatorSemiColon]));
            }
        }
        Ok(args)
    }

    /// Parse the next Expression from the TokenStream
    fn parse_expression(&mut self) -> Result<Expression,ParseError> {
        unimplemented!("not implemented right now!");
    }

    fn parse_argument(&mut self) -> Result<VariableBinding,ParseError> {
        //identifier : DataType
        let symbol = match self.lookup_next().kind(){
            TokenType::Identifier => self.next(),
            _ => return Err(ParseError::WrongToken(self.next(),vec![TokenType::Identifier]))
        };
        let colon = self.next();
        if colon.kind() != TokenType::SeparatorColon {
            return Err(ParseError::WrongToken(colon,vec![TokenType::SeparatorColon]));
        }
        let datatype = self.parse_datatype()?;
        Ok(VariableBinding::new(datatype,symbol.move_value()))
    }

    fn parse_block_stmt(&mut self) -> Result<Block,ParseError>{
        let token = self.next();
        match token.kind() {
            TokenType::SeparatorCurvedBracketOpen => {},
            _ => {return Err(ParseError::WrongToken(token,vec![TokenType::SeparatorCurvedBracketOpen]))}
        }
        let mut stmts = Vec::with_capacity(20);
        while self.lookup_next().kind() != TokenType::SeparatorCurvedBracketClosed {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        }

        Ok(Block::new(stmts))
    }

    fn parse_return_type(&mut self) -> Result<Option<DataType>,ParseError>{
        if self.lookup_next().kind() == TokenType::SeparatorColon {
            drop(self.next()); //drop the : because we dont need it
            let datatype = self.parse_datatype()?;
            return Ok(Some(datatype));
        } else if self.lookup_next().kind() != TokenType::SeparatorCurvedBracketOpen {
            return Err(ParseError::WrongToken(self.next(),vec![TokenType::SeparatorColon,TokenType::SeparatorCurvedBracketOpen]));
        }
        Ok(Option::None)
    }

    fn parse_datatype(&mut self) -> Result<DataType,ParseError> {
        let datatype_token = self.next();
        let datatype = match datatype_token.kind() {
            TokenType::Boolean => { DataType::Boolean },
            TokenType::Integer => { DataType::Integer },
            TokenType::Float => { DataType::Float },
            TokenType::String => { DataType::String },
            _ => { return Err(ParseError::WrongToken(datatype_token, vec![TokenType::String, TokenType::Float,TokenType::Boolean, TokenType::Integer])) }
        };
        Ok(datatype)
    }

    fn parse_stmt(&mut self) -> Result<Statement,ParseError>{
        unimplemented!("statement parsing is not implemented yet");
    }

}
