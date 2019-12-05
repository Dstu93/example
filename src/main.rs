use frontend::lexer::Lexer;
use frontend::parser::ast_parser::*;
use crate::backend::interpreter::RuntimeInterpreter;
use crate::backend::memory::Heap;
use crate::frontend::syntax::DataValue;

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
    let ast = match result {
        Ok(ast) => {ast},
        Err(error) => {
            println!("Could not parse program: {:#?}", error);
            return;
        },
    };

    //execute program
    let calculated_heap_size = 512 * 1024 * 1024 / std::mem::size_of::<DataValue>();
    println!("max heap size: {}", calculated_heap_size);
    let heap_space = Heap::new(calculated_heap_size);
    let mut interpreter = RuntimeInterpreter::new(&ast,Box::new(heap_space));
    let execution_result = interpreter.start();
    match execution_result {
        Ok(_) => {},
        Err(error) => {
            println!("RuntimeError {:#?}", error);
        },
    }
}
