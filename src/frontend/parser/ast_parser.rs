use std::collections::VecDeque;

use crate::frontend::parser::token_pattern::ParseError;
use crate::frontend::syntax::ast::{AbstractSyntaxTree, Block, Expression, Statement, VariableBinding, BinOp, UnOp};
use crate::frontend::syntax::{DataType, DataValue};
use crate::frontend::syntax::token::{Token, TokenStream, TokenType};

const TOKEN_STACK_SIZE: usize = 3;

pub struct ASTParser{
    queue: VecDeque<Token>,
    stream: TokenStream,
}

impl ASTParser {

    pub fn new(stream: TokenStream) -> Self {
        ASTParser {
            queue: VecDeque::with_capacity(TOKEN_STACK_SIZE),
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
                Some(t) => { self.queue.push_back(t); },
            };
        }
    }

    /// Returns next token from the stack, panics if read after EOF
    fn next(&mut self) -> Token {
        match self.stream.next() {
            None => {},
            Some(token) => {
                self.queue.push_back(token);
            },
        };

        self.queue.pop_front().expect("called next after EOF")
    }

    /// lookahead for the next token on the stack.
    /// panics if look after EOF
    fn lookup_next(&mut self) -> &Token {
        self.queue.front().expect("called lookup_next after EOF")
    }

    fn match_next(&mut self,token_kind: TokenType) -> bool {
        self.lookup_next().kind() == token_kind
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

        let return_type = self.parse_return_type()?;

        //parsing the function body
        let block = self.parse_block_stmt()?;
        let opt_args = if args.is_empty() { None} else { Some(args) };
        let fn_stmt = Statement::FnDecl(fn_name, block, opt_args, return_type);
        Ok(fn_stmt)
    }

    /// reads from the Tokenstream to read the argument list from a function signature
    fn parse_arg_list(&mut self) -> Result<Vec<VariableBinding>,ParseError>{
        let next = self.next();
        if next.kind() != TokenType::SeparatorBracketOpen {
            return Err(ParseError::WrongToken(next,vec![TokenType::SeparatorBracketOpen]))
        }
        let mut args = Vec::new();
        while self.lookup_next().kind() != TokenType::SeparatorBracketClose {
            let arg = self.parse_argument()?;
            args.push(arg);
            let next = self.lookup_next().kind();
            match next {
                TokenType::SeparatorComma => {
                    //consume , and parse next Argument
                    self.consume_next_token();
                    continue;
                },
                TokenType::SeparatorBracketClose => {break;}
                _=> {return Err(ParseError::WrongToken(self.next(), vec![TokenType::SeparatorComma,TokenType::SeparatorBracketClose]));}
            };
        }
        self.expect_nxt_and_consume(TokenType::SeparatorBracketClose)?;
        Ok(args)
    }

    /// consume the next token from the tokenstream and drop it
    fn consume_next_token(&mut self) {
        drop(self.next());
    }

    /// look up the next token and check if it is of expected type.
    /// returns a ParseError::WrongToken if the types dont match
    fn expect_nxt(&mut self, expect: TokenType) -> Result<(),ParseError> {
        if self.lookup_next().kind() != expect {
            return Err(ParseError::WrongToken(self.next(),vec![expect]));
        }
        Ok(())
    }

    /// Validates the next Token and consumes it if tokentype matches
    fn expect_nxt_and_consume(&mut self, expect: TokenType) -> Result<(),ParseError> {
        self.expect_nxt(expect)?;
        self.consume_next_token();
        Ok(())
    }

    /// Parse the next Expression from the TokenStream
    fn parse_expression(&mut self) -> Result<Expression,ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression,ParseError> {
        let expr = self.or()?;
        if self.match_next(TokenType::Assign) {
            self.consume_next_token(); //Consume the Assigment
            let value_expr = self.assignment()?;
            if let Expression::Symbol(var) = expr {
                return Ok(Expression::Assignment(var, Box::from(value_expr)));
            }else {
                return Err(ParseError::GrammarMistake("Expected symbol left on the assignment"));
            }
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expression,ParseError>{
        let expr = self.and()?;
        if self.match_next(TokenType::Or) {
            self.consume_next_token();
            let right_expr = self.and()?;
            let or = Expression::BinaryOp(Box::from(expr),BinOp::Or, Box::from(right_expr));
            return Ok(or);
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expression,ParseError> {
        let expr = self.equality()?;
        if self.match_next(TokenType::And) {
            self.consume_next_token();
            let right_expr = self.equality()?;
            let and_expr = Expression::BinaryOp( Box::from(expr),BinOp::And, Box::from(right_expr));
            return Ok(and_expr);
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression,ParseError> {
        let expr = self.comparison()?;
        let not_equal = self.match_next(TokenType::OperatorNotEqual);
        let equal = self.match_next(TokenType::OperatorEqual);
        if not_equal || equal {
            self.consume_next_token();
            let op = if equal { BinOp::Eq } else { BinOp::Neq };
            let right = self.comparison()?;
            return Ok(Expression::BinaryOp(Box::from(expr),op, Box::from(right)));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression,ParseError> {
        let expr = self.addition()?;

        let greater = self.match_next(TokenType::OperatorGreaterThen);
        let greater_equal = self.match_next(TokenType::OperatorGreaterOrEqual);
        let less = self.match_next(TokenType::OperatorLessThen);
        let less_equal = self.match_next(TokenType::OperatorLessOrEqual);

        if greater {
            self.consume_next_token();
            return Ok(Expression::BinaryOp(Box::from(expr), BinOp::Gt, Box::from(self.addition()?)))
        } else if greater_equal {
            self.consume_next_token();
            return Ok(Expression::BinaryOp(Box::from(expr), BinOp::Ge, Box::from(self.addition()?)))
        } else if less {
            self.consume_next_token();
            return Ok(Expression::BinaryOp(Box::from(expr), BinOp::Lt, Box::from(self.addition()?)))
        } else if less_equal {
            self.consume_next_token();
            return Ok(Expression::BinaryOp(Box::from(expr), BinOp::Le, Box::from(self.addition()?)))
        }

        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expression,ParseError> {
        let expr = self.multiplication()?;
        let addition = self.match_next(TokenType::OperatorPlus);
        let subtraction = self.match_next(TokenType::OperatorMinus);
        if addition || subtraction {
            self.consume_next_token();
            let op = if addition { BinOp::Plus } else { BinOp::Minus };
            let right = self.multiplication()?;
            return Ok(Expression::BinaryOp(Box::from(expr),op, Box::from(right)));
        }
        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expression,ParseError> {
        let expr = self.unary()?;
        let multiplication = self.match_next(TokenType::OperatorMultiplication);
        let divide = self.match_next(TokenType::OperatorDivide);
        if multiplication || divide {
            self.consume_next_token();
            let op = if multiplication {BinOp::Multi}else{BinOp::Divide};
            let right = self.unary()?;
            return Ok(Expression::BinaryOp(Box::from(expr), op, Box::from(right)));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression,ParseError> {
        let is_negation = self.match_next(TokenType::OperatorNegation);
        let is_negative = self.match_next(TokenType::OperatorMinus);
        if is_negation || is_negative {
            self.consume_next_token();
            let op = if is_negation {UnOp::Negation } else { UnOp::Minus };
            let right = self.unary()?;
            return Ok(Expression::UnaryOp(op, Box::from(right)));
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expression,ParseError> {
        //if '(' comes after this token this value string is needed as function name
        let expr = self.atom()?;

        if self.match_next(TokenType::SeparatorBracketOpen) {
            self.consume_next_token(); //Consume the opening (
            let name = match expr {
              Expression::Symbol(name) => name,
                _ => return Err(ParseError::GrammarMistake("invalid function Name"))
            };
            let mut arguments = Vec::new();
            while !self.match_next(TokenType::SeparatorBracketClose){
                let expr = self.parse_expression()?;
                arguments.push(expr);
                if self.match_next(TokenType::SeparatorComma) {
                    self.consume_next_token();
                    continue;
                }
            }
            //consume the closing of the function call
            self.expect_nxt_and_consume(TokenType::SeparatorBracketClose)?;
            return Ok(Expression::FnCall(name, arguments));
        }

        Ok(expr)
    }

    fn atom(&mut self) -> Result<Expression,ParseError> {
        let token = self.next();
        match token.kind() {
            TokenType::BooleanTrue => Ok(Expression::Literal(DataValue::Boolean(true))),
            TokenType::BooleanFalse => Ok(Expression::Literal(DataValue::Boolean(false))),
            TokenType::Identifier => Ok(Expression::Symbol(token.move_value())),
            TokenType::LiteralInteger => Ok(Expression::Literal(DataValue::Integer(token.move_value()))),
            TokenType::LiteralFloat => Ok(Expression::Literal(DataValue::Float(token.move_value()))),
            TokenType::LiteralString => Ok(Expression::Literal(DataValue::String(token.move_value()))),
            TokenType::SeparatorBracketOpen => {
                let expr = self.parse_expression()?;
                self.expect_nxt_and_consume(TokenType::SeparatorBracketClose)?;
                Ok(expr)
            },
            _ => Err(ParseError::GrammarMistake("Expected literal or identifier"))
        }
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
        self.expect_nxt_and_consume(TokenType::SeparatorCurvedBracketClosed)?;
        Ok(Block::new(stmts))
    }

    fn parse_return_type(&mut self) -> Result<Option<DataType>,ParseError>{
        if self.lookup_next().kind() == TokenType::SeparatorColon {
            self.consume_next_token(); //drop the : because we dont need it
            let datatype = self.parse_datatype()?;
            return Ok(Some(datatype));
            //if we dont find a return type it must follow the function body/ block main(){}
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

    fn parse_stmt(&mut self) -> Result<Statement,ParseError> {
        //possible statements
        // fnCall identifer.identifier ( expression )
        // if
        // let declaration
        // break
        // continue
        // return
        // while
        // loop

        let next_token = self.lookup_next().kind();
        let stmt = match next_token {
            TokenType::Let => {self.parse_let_stmt()? },
            TokenType::For => {unimplemented!("For keyword is not supported yet")}, //TODO for loop does not exists at the moment
            TokenType::Loop => {self.parse_loop()?},
            TokenType::Break => {self.parse_break_stmt()?},
            TokenType::Continue => {self.parse_continue_stmt()?},
            TokenType::Return => {self.parse_return_stmt()?},
            TokenType::While => {self.parse_while_stmt()?},
            TokenType::If => { self.parse_if()? },
            TokenType::Identifier => {self.parse_expression_stmt()?},
            _ => {return Err(ParseError::WrongToken(self.next(),vec![
                TokenType::If,
                TokenType::For,
                TokenType::Let,
                TokenType::Loop,
                TokenType::Break,
                TokenType::Return,
                TokenType::While
            ]))}
        };

        Ok(stmt)
    }

    fn parse_expression_stmt(&mut self) -> Result<Statement,ParseError> {
        let expr = self.parse_expression()?;
        self.expect_nxt_and_consume(TokenType::SeparatorSemiColon)?;
        Ok(Statement::Expression(expr))
    }

    fn parse_if(&mut self) -> Result<Statement,ParseError> {
        self.expect_nxt_and_consume(TokenType::If)?;
        let condition_expr = Box::new(self.parse_expression()?);
        self.expect_nxt(TokenType::SeparatorCurvedBracketOpen)?;
        let if_block = self.parse_block_stmt()?;
        let else_block = match self.lookup_next().kind() {
            TokenType::Else => {
                //consume the Else Token and parse the else block
                self.consume_next_token();
                Some(self.parse_block_stmt()?)
            }
            _ => {None}
        };
        let if_stmt = Statement::If(condition_expr, if_block, else_block);
        Ok(if_stmt)
    }

    fn parse_let_stmt(&mut self) -> Result<Statement,ParseError> {
        self.expect_nxt_and_consume(TokenType::Let)?;
        self.expect_nxt(TokenType::Identifier)?;
        let variable_name = self.next().move_value();
        self.expect_nxt_and_consume(TokenType::SeparatorColon)?;
        let variable_type = self.parse_datatype()?;
        self.expect_nxt_and_consume(TokenType::Assign)?;
        let expr = self.parse_expression()?;
        let binding = VariableBinding::new(variable_type,variable_name);
        self.expect_nxt_and_consume(TokenType::SeparatorSemiColon)?;
        Ok(Statement::Declaration(binding, expr))
    }

    fn parse_loop(&mut self) -> Result<Statement,ParseError> {
        self.expect_nxt(TokenType::Loop)?;
        self.consume_next_token(); //consume the loop token
        self.expect_nxt(TokenType::SeparatorCurvedBracketOpen)?;
        let loop_block = self.parse_block_stmt()?;
        let loop_stmt = Statement::Loop(loop_block);
        Ok(loop_stmt)
    }

    fn parse_break_stmt(&mut self) -> Result<Statement,ParseError> {
        self.expect_nxt_and_consume(TokenType::Break)?;
        let break_stmt = Statement::Break;
        self.expect_nxt_and_consume(TokenType::SeparatorSemiColon)?;
        Ok(break_stmt)
    }

    fn parse_continue_stmt(&mut self) -> Result<Statement,ParseError> {
        self.expect_nxt_and_consume(TokenType::Continue)?;
        self.expect_nxt_and_consume(TokenType::SeparatorSemiColon)?;
        Ok(Statement::Continue)
    }

    fn parse_while_stmt(&mut self) -> Result<Statement,ParseError> {
        self.expect_nxt_and_consume(TokenType::While)?;
        let while_condition = self.parse_expression()?;
        let while_block = self.parse_block_stmt()?;
        let while_stmt = Statement::WhileLoop(Box::from(while_condition), while_block);
        Ok(while_stmt)
    }

    fn parse_return_stmt(&mut self) -> Result<Statement,ParseError> {
        self.expect_nxt(TokenType::Return)?;
        self.consume_next_token();
        let expr = match self.lookup_next().kind() {
            TokenType::SeparatorSemiColon => {
                self.consume_next_token();
                None
            }
            _ => {Some(Box::new(self.parse_expression()?))}
        };
        let return_stmt = Statement::Return(expr);
        self.expect_nxt_and_consume(TokenType::SeparatorSemiColon)?;
        Ok(return_stmt)
    }

}
