

/// Enum for Types of Nodes for the Abstract Syntax Tree
#[derive(Copy, Clone,Eq, PartialEq,Ord, PartialOrd,Debug,Hash)]
pub enum NodeType{
    INTEGER,
    FLOAT,
    STRING,
    BOOLEAN,
    DECLARATION,
    ASSIGN,
    CALL,
    IF,
    OPERATION,
}

/// Node of an Abstract Syntax Tree
pub struct AstNode{
    node_type: NodeType,
}


/// Representation of the abstract syntax tree (short AST).
/// represents the program in memory
pub struct AbstractSyntaxTree{
    root: AstNode,
}