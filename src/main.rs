
mod frontend;
#[cfg(test)]
mod tests;

use std::fs::File;
use std::io:: {Read,BufRead,BufReader};

use frontend::syntax::*;
use frontend::lexer::Lexer;
use frontend::parser::ast_parser::*;

fn main() {
    let src = String::from("fn test(): boolean{\n\tlet s = 5;\n\tlet b = a;\n\tif s == 5 { \n\t\treturn true; \n\t} \t\n}");
    println!("{}",&src);
    let ts = Lexer::tokenize(src.clone()).0;
    let parser = ASTParser::new();
    let result  = parser.parse(ts);
}
