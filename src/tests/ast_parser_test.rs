use crate::frontend::lexer::Lexer;
use crate::frontend::parser::ast_parser::ASTParser;
use crate::frontend::syntax::{DataType, DataValue};
use crate::frontend::syntax::ast::*;

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

    // a is the first and only symbol
    let binding = VariableBinding::new(DataType::Boolean,"a".into());
    let boolean_expression = Expression::new(ExpressionKind::Literal(DataValue::Boolean(false)));
    let a_declaration = Statement::new(StatementKind::Declaration(binding,boolean_expression));

    let a = Expression::new(ExpressionKind::Symbol("a".into()));
    let if_condition = Expression::new(ExpressionKind::UnaryOp(UnOp::Negation,Box::new(a)));
    let message = Expression::new(ExpressionKind::Literal(DataValue::String("a is false".into())));
    let args = vec![Argument::new(DataType::String,message)];
    let print_call = Expression::new(ExpressionKind::FnCall("print".into(),Some(args)));
    let print_stmt = Statement::new(StatementKind::Expression(print_call));
    let block = Block::new(vec![print_stmt]);
    let if_stmt_kind = StatementKind::Expression(Expression::new(ExpressionKind::If(Box::new(if_condition),block,None)));
    let if_expr = Statement::new(if_stmt_kind);

    let block = Block::new(Vec::new());
    let main_expression = ExpressionKind::FnDecl("main".into(),block,None,None);
    let main_function = Statement::new(StatementKind::Expression(Expression::new(main_expression)));

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

    let return_str = Expression::new(ExpressionKind::Literal(DataValue::String("a b c d e f g".into())));
    let return_expr = Expression::new(ExpressionKind::Return(Some(Box::new(return_str))));
    let return_stmt = Statement::new(StatementKind::Expression(return_expr));
    let block = Block::new(vec![return_stmt]);
    let test_fn_decl = StatementKind::Expression(Expression::new(ExpressionKind::FnDecl("test".into(),block,None,Some(DataType::String))));
    let test_fn = Statement::new(test_fn_decl);

    let expected_ast = AbstractSyntaxTree::new(vec![test_fn]);
    assert_eq!(expected_ast,ast, "We are comparing two ast build from this source: {}",src);
}


#[test]
fn function_with_arguments() {
    let src = String::from("\
    fn calculate(x: int, y: int): int {\
        let solution: int = x * y;\
        return solution;\
    }");
    println!("src: {}",&src);
    let (ts,handle) = Lexer::tokenize(src);
    let ast = ASTParser::new(ts).parse().expect("Expected Abstract Syntax Tree");

    let return_value = Expression::new(ExpressionKind::Symbol("solution".into()));
    let return_expression = Expression::new(ExpressionKind::Return(Some(Box::new(return_value))));
    let return_statement = Statement::new(StatementKind::Expression(return_expression));
    let y = Expression::new(ExpressionKind::Symbol("y".into()));
    let x = Expression::new(ExpressionKind::Symbol("x".into()));
    let multiplication = Expression::new(ExpressionKind::BinaryOp(BinOp::Multi,Box::new(x),Box::new(y)));
    let solution_binding = VariableBinding::new(DataType::Integer,"solution".into());
    let assignment = Statement::new(StatementKind::Declaration(solution_binding,multiplication));

    //Function declaration
    let args = vec![
        VariableBinding::new(DataType::Integer, "x".into()),
        VariableBinding::new(DataType::Integer,"y".into())
    ];
    let test_fn_block = Block::new(vec![assignment,return_statement]);
    let fn_declaration = Expression::new(ExpressionKind::FnDecl("calculate".into(),test_fn_block,Some(args),Some(DataType::Integer)));
    let test_fn = Statement::new(StatementKind::Expression(fn_declaration));

    let expected_ast = AbstractSyntaxTree::new(vec![test_fn]);
    assert_eq!(ast,expected_ast);
}