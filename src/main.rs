use frontend::lexer::Lexer;
use frontend::parser::ast_parser::*;

mod frontend;
mod backend;
#[cfg(test)]
mod tests;

fn main() {
    let src = String::from("fn test(): boolean{\n\tlet s = 5;\n\tlet b = a;\n\tif s == 5 { \n\t\treturn true; \n\t} \t\n}\nfn test1(): boolean{\n\tlet s = 5;\n\tlet b = a;\n\tif s == 5 { \n\t\treturn true; \n\t} \t\n}\nfn test2(): boolean{\n\tlet s = 5;\n\tlet b = a;\n\tif s == 5 { \n\t\treturn true; \n\t} \t\n}");
    println!("{}",&src);
    let (ts,_handle) = Lexer::tokenize(src.clone());
    let parser = ASTParser::new(ts);
    let _result  = parser.parse();
}
