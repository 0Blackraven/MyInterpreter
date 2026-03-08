use crate::lox_class::LoxClass;
use crate::loxfuncs::LoxFunction;
use crate::token::{Token,AtomicLiteral};
use std::collections::HashMap;
use std::rc::Rc;
use crate::expression::{ClassType, ExpressionType, FunctionType, is_truthy};
use crate::resolver::{Resolvable, Resolver};
use crate::lox_error::LoxResult;
use crate::interpreter::Interpreter;
use crate::token::Literal;
use crate::lox_error::LoxError;
use std::cell::RefCell;


// line 193

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
    ClassStatement(ClassProps),
}
#[derive(Clone)]
pub struct ClassProps {
    pub name : Token,
    pub methods : Vec<StatementType>,
    pub superclass : Option<ExpressionType>

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
        match self {
            StatementType::BlockStatement(statements) => {
                resolver.begin_scope();
                resolver.resolve(statements)?;
                resolver.end_scope();
            }

            StatementType::LetStatement(statement) => {
                resolver.declare(&statement.name)?;
                match *statement.initializer {
                    ExpressionType::Literal(AtomicLiteral::Nil) => {},
                    _ => {
                        resolver.resolve(&*statement.initializer)?;
                    }
                }
                resolver.define(&statement.name);
            }

            StatementType::Function(func) => {
                resolver.declare(&func.name)?;
                resolver.define(&func.name);
                resolver.resolve_function(func, FunctionType::Function)?;
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
                if resolver.current_function == FunctionType::None {
                    return Err(LoxError::RuntimeError {
                        token: Some(statement._keyword.clone()),
                        message: "Cannot return from top-level code.".to_string(),
                    });
                }
                if let Some(value) = &statement.value {
                    if resolver.current_function == FunctionType::Initializer {
                        return Err(LoxError::RuntimeError {
                            token: Some(statement._keyword.clone()),
                            message: "Cannot return from a constructor".to_string() 
                        })
                    }
                    resolver.resolve(value)?;
                }
            }

            StatementType::WhileStatement(statement) => {
                resolver.resolve(&statement.condition)?;
                resolver.resolve(&*statement.statement)?;
            }
            StatementType::ClassStatement(class_prop) => {
                let enclosing_class = resolver.current_class.clone();
                let mut declaration = FunctionType::Method;
                resolver.current_class = ClassType::Class;

                resolver.declare(&class_prop.name)?;
                resolver.define(&class_prop.name);
                resolver.begin_scope();
                {
                    let mut scopes = resolver.scopes.borrow_mut();
                    let scope_result = scopes.last_mut();
                    if let Some(scope) = scope_result {
                        scope.insert("this".to_string(), true);
                    }
                }
                for method in &class_prop.methods {
                    match method {
                        StatementType::Function(func) => {
                            if func.name.lexeme == "init".to_string() {
                                declaration = FunctionType::Initializer;
                            }
                            resolver.resolve_function(func, declaration.clone())?;
                        }
                        _ => {
                            return Err(LoxError::RuntimeError {
                                token: Some(class_prop.name.clone()),
                                message: "Only functions can be methods.".to_string(),
                            });
                        }
                    }
                }
                resolver.end_scope();
                resolver.current_class = enclosing_class;
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
                        expr.name.clone(),
                        Literal::Basic(AtomicLiteral::Nil),
                    )?;
                    Ok(())
                }
                _ => {
                    let result = expr.initializer.evaluate(interpreter)?;
                    interpreter.env
                        .borrow_mut()
                        .define(expr.name.clone(), result)?;
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
                let function = LoxFunction::new(Rc::new(func_props), interpreter, false);
                interpreter.env.borrow_mut().define(
                    func_props.name.clone(),
                    Literal::LoxCallable(Rc::new(function)),
                )?;
                Ok(())
            }
            StatementType::ReturnStatement(prop) => {
                let value = match &mut prop.value {
                    Some(expr) => expr.evaluate(interpreter)?,
                    None => Literal::Basic(AtomicLiteral::Nil),
                };
                Err(LoxError::ReturnValue(value))
            }
            StatementType::ClassStatement(class_prop) => {
                interpreter.env.borrow_mut().define(class_prop.name.clone(), Literal::Basic(AtomicLiteral::Nil))?;
                let mut methods = HashMap::new();
                for method in &class_prop.methods {
                    match method {
                        StatementType::Function(func) => {
                            let is_initializer = func.name.lexeme == "init".to_string();
                            let function = LoxFunction::new(Rc::new(func), interpreter, is_initializer);
                            methods.insert(func.name.lexeme.clone(),function)
                        },
                        _ => unreachable!(),
                    };
                }
                let class = LoxClass::new(class_prop.name.clone(),methods);
                interpreter.env.borrow_mut().assign(class_prop.name.clone(), Literal::LoxCallable(Rc::new(class)))?;
                Ok(())
            }
            // _ => Ok(()),
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

