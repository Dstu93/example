use crate::frontend::syntax::ast::AbstractSyntaxTree;
use crate::backend::memory::MemUnit;
use crate::frontend::syntax::DataValue;

/// Takes an AbstractSyntaxTree and executes it at runtime.
pub struct RuntimeInterpreter<'a,T> where T: MemUnit<DataValue> {
    ast: &'a AbstractSyntaxTree,
    heap: T,
}

impl <'a,T>RuntimeInterpreter<'a,T> where T: MemUnit<DataValue> {

    /// creates a new RuntimeInterpreter Object with an AbstractSyntaxTree to execute
    pub fn new(ast: &'a AbstractSyntaxTree, mem_unit: T) -> Self {
        RuntimeInterpreter{ast, heap: mem_unit}
    }

    /// starts the RuntimeInterpreter and executes the program described by the AbstractSyntaxTree.
    /// This method is self consuming and the interpreter only stops after completed the execution or
    /// a RuntimeError occurs
    pub fn start(self) -> Result<(),RuntimeError> {
        //TODO
        Err(RuntimeError::NullPointerError)
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq,Copy, Clone,Hash,Debug)]
pub enum RuntimeError {
    NullPointerError
}