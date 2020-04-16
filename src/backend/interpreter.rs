use std::collections::HashMap;

use crate::backend::memory::{MemUnit, Ptr, AllocError};
use crate::frontend::syntax::{DataType, DataValue};
use crate::frontend::syntax::ast::{AbstractSyntaxTree, Block, Statement, VariableBinding, Expression, BinOp};
use std::cell::RefCell;

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
                _ => return Err(RuntimeError::ExpectedFnDeclaration),
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
            let ret = self.execute_stmt(&mut symbol_table,stmt)?;
            if let Some(_) = ret  {
                return Err(RuntimeError::UnexpectedReturnType);
            }
        };
        Ok(())
    }

    fn execute_if_stmt(&mut self,
                       mut symbol_table: &mut SymbolTable<'a>,
                       condition: &'a Expression,
                       if_block: &'a Block, else_block: &'a Option<Block> ) -> Result<(),RuntimeError> {

        let condition = self.resolve_boolean_expr(condition, &mut symbol_table)?;
        if condition {
            //TODO
            if_block.statements.iter();
        } else {
            //TODO
            match else_block {
                None => {},
                Some(else_block) => {},
            }
        }
        Ok(())
    }

    fn var_declaration(&mut self, mut symbol_table: &mut SymbolTable<'a>, stmt: &Statement, var: &'a VariableBinding, exp: &'a Expression) -> Result<(),RuntimeError> {
        let value = self.resolve_expression(exp, &mut symbol_table)?;
        match value {
            Some(v) => {
                let ptr = self.heap.get_mut().allocate(v)?;
                symbol_table.insert(&var.symbol, (ptr, var.data_type));
                Ok(())
            }
            None => Err(RuntimeError::InvalidStmt(stmt.clone(), "right side of assignment returns no value")),
        }
    }

    fn execute_function(&mut self, func: &Funct<'a>, args: Vec<DataValue>) -> Result<Option<DataValue>,RuntimeError> {
        let mut symbol_table: SymbolTable = HashMap::new();
        if func.args.len() != args.len() {
            return Err(RuntimeError::FnArgsCountMismatch { name: func.name.clone(), found: args.len() as u8, expected: func.args.len() as u8 })
        }
        //Add fn arguments on our "stack"/symbol table
        for (index, value) in args.into_iter().enumerate() {
            let binding = func.args.get(index).expect("fn args mismatch");
            typecheck(&binding.symbol,binding.data_type,&value)?;
            let ptr = self.heap.get_mut().allocate(value)?;
            symbol_table.insert(binding.symbol.as_str(), (ptr, binding.data_type));
        }

        let mut return_value = None;
        for stmt in &func.body.statements {
            self.execute_stmt(&mut symbol_table, &stmt)?; //TODO
        };


        Ok(return_value)
    }

    fn execute_stmt(&mut self, mut symbol_table: &mut SymbolTable<'a>, stmt: &'a Statement) -> Result<Option<DataValue>,RuntimeError> {
        match stmt {
            Statement::Declaration(binding, expr) => {
                self.var_declaration(&mut symbol_table, &stmt, &binding, &expr)?;
            },
            Statement::FnDecl(_, _, _, _) => invalid_fn_decl()?,
            Statement::Break => {return Ok(None)},
            Statement::Continue => {return Ok(None)},
            Statement::Return(e) => {
                match e {
                    None => {return Ok(None)},
                    Some(e) => {
                        let value = self.resolve_expression(e,&mut symbol_table)?;
                        return Ok(value);
                    },
                }
            },
            Statement::WhileLoop(_, _) => {},
            Statement::Loop(_) => {},
            Statement::If(condition, if_block, else_block) => {
                //TODO catch return
                self.execute_if_stmt(&mut symbol_table,condition,if_block,else_block)?;
            },
            Statement::Expression(e) => {let _ = self.resolve_expression(e,symbol_table)?;},
        }

        Ok(None)
    }

    fn resolve_expression(&mut self, expr: &'a Expression, symtbl: &mut SymbolTable<'a>) -> Result<Option<DataValue>,RuntimeError> {
        match expr {
            Expression::FnCall(name, args) => {
                let mut arg_values = Vec::new();
                for exp in args {
                    let value = self.resolve_expression(exp,symtbl)?;
                    match value {
                        None => { return Err(RuntimeError::NullPointerError)},
                        Some(val) => {arg_values.push(val)},
                    }
                    let func = self.fn_table.get(name.as_str());
                    match func {
                        None => {return Err(RuntimeError::FnNotExist(name.clone()))},
                        Some(func) => {
                            //TODO call func
                        },
                    }
                }
            },
            Expression::UnaryOp(unary_op, expr) => {
                //TODO
            },
            Expression::Assignment(symbol_name, expr) => {
                let value = self.resolve_expression(expr,symtbl)?;
                match symtbl.get(&**symbol_name) {
                    None => {return Err(RuntimeError::VarDoesNotExist(symbol_name.clone()))},
                    Some((ptr,datatype)) => {
                        match value {
                            Some(data) => {
                                typecheck(symbol_name, *datatype, &data)?;
                                self.heap.get_mut().replace(ptr, data);
                            },
                            None => {
                                self.heap.get_mut().replace(ptr,DataValue::None);
                            }
                        }
                    },

                }
            }
            Expression::BinaryOp(left, op, right) => {
                match op {
                    BinOp::Plus => {},
                    BinOp::Minus => {},
                    BinOp::Multi => {},
                    BinOp::Divide => {},
                    BinOp::Eq => {},
                    BinOp::Neq => {},
                    BinOp::Gt => {},
                    BinOp::Ge => {},
                    BinOp::Lt => {},
                    BinOp::Le => {},
                    BinOp::And => {},
                    BinOp::Or => {},
                }
            },
            Expression::Symbol(var) => return self.lookup_symbol(symtbl, var),
            Expression::Literal(value) => return Ok(Some(value.clone())),
        };

        Ok(None)
    }

    fn lookup_symbol(&mut self, symtbl: &mut SymbolTable<'a>, var: &'a String) -> Result<Option<DataValue>,RuntimeError> {
        match symtbl.get(var.as_str()) {
            None => return Err(RuntimeError::VarDoesNotExist(var.clone())),
            Some((ptr, dtype)) => {
                match self.heap.get_mut().retrieve(ptr) {
                    None => return Err(RuntimeError::NullPointerError),
                    Some(value) => return Ok(Some(value.clone())),
                }
            },
        }
    }

    fn while_expr(&mut self,condition: &'a Expression, block: &Block, symtbl: &mut SymbolTable) -> Result<(),RuntimeError> {
        //TODO
        Ok(())
    }

    fn resolve_boolean_expr(&mut self,condition: &'a Expression,symtbl: &mut SymbolTable<'a>) -> Result<bool,RuntimeError> {
        let value = self.resolve_expression(condition,symtbl)?;
        let value = match value {
            None => return Err(RuntimeError::ExpectedBooleanExpr(condition.clone())),
            Some(v) => v,
        };

        match value {
            DataValue::Boolean(b) => Ok(b),
            _ => Err(RuntimeError::TypeError(value.clone(), DataType::Boolean)),
        }
    }
}

