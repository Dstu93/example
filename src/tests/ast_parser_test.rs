
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

    let return_str = Expression::new(NodeId::new(6),ExpressionKind::Literal(DataValue::String("a b c d e f g".into())));
    let return_expr = Expression::new(NodeId::new(5),ExpressionKind::Return(Some(Box::new(return_str))));
    let return_stmt = Statement::new(NodeId::new(4),StatementKind::Expression(return_expr));
    let block = Block::new(NodeId::new(3),vec![return_stmt]);
    let test_fn_decl = StatementKind::Expression(Expression::new(NodeId::new(2),ExpressionKind::FnDecl("test".into(),block,None,Some(DataType::String))));
    let test_fn = Statement::new(NodeId::new(1),test_fn_decl);

    let expected_ast = AbstractSyntaxTree::new(vec![test_fn]);
    assert_eq!(expected_ast,ast, "We are comparing two ast build from this source: {}",src);
}

//FIXME Return(Option<Box<Expression>>) && Statement(Expression) without NodeId

#[test]
fn function_with_arguments() {
    let src = String::from("\
    fn calculate(x: int, y: int): int {\
        let solution: integer = x * y;\
        return solution;\
    }");
    println!("src: {}",&src);
    let (ts,handle) = Lexer::tokenize(src);
    let ast = ASTParser::new(ts).parse().expect("Expected Abstract Syntax Tree");

    let return_value = Expression::new(11.into(),ExpressionKind::Symbol(2.into()));
    let return_expression = Expression::new(10.into(),ExpressionKind::Return(Some(Box::new(return_value))));
    let return_statement = Statement::new(9.into(),StatementKind::Expression(return_expression));
    let y = Expression::new(8.into(),ExpressionKind::Symbol(1.into()));
    let x = Expression::new(7.into(),ExpressionKind::Symbol(0.into()));
    let multiplication = Expression::new(6.into(),ExpressionKind::BinaryOp(BinOp::Multi,Box::new(x),Box::new(y)));
    let solution_binding = VariableBinding::new(4.into(),DataType::Integer,2.into());
    let assignment = Statement::new(4.into(),StatementKind::Declaration(solution_binding,multiplication));
    let args = vec![DataType::Integer,DataType::Integer];
    let test_fn_block = Block::new(3.into(),vec![assignment,return_statement]);
    let fn_declaration = Expression::new(2.into(),ExpressionKind::FnDecl("calculate".into(),test_fn_block,Some(args),Some(DataType::Integer)));
    let test_fn = Statement::new(1.into(),StatementKind::Expression(fn_declaration));

    let expected_ast = AbstractSyntaxTree::new(vec![test_fn]);
    assert_eq!(ast,expected_ast);
}