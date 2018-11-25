
mod frontend;
#[cfg(test)]
mod tests;

use std::fs::File;
use std::io:: {Read,BufRead,BufReader};

use frontend::syntax::*;
use frontend::lexer::Lexer;

fn main() {
    let src = " let s = 5; \n let b = a;".into();
    println!("{}",&src);
    let res = Lexer::tokenize(src).0.collect();
    println!("{:#?}",res);
}
