use frontend::lexer::Lexer;
use frontend::parser::ast_parser::*;

mod frontend;
mod backend;
#[cfg(test)]
mod tests;

fn main() {
    let src = String::from("\
        fn fibonacci(n: int): int {
            if n == 1 or n == 2 {
               return 1;
            }

            return fibonacci(n - 1) + fibonacci(n - 2);
        }
    ");
    println!("{}",&src);
    let (ts,_) = Lexer::tokenize(src.clone());
    let parser = ASTParser::new(ts);
    let result  = parser.parse();
    println!("Lexer Result: {:#?}", result);
}
