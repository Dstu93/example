use std::collections::HashMap;

use crate::backend::interpreter::RuntimeError::ExpectedFnDeclaration;
use crate::backend::memory::{MemUnit, Ptr, AllocError};
use crate::frontend::syntax::{DataType, DataValue};
use crate::frontend::syntax::ast::{AbstractSyntaxTree, Block, Statement, VariableBinding, Expression};
use std::cell::RefCell;
use std::rc::Rc;

type FnTable<'a> = HashMap<&'a str, Funct<'a>>;
type SymbolTable<'b> = HashMap<&'b str,(Ptr,DataType)>;
type Heap = Box<dyn MemUnit<DataValue>>;

/// Takes an AbstractSyntaxTree and executes it at runtime.
pub struct RuntimeInterpreter<'a> {
    ast: &'a AbstractSyntaxTree,
    heap: RefCell<Heap>,
    fn_table: FnTable<'a>,
}

impl <'a>RuntimeInterpreter<'a> {

    /// creates a new RuntimeInterpreter Object with an AbstractSyntaxTree to execute
    pub fn new(ast: &'a AbstractSyntaxTree, mem_unit: Heap) -> Self {
        RuntimeInterpreter{
            ast,
            heap: RefCell::from(mem_unit),
            fn_table: HashMap::new(),
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
                    let value = self.resolve_expression(&mut symbol_table)?;
                    if let Some(v) = value {
                        let ptr = self.heap.get_mut().allocate(v)?;
                        symbol_table.insert(&var.symbol,(ptr,var.data_type));
                    }
                },
                Statement::FnDecl(_, _, _, _) => invalid_fn_decl()?,
                Statement::Break => invalid_stmt(Statement::Break, "break is only allowed in loops")?,
                Statement::Continue => {invalid_stmt(Statement::Continue,"contine is only allowed in loops")?},
                Statement::Return(e) => {
                    //Mainfunction does not return data
                    if e.is_some() { return Err(RuntimeError::UnexpectedReturnType); }
                    return Ok(());
                },
                Statement::WhileLoop(condition,block) => {

                },
                Statement::Loop(_) => {},
                Statement::If(_, _, _) => {},
                Statement::Expression(e) => {
                    match e {
                        Expression::FnCall(name, arguments) => {

                        },
                        Expression::UnaryOp(_, _) => {},
                        Expression::Assignment(_, _) => {},
                        Expression::BinaryOp(_, _, _) => {},
                        Expression::Symbol(_) => {},
                        Expression::Literal(_) => {},
                    };
                },
            }
        };
        Ok(())
    }

    fn execute_function(&mut self, func: &Funct, args: Vec<DataValue>) -> Result<Option<DataValue>,RuntimeError> {
        //TODO
        Ok(None)
    }

    fn resolve_expression(&mut self,symtbl: &mut SymbolTable) -> Result<Option<DataValue>,RuntimeError> {
        //TODO
        Ok(None)
    }

}

/// wrapper for a function declaration
struct Funct<'a> {
    pub name: &'a String,
    pub args: &'a Vec<VariableBinding>,
    pub return_type: &'a Option<DataType>,
    pub body: &'a Block,
}

///Wrapper for fn,symbol-table and heap.
// used for shortening function signatures
struct Scope<'a, 'b,'c> {
    fntbl: &'b FnTable<'a>,
    symtbl: &'b mut SymbolTable<'c>,
    heap: &'b mut Heap,
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
    UnexpectedReturnType,
    OutOfMemory,
}

impl From<AllocError> for RuntimeError{
    fn from(alloc_error: AllocError) -> Self {
        RuntimeError::OutOfMemory
    }
}