fn execute_bin_expr(left: &DataValue,op: BinOp, right: &DataValue){

}

fn typecheck(symbol: &str,data_type: DataType,value: &DataValue) -> Result<(),RuntimeError> {
    match data_type {
        DataType::Float => {
            if let DataValue::Float(_) = value {
                return Ok(());
            }
        },
        DataType::Integer => {
            if let DataValue::Integer(_) = value {
                return Ok(())
            }
        },
        DataType::Boolean => {
            if let DataValue::Boolean(_) = value {
                return Ok(())
            }
        },
        DataType::String => {
            if let DataValue::String(_) = value {
                return Ok(())
            }
        },
    }

    Err(RuntimeError::WrongType(symbol.to_owned(),value.clone(),data_type))
}

/// wrapper for a function declaration
struct Funct<'a> {
    pub name: &'a String,
    pub args: &'a Vec<VariableBinding>,
    pub return_type: &'a Option<DataType>,
    pub body: &'a Block,
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
    FnNotExist(String),
    /// Variable/Symbol does not exist in this scope/stack
    VarDoesNotExist(String),
    ExpectedBooleanExpr(Expression),
    TypeError(DataValue,DataType),
    FnArgsCountMismatch{name: String,found: u8,expected: u8},
    WrongType(String,DataValue,DataType),
    OutOfMemory,
}

impl From<AllocError> for RuntimeError{
    fn from(_: AllocError) -> Self {
        RuntimeError::OutOfMemory
    }
}