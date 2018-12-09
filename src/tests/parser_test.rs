
use frontend::syntax::{*,ast::*,token::*};
use frontend::lexer::*;
use frontend::parser::ast_parser::*;

#[test]
fn fun_main_function(){

    let src = String::from("\
    fn main(){\
       let a = false;\
       if !a {\
          print(\"a is false\")\
       }\
    }");

    //TODO build ast for this source code by hand

    let (ts,handle) = Lexer::tokenize(src);
    let parser = ASTParser::new(ts);
    let ast= parser.parse().expect("expect abstract syntax tree");

    //FIXME compare trees
}