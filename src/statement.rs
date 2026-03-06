use crate::loxfuncs::LoxFunction;
use crate::token::{Token,AtomicLiteral};
use std::rc::Rc;
use crate::expression::{ExpressionType, is_truthy};
use crate::resolver::{Resolver, Resolvable};
use crate::lox_error::LoxResult;
use crate::interpreter::Interpreter;
use crate::token::Literal;
use crate::lox_error::LoxError;
use std::cell::RefCell;

#[derive(Clone)]
pub enum StatementType {
    ExpressionStatement(ExpressionType),
    PrintStatement(ExpressionType),
    LetStatement(LetExpressionProps),
    BlockStatement(Vec<StatementType>),
    IfStatement(IfProps),
    Function(FunctionProps),
    WhileStatement(WhileProps),
    ReturnStatement(ReturnProps),
}
#[derive(Clone)]
pub struct ReturnProps {
    pub _keyword: Token,
    pub value: Option<ExpressionType>
}
#[derive(Clone)]
pub struct FunctionProps {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Rc<StatementType>
}
#[derive(Clone)]
pub struct WhileProps {
    pub condition: ExpressionType,
    pub statement: Box<StatementType>,
}
#[derive(Clone)]
pub struct IfProps {
    pub comparison: ExpressionType,
    pub ifcase: Box<StatementType>,
    pub elsecase: Option<Box<StatementType>>,
}
#[derive(Clone)]
pub struct LetExpressionProps {
    pub name: Token,
    pub initializer: Box<ExpressionType>,
}

impl Resolvable for StatementType {
    fn resolve(&self, resolver: &mut Resolver) -> LoxResult<()> {
        match &self {
            StatementType::BlockStatement(statements) => {
                resolver.begin_scope();
                resolver.resolve(statements)?;
                resolver.end_scope();
            }

            StatementType::LetStatement(statement) => {
                resolver.declare(&statement.name);
                match *statement.initializer {
                    ExpressionType::Literal(AtomicLiteral::Nil) => {},
                    _ => {
                        resolver.resolve(&*statement.initializer)?;
                    }
                }
                resolver.define(&statement.name);
            }

            StatementType::Function(func) => {
                resolver.declare(&func.name);
                resolver.define(&func.name);
                resolver.resolve_function(func)?;
            }

            StatementType::ExpressionStatement(statement) => {
                resolver.resolve(statement)?;
            }

            StatementType::IfStatement(ifprop) => {
                resolver.resolve(&ifprop.comparison)?;
                resolver.resolve(&*ifprop.ifcase)?;
                if let Some(elsecase) = &ifprop.elsecase {
                    resolver.resolve(&**elsecase)?;
                }
            }

            StatementType::PrintStatement(statement) => {
                resolver.resolve(statement)?;
            }

            StatementType::ReturnStatement(statement) => {
                if let Some(value) = &statement.value {
                    resolver.resolve(value)?;
                }
            }

            StatementType::WhileStatement(statement) => {
                resolver.resolve(&statement.condition)?;
                resolver.resolve(&*statement.statement)?;
            }
            // _ => {}
        }
        Ok(())
    }   
}

impl StatementType {
    pub fn evaluate(&mut self, interpreter: &mut Interpreter) -> LoxResult<()> {
        match self {
            StatementType::ExpressionStatement(value) => {
                value.evaluate(interpreter)?;
                Ok(())
            }
            StatementType::PrintStatement(expr) => {
                let output = expr.evaluate(interpreter)?;
                println!("{}", output);
                Ok(())
            }
            StatementType::LetStatement(expr) => match *expr.initializer {
                ExpressionType::Literal(AtomicLiteral::Nil) => {
                    interpreter.env.borrow_mut().define(
                        expr.name.lexeme.clone(),
                        Rc::new(Literal::Basic(AtomicLiteral::Nil)),
                    );
                    Ok(())
                }
                _ => {
                    let result = expr.initializer.evaluate(interpreter)?;
                    interpreter.env
                        .borrow_mut()
                        .define(expr.name.lexeme.clone(), result);
                    Ok(())
                }
            },
            StatementType::BlockStatement(statements) => Self::evaluate_blocks(statements, interpreter),
            StatementType::IfStatement(iftype) => {
                Self::evaluate_if(iftype, interpreter)
            },
            StatementType::WhileStatement(wild) => {
                Self::evaluate_while(wild, interpreter)
            },
            StatementType::Function(func_props) => {
                let function = LoxFunction::new(Rc::new(func_props), interpreter);
                interpreter.env.borrow_mut().define(
                    func_props.name.lexeme.clone(),
                    Rc::new(Literal::LoxCallable(Box::new(function))),
                );
                Ok(())
            }
            StatementType::ReturnStatement(prop) => {
                let value = match &mut prop.value {
                    Some(expr) => expr.evaluate(interpreter)?,
                    None => Rc::new(Literal::Basic(AtomicLiteral::Nil)),
                };
                Err(LoxError::ReturnValue(value))
            }
        }
    }

    pub fn evaluate_blocks(statements: &mut Vec<StatementType>, interpreter: &mut Interpreter) -> LoxResult<()> {
        let previous = Rc::clone(&interpreter.env);
    
        interpreter.env = Rc::new(RefCell::new(crate::environment::Environment::new(Some(previous.clone()))));
    
        for statement in statements {
            statement.evaluate(interpreter)?;
        }
    
        interpreter.env = previous;
        Ok(())
    }

    pub fn evaluate_func_block(
        statement: &mut StatementType,
        closure: Rc<RefCell<crate::environment::Environment>>,
        interpreter: &mut Interpreter,
    ) -> LoxResult<()> {
        let previous = Rc::clone(&interpreter.env);
    
        interpreter.env = closure;
    
        let result = statement.evaluate(interpreter);
    
        interpreter.env = previous;
        result  
    }

    pub fn evaluate_if(ifinput: &mut IfProps, interpreter: &mut Interpreter) -> LoxResult<()> {
        let comparison = ifinput.comparison.evaluate(interpreter)?;
    
        if is_truthy(&comparison) {
            ifinput.ifcase.evaluate(interpreter)?;
        } else if let Some(elsecase) = &mut ifinput.elsecase {
            elsecase.evaluate(interpreter)?;
        }
        Ok(())
    }

    pub fn evaluate_while(wild: &mut WhileProps, interpreter: &mut Interpreter) -> LoxResult<()> {
        while {
            let cond = wild.condition.evaluate(interpreter)?;
            is_truthy(&cond)
        } {
            wild.statement.evaluate(interpreter)?;
        }
        Ok(())
    }
}

