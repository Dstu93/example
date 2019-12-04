use crate::frontend::syntax::ast::{AbstractSyntaxTree, Statement, VariableBinding, Block};
use crate::backend::memory::MemUnit;
use crate::frontend::syntax::{DataValue, DataType};
use std::collections::HashMap;
use crate::backend::interpreter::RuntimeError::ExpectedFnDeclaration;

/// Takes an AbstractSyntaxTree and executes it at runtime.
pub struct RuntimeInterpreter<'a,T> where T: MemUnit<DataValue> {
    ast: &'a AbstractSyntaxTree,
    heap: T,
    fn_map: HashMap<&'a str,Function<'a>>,
}

impl <'a,T>RuntimeInterpreter<'a,T> where T: MemUnit<DataValue> {

    /// creates a new RuntimeInterpreter Object with an AbstractSyntaxTree to execute
    pub fn new(ast: &'a AbstractSyntaxTree, mem_unit: T) -> Self {
        RuntimeInterpreter{
            ast,
            heap: mem_unit,
            fn_map: HashMap::new()
        }
    }

    /// starts the RuntimeInterpreter and executes the program described by the AbstractSyntaxTree.
    /// This method is self consuming and the interpreter only stops after completed the execution or
    /// a RuntimeError occurs
    pub fn start(&mut self) -> Result<(),RuntimeError> {
        self.add_fn_declarations()?;
        self.execute_main()
    }

    fn add_fn_declarations(&mut self) -> Result<(),RuntimeError> {
        for node in &self.ast.nodes {
            match node {
                Statement::FnDecl(name, block, args, return_type) => {
                    let function = Function { name, args, return_type, body: block };
                    self.fn_map.insert(name, function);
                },
                _ => return Err(ExpectedFnDeclaration),
            }
        }
        Ok(())
    }

    fn execute_main(&mut self) -> Result<(),RuntimeError> {
        let main_func = match self.fn_map.get_mut("main") {
            Some(func) => func,
            None => return Err(RuntimeError::NoMainFn),
        };
        self.execute_function(main_func)
    }

    fn execute_function(&mut self, func: &Function) -> Result<(),RuntimeError> {
        Ok(())
    }
}

#[derive(PartialOrd,PartialEq,Clone,Debug)]
pub enum RuntimeError {
    /// Error when Function declaration is expected,
    /// like the first statement in the AST
    ExpectedFnDeclaration,
    /// no main function found
    NoMainFn,
    NullPointerError
}

/// wrapper for a function declaration
struct Function<'a> {
    pub name: &'a String,
    pub args: &'a Vec<VariableBinding>,
    pub return_type: &'a Option<DataType>,
    pub body: &'a Block,
}
