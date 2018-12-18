
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
    let binding = VariableBinding::new(NodeId::new(5),DataType::Boolean,SymbolId::new(0));
    let boolean_expression = Expression::new(NodeId::new(6),ExpressionKind::Literal(DataValue::Boolean(false)));
    let a_declaration = Statement::new(NodeId::new(4),StatementKind::Declaration(binding,boolean_expression));

    let a = Expression::new(NodeId::new(10),ExpressionKind::Symbol(SymbolId::new(0)));
    let if_condition = Expression::new(NodeId::new(9),ExpressionKind::UnaryOp(UnOp::Negation,Box::new(a)));
    let message = Expression::new(NodeId::new(14),ExpressionKind::Literal(DataValue::String("a is false".into())));
    let args = vec![Argument::new(DataType::String,message)];
    let print_call = Expression::new(NodeId::new(13),ExpressionKind::FnCall("print".into(),Some(args)));
    let print_stmt = Statement::new(NodeId::new(12),StatementKind::Expression(print_call));
    let block = Block::new(NodeId::new(11),vec![print_stmt]);
    let if_stmt_kind = StatementKind::Expression(Expression::new(NodeId::new(8),ExpressionKind::If(Box::new(if_condition),block,None)));
    let if_expr = Statement::new(NodeId::new(7),if_stmt_kind);

    let block = Block::new(NodeId::new(3),Vec::new());
    let main_expression = ExpressionKind::FnDecl("main".into(),block,None,None);
    let main_function = Statement::new(NodeId::new(1),StatementKind::Expression(Expression::new(NodeId::new(2),main_expression)));

    let expected_ast = AbstractSyntaxTree::new(vec![main_function]);
    assert_eq!(expected_ast,ast);
}

#[test]
fn function_with_return_type_test(){
    let src = String::from("\
    fn test() : string{\
        return \"a b c d e f g\";\
    }");

    let (ts,handle) = Lexer::tokenize(src.clone());
    let ast = ASTParser::new(ts).parse().expect("expected abstract syntax tree");


    let block = Block::new(NodeId::new(2),vec![]);
    let test_fn_decl = StatementKind::Expression(Expression::new(NodeId::new(1),ExpressionKind::FnDecl("test".into(),block,None,Some(DataType::String))));
    let test_fn = Statement::new(NodeId::new(0),test_fn_decl);

    let expected_ast = AbstractSyntaxTree::new(vec![test_fn]);
    assert_eq!(expected_ast,ast, "We are comparing two ast build from this source: {}",src);
}