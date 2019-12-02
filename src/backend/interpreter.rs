use crate::frontend::syntax::ast::AbstractSyntaxTree;

/// Takes an AbstractSyntaxTree and executes it at runtime.
pub struct RuntimeInterpreter<'a> {
    ast: &'a AbstractSyntaxTree,
    heap:
}

impl <'a>RuntimeInterpreter<'a> {

    /// creates a new RuntimeInterpreter Object with an AbstractSyntaxTree to execute
    pub fn new(ast: &'a AbstractSyntaxTree) -> Self {
        RuntimeInterpreter{ast}
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