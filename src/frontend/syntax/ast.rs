
use std::convert::From;
use frontend::syntax::{DataType, DataValue};

/// Represents an unique id of an Node in the abstract frontend.syntax tree
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

impl From<u32> for NodeId {
    fn from(n: u32) -> Self {
        NodeId::new(n)
    }
}

/// Represents an Id for identify an Symbol/Variable on the Stack,
/// like ' let x = 5;'  so we replace 'x' intern with an unique id (SymbolId)
#[derive(Ord, PartialOrd, Eq, PartialEq,Copy, Clone,Debug,Hash)]
pub struct SymbolId {
    id: u32
}

impl SymbolId {

    pub fn new(id: u32) -> SymbolId {
        SymbolId{id}
    }

    /// Returns a new SymbolId which is the successor of this SymbolId
    pub fn successor(&self) -> SymbolId {
        SymbolId{id: self.id + 1}
    }

    /// returns this id as u32
    pub fn as_u32(&self) -> u32{
        self.id
    }
}

impl From<u32> for SymbolId {
    fn from(n: u32) -> Self {
        SymbolId::new(n)
    }
}

/// Representation of the abstract frontend.syntax tree (short AST).
/// represents the program in memory
#[derive(PartialOrd, PartialEq,Clone,Debug)]
pub struct AbstractSyntaxTree {
    pub uid: NodeId,
    pub nodes: Vec<Statement>,
}

impl AbstractSyntaxTree{
    pub fn new(stmts: Vec<Statement>) -> AbstractSyntaxTree{
        AbstractSyntaxTree{uid: NodeId::new(0),nodes: stmts}
    }

    pub fn add_stmt(&mut self,stmt: Statement){
        self.nodes.push(stmt);
    }
}

/// Represents an Statement
#[derive(PartialOrd, PartialEq,Clone,Debug)]
pub struct Statement {
    pub uid: NodeId,
    pub kind: StatementKind,
}
impl Statement{
    pub fn new(uid: NodeId,kind: StatementKind) -> Statement{
        Statement{uid,kind}
    }
}

#[derive(PartialOrd, PartialEq,Clone,Debug)]
pub enum StatementKind {
    Declaration(VariableBinding,Expression),
    Expression(Expression),
}

/// Represents an Expression
#[derive(PartialEq, PartialOrd,Debug,Clone)]
pub struct Expression {
    pub uid: NodeId,
    pub kind: ExpressionKind,
}
impl Expression{
    pub fn new(uid: NodeId,kind: ExpressionKind) -> Expression{
        Expression{uid,kind}
    }
}

//TODO example
/// Represents an Binding of a value to a symbol (name of a variable)
#[derive(PartialEq, PartialOrd,Hash,Debug,Clone,Ord, Eq,Copy)]
pub struct VariableBinding {
    pub uid: NodeId,
    pub data_type: DataType,
    pub symbol: SymbolId,
}
impl VariableBinding{
    pub fn new(uid: NodeId,data_type: DataType,symbol: SymbolId) -> VariableBinding{
        VariableBinding{uid,data_type,symbol}
    }
}

/// Enum of all Expressions
#[derive(PartialOrd, PartialEq,Clone,Debug)]
pub enum ExpressionKind {
    /// call of an std function or a user created function,
    /// String represents the function name
    FnCall(String,Option<Vec<Argument>>),
    /// Declaration of a new Function, String = Name,Block of statements in the function Body, Option with possible arguments
    /// and an Option of an Returned DataType
    FnDecl(String,Block,Option<Vec<VariableBinding>>,Option<DataType>),
    /// Unary Operator Expression like "!isValid"
    UnaryOp(UnOp,Box<Expression>),
    /// binary operator like "*" or "!="
    BinaryOp(BinOp,Box<Expression>,Box<Expression>),
    /// If statement with an optional else block.
    /// if "expression " {block} else {block}
    If(Box<Expression>,Block,Option<Block>), //Expression must be boxed because of recursion
    /// single variable like "counter"
    Symbol(SymbolId),
    /// represents a literal like "42" or "foobar"
    Literal(DataValue),
    /// Break of an loop
    Break,
    /// Continue of an loop
    Continue,
    /// Return statement, can return an value or nothing
    Return(Option<Box<Expression>>),
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
    pub uid: NodeId,
    pub statements: Vec<Statement>,
}
impl Block{
    pub fn new(uid: NodeId, stmts: Vec<Statement>) -> Block{
        Block{uid,statements: stmts}
    }
    pub fn add_stmt(&mut self,stmt: Statement){
        self.statements.push(stmt);
    }
}

/// Represents a function argument.
/// An argument consists of an data Type and the concrete value
#[derive(PartialOrd, PartialEq,Clone,Debug)]
pub struct Argument {
    pub data_type: DataType,
    pub value: Expression,
}

impl Argument{
    pub fn new(dtype: DataType,value: Expression) -> Argument{
        Argument{data_type: dtype,value}
    }
}