pub mod ast;
pub mod token;


/// Represents basic data types
#[derive(PartialEq,PartialOrd,Clone,Debug)]
pub enum DataValue {
    Integer(String),
    Float(String),
    Boolean(bool),
    String(String),
}

/// Enum of all standard data types
#[derive(Eq, PartialEq,Ord, PartialOrd,Copy, Clone,Hash,Debug)]
pub enum DataType{
    Float,
    Integer,
    Boolean,
    String,
}