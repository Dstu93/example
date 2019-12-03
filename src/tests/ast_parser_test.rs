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


    let (ts,_) = Lexer::tokenize(src);
    let parser = ASTParser::new(ts);
    let ast= parser.parse().expect("expect abstract syntax tree");

    // a is the first and only symbol
    let binding = VariableBinding::new(DataType::Boolean,"a".into());
    let boolean_expression = Expression::Literal(DataValue::Boolean(false));
    let a_declaration = Statement::new(StatementKind::Declaration(binding,boolean_expression));

    let a = Expression::Symbol("a".into());
    let if_condition = Expression::UnaryOp(UnOp::Negation, Box::new(a));
    let message = Expression::Literal(DataValue::String("a is false".into()));
    let args = vec![message];
    let print_call = Expression::FnCall("print".into(), args);
    let print_stmt = Statement::new(StatementKind::Expression(print_call));
    let block = Block::new(vec![print_stmt]);
    let if_stmt_kind = StatementKind::Expression(Expression::If(Box::new(if_condition), block, None));
    let if_expr = Statement::new(if_stmt_kind);

    let block = Block::new(vec![a_declaration,if_expr]);
    let main_expression = Expression::FnDecl("main".into(), block, None, None);
    let main_function = Statement::new(StatementKind::Expression(main_expression));

    let expected_ast = AbstractSyntaxTree::new(vec![main_function]);
    assert_eq!(expected_ast,ast);
}

#[test]
fn function_with_return_type_test(){
    let src = String::from("\
    fn test() : string{\
        return \"a b c d e f g\";\
    }");

    let (ts,_) = Lexer::tokenize(src.clone());
    let ast = ASTParser::new(ts).parse().expect("expected abstract syntax tree");

    let return_str = Expression::Literal(DataValue::String("a b c d e f g".into()));
    let return_expr = Expression::Return(Some(Box::new(return_str)));
    let return_stmt = Statement::new(StatementKind::Expression(return_expr));
    let block = Block::new(vec![return_stmt]);
    let test_fn_decl = StatementKind::Expression(Expression::FnDecl("test".into(), block, None, Some(DataType::String)));
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
    let (ts,_) = Lexer::tokenize(src);
    let ast = ASTParser::new(ts).parse().expect("Expected Abstract Syntax Tree");

    let return_value = Expression::Symbol("solution".into());
    let return_expression = Expression::Return(Some(Box::new(return_value)));
    let return_statement = Statement::new(StatementKind::Expression(return_expression));
    let y = Expression::Symbol("y".into());
    let x = Expression::Symbol("x".into());
    let multiplication = Expression::BinaryOp(Box::new(x), BinOp::Multi, Box::new(y));
    let solution_binding = VariableBinding::new(DataType::Integer,"solution".into());
    let assignment = Statement::new(StatementKind::Declaration(solution_binding,multiplication));

    //Function declaration
    let args = vec![
        VariableBinding::new(DataType::Integer, "x".into()),
        VariableBinding::new(DataType::Integer,"y".into())
    ];
    let test_fn_block = Block::new(vec![assignment,return_statement]);
    let fn_declaration = Expression::FnDecl("calculate".into(), test_fn_block, Some(args), Some(DataType::Integer));
    let test_fn = Statement::new(StatementKind::Expression(fn_declaration));

    let expected_ast = AbstractSyntaxTree::new(vec![test_fn]);
    assert_eq!(ast,expected_ast);
}

#[test]
fn while_test() {
    let src = String::from("\
    fn count(start: int, end: int) {\
        while start < end {
            start = start + 1;
        }
    }");

    let (ts,_) = Lexer::tokenize(src);
    let ast = ASTParser::new(ts).parse().expect("Expected Abstract Syntax Tree");

    let var_start = Expression::Symbol("start".into());
    let constant =  Expression::Literal(DataValue::Integer("1".into()));
    let increment = Expression::BinaryOp(Box::from(var_start), BinOp::Plus, Box::from(constant));
    let assignment = Expression::Assignment("start".into(), Box::from(increment));
    let while_condition = Expression::BinaryOp(Box::from(Expression::Symbol("start".into())), BinOp::Lt, Box::from(Expression::Symbol("end".into())));

    let stmts = vec![Statement::new(StatementKind::Expression(assignment))];
    let while_block = Block::new(stmts);

    let while_stmt = Statement::new(
        StatementKind::Expression(Expression::WhileLoop(Box::from(while_condition), while_block)));

    let function_stmts = vec![while_stmt];
    let start = VariableBinding::new(DataType::Integer,"start".into());
    let end = VariableBinding::new(DataType::Integer,"end".into());
    let fn_decl = Expression::FnDecl("count".into(),Block::new(function_stmts), Some(vec![start,end]), None);
    let function = Statement::new(StatementKind::Expression(fn_decl));
    let expected_ast = AbstractSyntaxTree::new(vec![function]);
    assert_eq!(ast,expected_ast);
}

#[test]
fn math_expression() {
    let src = String::from("fn doMath() {\
        let x: float = a - b * ( c/d + e/f);
    }");

    let c = Expression::Symbol(String::from("c"));
    let d = Expression::Symbol(String::from("d"));
    let e = Expression::Symbol(String::from("e"));
    let f = Expression::Symbol(String::from("f"));

    let a = Expression::Symbol(String::from("a"));
    let b = Expression::Symbol(String::from("b"));

    let c_div_d = Expression::BinaryOp(Box::from(c), BinOp::Divide, Box::from(d));
    let e_div_f = Expression::BinaryOp(Box::from(e), BinOp::Divide, Box::from(f));
    let addition = Expression::BinaryOp(Box::from(c_div_d), BinOp::Plus, Box::from(e_div_f));
    let multiplication = Expression::BinaryOp(Box::from(b), BinOp::Multi, Box::from(addition));
    let subtraction = Expression::BinaryOp(Box::from(a), BinOp::Minus, Box::from(multiplication));
    let binding = VariableBinding::new(DataType::Float,"x".into());
    let let_stmt =  Statement::new(StatementKind::Declaration(binding,subtraction));

    let function_block = Block::new(vec![let_stmt]);
    let fn_decl = Expression::FnDecl("doMath".into(),function_block,None,None);
    let fn_stmt = Statement::new(StatementKind::Expression(fn_decl));
    let expected_ast = AbstractSyntaxTree::new(vec![fn_stmt]);

    let (ts,_) = Lexer::tokenize(src);
    let ast = ASTParser::new(ts).parse().expect("ast parsing failed");
    assert_eq!(expected_ast,ast);
}