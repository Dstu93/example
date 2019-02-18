
use frontend::syntax::{DataValue,DataType,ast::*};

use std::collections::HashMap;

/// Takes an AbstractSyntaxTree and executes it at runtime.
pub struct RuntimeInterpreter {
    /// table of symbols and its values on the heap
    symbol_table: HashMap<SymbolId, DataValue>,
    ast: AbstractSyntaxTree,
    pos: NodeId,
}

impl RuntimeInterpreter {

    /// creates a new RuntimeInterpreter Object with an AbstractSyntaxTree to execute
    pub fn new(ast: AbstractSyntaxTree) -> Self{
        RuntimeInterpreter{symbol_table: HashMap::new(),ast,pos: 0.into()}
    }

    /// starts the RuntimeInterpreter and executes the program described by the AbstractSyntaxTree.
    /// This method is self consuming and the interpreter only stops after completed the execution or
    /// an RuntimeError occurs
    pub fn start(self) -> Result<(),RuntimeError> {
        //TODO
        Err(RuntimeError::NullPointerError)
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq,Copy, Clone,Hash,Debug)]
pub enum RuntimeError {
    NullPointerError
}