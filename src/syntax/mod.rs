pub mod ast;
pub mod lexer;
pub mod token;


/// Represents basic data types
#[derive(PartialEq,PartialOrd,Clone,)]
pub enum Value{
    INTEGER(i64),
    FLOAT(f64),
    BOOLEAN(bool),
    STRING(String),
}