use std::collections::HashMap;

use crate::backend::interpreter::RuntimeError::ExpectedFnDeclaration;
use crate::backend::memory::{MemUnit, Ptr};
use crate::frontend::syntax::{DataType, DataValue};
use crate::frontend::syntax::ast::{AbstractSyntaxTree, Block, Statement, VariableBinding};

type FnTable<'a> = HashMap<&'a str, Funct<'a>>;
type SymbolTable<'b> = HashMap<&'b str,Ptr>;

/// Takes an AbstractSyntaxTree and executes it at runtime.
pub struct RuntimeInterpreter<'a,T> where T: MemUnit<DataValue> {
    ast: &'a AbstractSyntaxTree,
    heap: T,
    fn_table: FnTable<'a>,
}

impl <'a,T>RuntimeInterpreter<'a,T> where T: MemUnit<DataValue> {

    /// creates a new RuntimeInterpreter Object with an AbstractSyntaxTree to execute
    pub fn new(ast: &'a AbstractSyntaxTree, mem_unit: T) -> Self {
        RuntimeInterpreter{
            ast,
            heap: mem_unit,
            fn_table: HashMap::new()
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
                    let function = Funct { name, args, return_type, body: block };
                    self.fn_table.insert(name, function);
                },
                _ => return Err(ExpectedFnDeclaration),
            }
        }
        Ok(())
    }

    fn execute_main(&mut self) -> Result<(),RuntimeError> {
        let main_func = match self.fn_table.get("main") {
            Some(func) => func,
            None => return Err(RuntimeError::NoMainFn),
        };

        let mut symbol_table: SymbolTable = HashMap::new();
        for stmt in &main_func.body.statements {
            match stmt {
                Statement::Declaration(var, exp) => {
                    //TODO resolve and put variable on the heap and symbol table
                },
                Statement::FnDecl(_, _, _, _) => invalid_fn_decl()?,
                Statement::Break => invalid_stmt(Statement::Break, "break is only allowed in loops")?,
                Statement::Continue => {invalid_stmt(Statement::Continue,"contine is only allowed in loops")?},
                Statement::Return(e) => {

                },
                Statement::WhileLoop(_, _) => {},
                Statement::Loop(_) => {},
                Statement::If(_, _, _) => {},
                Statement::Expression(_) => {},
            }
        };
        Ok(())
    }

}

fn execute_function<'a,T>(func: &Funct,
                          fn_map: &FnTable,
                          heap: &mut T,
                          stack: SymbolTable) -> Result<(),RuntimeError> where T: MemUnit<DataValue> {

    Ok(())
}

fn invalid_stmt(stmt: Statement, reason: &'static str) -> Result<(),RuntimeError> {
    Err(RuntimeError::InvalidStmt(stmt,reason))
}

fn invalid_fn_decl() -> Result<(),RuntimeError> {
    Err(RuntimeError::FnDeclInFnBody)
}

#[derive(PartialOrd,PartialEq,Clone,Debug)]
pub enum RuntimeError {
    /// Error when Function declaration is expected,
    /// like the first statement in the AST
    ExpectedFnDeclaration,
    /// no main function found
    NoMainFn,
    /// Found Function Declaration in Fn Body
    FnDeclInFnBody,
    /// invalid stmt, reason as string
    InvalidStmt(Statement,&'static str),
    NullPointerError,
}

/// wrapper for a function declaration
struct Funct<'a> {
    pub name: &'a String,
    pub args: &'a Vec<VariableBinding>,
    pub return_type: &'a Option<DataType>,
    pub body: &'a Block,
}
