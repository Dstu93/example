
use frontend::syntax::{*,ast::*,token::*};
use frontend::lexer::*;
use frontend::parser::ast_parser::*;

#[test]
fn fun_main_function(){

    let src = String::from("\
    fn main(){\
       let a : boolean = false;\
       if !a {\
          print(\"a is false\")\
       }\
    }");


    let (ts,handle) = Lexer::tokenize(src);
    let parser = ASTParser::new(ts);
    let ast= parser.parse().expect("expect abstract syntax tree");

    // a is the first and only symbol, so a is replaced with a 0 as SymbolId
    let binding = VariableBinding::new(NodeId::new(4),DataType::Boolean,SymbolId::new(0));
    let boolean_expression = Expression::new(NodeId::new(5),ExpressionKind::Literal(DataValue::Boolean(false)));
    let a_declaration = Statement::new(NodeId::new(3),StatementKind::Declaration(binding,boolean_expression));

    //TODO if expression  

    let block = Block::new(NodeId::new(2),Vec::new());
    let main_expression = ExpressionKind::FnDecl("main".into(),block,None,None);
    let main_function = Statement::new(NodeId::new(0),StatementKind::Expression(Expression::new(NodeId::new(1),main_expression)));

    let expected_ast = AbstractSyntaxTree::new(vec![main_function]);
}