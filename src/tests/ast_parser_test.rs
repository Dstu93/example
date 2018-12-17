
use frontend::syntax::{*,ast::*,token::*};
use frontend::lexer::*;
use frontend::parser::ast_parser::*;

#[test]
fn fun_main_function(){

    let src = String::from("\
    fn main(){\
       let a : boolean = false;\
       if !a {\
          print(\"a is false\");\
       }\
    }");


    let (ts,handle) = Lexer::tokenize(src);
    let parser = ASTParser::new(ts);
    let ast= parser.parse().expect("expect abstract syntax tree");

    // a is the first and only symbol, so a is replaced with a 0 as SymbolId
    let binding = VariableBinding::new(NodeId::new(4),DataType::Boolean,SymbolId::new(0));
    let boolean_expression = Expression::new(NodeId::new(5),ExpressionKind::Literal(DataValue::Boolean(false)));
    let a_declaration = Statement::new(NodeId::new(3),StatementKind::Declaration(binding,boolean_expression));

    let a = Expression::new(NodeId::new(9),ExpressionKind::Symbol(SymbolId::new(0)));
    let if_condition = Expression::new(NodeId::new(8),ExpressionKind::UnaryOp(UnOp::Negation,Box::new(a)));
    let message = Expression::new(NodeId::new(13),ExpressionKind::Literal(DataValue::String("a is false".into())));
    let args = vec![Argument::new(DataType::String,message)];
    let print_call = Expression::new(NodeId::new(12),ExpressionKind::FnCall("print".into(),Some(args)));
    let print_stmt = Statement::new(NodeId::new(11),StatementKind::Expression(print_call));
    let block = Block::new(NodeId::new(10),vec![print_stmt]);
    let if_stmt_kind = StatementKind::Expression(Expression::new(NodeId::new(7),ExpressionKind::If(Box::new(if_condition),block,None)));
    let if_expr = Statement::new(NodeId::new(6),if_stmt_kind);

    let block = Block::new(NodeId::new(2),Vec::new());
    let main_expression = ExpressionKind::FnDecl("main".into(),block,None,None);
    let main_function = Statement::new(NodeId::new(0),StatementKind::Expression(Expression::new(NodeId::new(1),main_expression)));

    let expected_ast = AbstractSyntaxTree::new(vec![main_function]);
    assert_eq!(expected_ast,ast);
}