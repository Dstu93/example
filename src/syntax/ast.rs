
use syntax::{DataType, DataValue};

//FIXME implements method or pub fields?

/// Represents an unique id of an Node in the abstract syntax tree
#[derive(Copy, Clone,Eq, PartialEq,Ord, PartialOrd,Debug,Hash)]
pub struct NodeId {
    id: u32,
}

impl NodeId {

    /// Creates a new NodeId with a certain id
    pub fn new(id: u32) -> NodeId{
        NodeId{id}
    }

    /// creates a new NodeId which is the successor of the given NodeId
    pub fn new_next_id(node: NodeId) -> NodeId{
        let next = node.as_u32() + 1;
        NodeId::new(next)
    }

    /// returns the unique id as u32
    pub fn as_u32(&self) -> u32{
        self.id
    }

}

/// Representation of the abstract syntax tree (short AST).
/// represents the program in memory
pub struct AbstractSyntaxTree {
    //root: AstNode, //FIXME Root Node
}

//TODO explain what an statement is
/// Represents an Statement
#[derive(PartialOrd, PartialEq,Clone,Debug)]
pub struct Statement {
    uid: NodeId,
    kind: StatementKind,
}

#[derive(PartialOrd, PartialEq,Clone,Debug)]
pub enum StatementKind {
    Declaration(VariableBinding,Expression),
    Expression(Expression),
}

//TODO explain what an expression is
#[derive(PartialEq, PartialOrd,Debug,Clone)]
pub struct Expression {
    uid: NodeId,
    kind: ExpressionKind,
}

//TODO example
/// Represents an Binding of a value to a symbol (name of a variable)
#[derive(PartialEq, PartialOrd,Hash,Debug,Clone)]
pub struct VariableBinding {
    uid: NodeId,
    data_type: DataType,
    symbol: String,
}

/// Enum of different
#[derive(PartialOrd, PartialEq,Clone,Debug)]
pub enum ExpressionKind {
    /// call of an std function or a user created function,
    /// String represents the function name
    FnCall(String,Option<Vec<Argument>>),
    /// Declaration of a new Function, String = Name, Option with possible arguments
    /// and an Option of an Returned DataType
    FnDecl(String,Option<Vec<Argument>>,Option<DataType>),
    /// Unary Operator Expression like "!isValid"
    UnaryOp(UnOp,Box<Expression>),
    /// binary operator like "*" or "!="
    BinaryOp(BinOp,Box<Expression>,Box<Expression>),
    /// If statement with an optional else block.
    /// if "expression " {block} else {block}
    If(Box<Expression>,Block,Option<Block>), //Expression must be boxed because of recursion
    /// single variable like "counter"
    Symbol(VariableBinding),
    /// represents a literal like "42" or "foobar"
    Literal, //FIXME replace with correct value struct
    /// Break of an loop
    Break,
    /// Continue of an loop
    Continue,
    /// Return statement, can return an value or nothing
    Return(Box<Option<Expression>>),
    /// While loop. The expression represents the condition and the
    /// block will be executed every loop cycle
    WhileLoop(Box<Expression>,Block),
    /// loop{block}, loops until break or return statement
    Loop(Block),
}

/// Enum of binary operators
#[derive(Ord, PartialOrd, Eq, PartialEq,Copy, Clone,Debug,Hash)]
pub enum BinOp {
    /// + Operator
    Plus,
    /// - Minus Operator
    Minus,
    /// * Multiplication Operator
    Multi,
    /// division Operator "3/4"
    Divide,
    /// Equal Operator "a == b"
    Eq,
    /// Not Equal Operator " a != b"
    Neq,
    /// Greater Then Operator " a > b"
    Gt,
    /// Greater Then Equal Operator " a >= b"
    Ge,
    /// Less Then Operator "a < b"
    Lt,
    /// Less Then Equal Operator "a <= b"
    Le,
}

/// Enum of unary operators
#[derive(Ord, PartialOrd, Eq, PartialEq,Copy, Clone,Hash,Debug)]
pub enum UnOp {
    /// ! Operator for inverting an single Expression
    Negation,
}

/// represents an block of statements like if {block} else {block}
/// or an function call like fn doSomething(){block}
#[derive(PartialOrd, PartialEq,Clone,Debug)]
pub struct Block {
    uid: NodeId,
    statements: Vec<Statement>,
}

/// Represents a function argument.
/// An argument consists of an data Type and the concrete value
#[derive(PartialOrd, PartialEq,Clone,Debug)]
pub struct Argument {
    data_type: DataType,
    value: DataValue,
}