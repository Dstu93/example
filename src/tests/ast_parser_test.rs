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
    let a_declaration = Statement::Declaration(binding, boolean_expression);

    let a = Expression::Symbol("a".into());
    let if_condition = Expression::UnaryOp(UnOp::Negation, Box::new(a));
    let message = Expression::Literal(DataValue::String("a is false".into()));
    let args = vec![message];
    let print_call = Expression::FnCall("print".into(), args);
    let print_stmt = Statement::Expression(print_call);
    let block = Block::new(vec![print_stmt]);
    let if_stmt = Statement::If(Box::new(if_condition), block, None);

    let block = Block::new(vec![a_declaration,if_stmt]);
    let main_function = Statement::FnDecl("main".into(), block, Vec::new(), None);

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
    let return_stmt = Statement::Return(Some(Box::new(return_str)));
    let block = Block::new(vec![return_stmt]);
    let test_fn = Statement::FnDecl("test".into(), block, Vec::new(), Some(DataType::String));

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
    let return_statement = Statement::Return(Some(Box::new(return_value)));
    let y = Expression::Symbol("y".into());
    let x = Expression::Symbol("x".into());
    let multiplication = Expression::BinaryOp(Box::new(x), BinOp::Multi, Box::new(y));
    let solution_binding = VariableBinding::new(DataType::Integer,"solution".into());
    let assignment = Statement::Declaration(solution_binding, multiplication);

    //Function declaration
    let args = vec![
        VariableBinding::new(DataType::Integer, "x".into()),
        VariableBinding::new(DataType::Integer,"y".into())
    ];
    let test_fn_block = Block::new(vec![assignment,return_statement]);
    let test_fn = Statement::FnDecl("calculate".into(), test_fn_block, args, Some(DataType::Integer));

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

    let stmts = vec![Statement::Expression(assignment)];
    let while_block = Block::new(stmts);

    let while_stmt = Statement::WhileLoop(Box::from(while_condition), while_block);

    let function_stmts = vec![while_stmt];
    let start = VariableBinding::new(DataType::Integer,"start".into());
    let end = VariableBinding::new(DataType::Integer,"end".into());
    let fn_decl = Statement::FnDecl("count".into(),Block::new(function_stmts), vec![start,end], None);
    let expected_ast = AbstractSyntaxTree::new(vec![fn_decl]);
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
    let let_stmt =  Statement::Declaration(binding, subtraction);

    let function_block = Block::new(vec![let_stmt]);
    let fn_decl = Statement::FnDecl("doMath".into(),function_block,Vec::new(),None);
    let expected_ast = AbstractSyntaxTree::new(vec![fn_decl]);

    let (ts,_) = Lexer::tokenize(src);
    let ast = ASTParser::new(ts).parse().expect("ast parsing failed");
    assert_eq!(expected_ast,ast);
}