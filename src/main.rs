
mod syntax;
#[cfg(test)]
mod tests;

use std::fs::File;
use std::io:: {Read,BufRead,BufReader};

use syntax::lexer::*;

fn main() {
    let src = " let s = 5; \n let b = a;";
    println!("{}",&src);
    let res = Lexer::tokenize(&src);
    println!("{:#?}",res);
}
