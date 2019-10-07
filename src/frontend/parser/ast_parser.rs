use crate::frontend::syntax::token::{TokenStream, Token, TokenType};
use crate::frontend::syntax::ast::{AbstractSyntaxTree, Expression, VariableBinding, Statement, NodeId};
use crate::frontend::parser::token_pattern::ParseError;

pub struct ASTParser{
    stack: Vec<Token>,
    stream: TokenStream,
    current_node_id: NodeId,
}

impl ASTParser {

    pub fn new(stream: TokenStream) -> Self {
        ASTParser {
            stack: Vec::with_capacity(1),
            stream,
            current_node_id: 0.into()
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
        let mut counter = 0;
        while counter < 3 {
            match self.stream.next() {
                None => {
                    break;
                },
                Some(t) => {
                    self.stack.push(t);
                },
            };
            counter += 1;
        };
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
        if token.kind() != TokenType::Identifier {
            return Err(ParseError::WrongToken(token,TokenType::Identifier));
        }

        let fn_name = token.move_value();
        //expecting parenthesis
        if self.lookup_next().kind() != TokenType::SeparatorBracketOpen {
            return Err(ParseError::WrongToken(self.next(), TokenType::SeparatorBracketOpen));
        }

        let mut args = Vec::new();
        while self.lookup_next().kind() != TokenType::SeparatorBracketClose {
            //TODO
            //FIXME epected identifier PunctationColon DataType
            let arg = self.parse_argument()?;
            match arg {
                None => {},
                Some(variable_binding) => {
                    args.push(variable_binding);
                },
            };
        }

        unimplemented!("not implemented right now!");
    }

    /// Parse the next Expression from the TokenStream
    fn parse_expression(&mut self) -> Result<Expression,ParseError> {
        unimplemented!("not implemented right now!");
    }

    fn parse_argument(&mut self) -> Result<Option<VariableBinding>,ParseError> {
        unimplemented!("not implemented right now");
    }
}
