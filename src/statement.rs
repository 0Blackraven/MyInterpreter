use crate::environment::Environment;
use crate::expression::{ClassType, ExpressionType, FunctionType, is_truthy};
use crate::interpreter::Interpreter;
use crate::lox_class::LoxClass;
use crate::lox_error::LoxError;
use crate::lox_error::LoxResult;
use crate::loxfuncs::LoxFunction;
use crate::resolver::{Resolvable, Resolver};
use crate::token::Literal;
use crate::token::{AtomicLiteral, Token, TokenType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
    pub name: Token,
    pub methods: Vec<StatementType>,
    pub superclass: Option<ExpressionType>,
}
#[derive(Clone)]
pub struct ReturnProps {
    pub _keyword: Token,
    pub value: Option<ExpressionType>,
}
#[derive(Clone)]
pub struct FunctionProps {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Rc<StatementType>,
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
                    ExpressionType::Literal(AtomicLiteral::Nil) => {}
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
                            message: "Cannot return from a constructor".to_string(),
                        });
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
                resolver.current_class = ClassType::Class;

                resolver.declare(&class_prop.name)?;
                resolver.define(&class_prop.name);

                if let Some(superclass) = &class_prop.superclass {
                    resolver.current_class = ClassType::SubClass;
                    resolver.resolve(superclass)?;

                    resolver.begin_scope();
                    let mut scopes = resolver.scopes.borrow_mut();
                    if let Some(scope) = scopes.last_mut() {
                        scope.insert("super".to_string(), true);
                    }
                }

                resolver.begin_scope();
                {
                    let mut scopes = resolver.scopes.borrow_mut();
                    if let Some(scope) = scopes.last_mut() {
                        scope.insert("this".to_string(), true);
                    }
                }

                for method in &class_prop.methods {
                    let mut declaration = FunctionType::Method;
                    if let StatementType::Function(func) = method {
                        if func.name.lexeme == "init" {
                            declaration = FunctionType::Initializer;
                        }
                        resolver.resolve_function(func, declaration)?;
                    }
                }

                resolver.end_scope(); 

                if class_prop.superclass.is_some() {
                    resolver.end_scope();
                }

                resolver.current_class = enclosing_class;
            } // _ => {}
        }
        Ok(())
    }
}

impl StatementType {
    pub fn evaluate(&self, interpreter: &mut Interpreter) -> LoxResult<()> {
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
                    interpreter
                        .env
                        .borrow_mut()
                        .define(expr.name.clone(), Literal::Basic(AtomicLiteral::Nil))?;
                    Ok(())
                }
                _ => {
                    let result = expr.initializer.evaluate(interpreter)?;
                    interpreter
                        .env
                        .borrow_mut()
                        .define(expr.name.clone(), result)?;
                    Ok(())
                }
            },
            StatementType::BlockStatement(statements) => {
                Self::evaluate_blocks(statements, interpreter)
            }
            StatementType::IfStatement(iftype) => Self::evaluate_if(iftype, interpreter),
            StatementType::WhileStatement(wild) => Self::evaluate_while(wild, interpreter),
            StatementType::Function(func_props) => {
                let function = LoxFunction::new(Rc::new(func_props), interpreter, false);
                interpreter.env.borrow_mut().define(
                    func_props.name.clone(),
                    Literal::LoxCallable(Rc::new(function)),
                )?;
                Ok(())
            }
            StatementType::ReturnStatement(prop) => {
                let value = match &prop.value {
                    Some(expr) => expr.evaluate(interpreter)?,
                    None => Literal::Basic(AtomicLiteral::Nil),
                };
                Err(LoxError::ReturnValue(value))
            }
            StatementType::ClassStatement(class_prop) => {
                let mut superclass = None;
                if let Some(result) = &class_prop.superclass {
                    superclass = Some(Rc::new(result.evaluate(interpreter)?.as_class()?));
                }
                interpreter
                    .env
                    .borrow_mut()
                    .define(class_prop.name.clone(), Literal::Basic(AtomicLiteral::Nil))?;
                let previous = Rc::clone(&interpreter.env);
                if let Some(superclass) = superclass.clone() {
                    let current = Environment::new(Some(previous.clone()));
                    interpreter.env = Rc::new(RefCell::new(current));
                    interpreter.env.borrow_mut().define(
                        Token::new(TokenType::SUPER, "super".to_string(), 0, AtomicLiteral::Nil),
                        Literal::LoxCallable(superclass),
                    )?;
                }
                let mut methods = HashMap::new();
                for method in &class_prop.methods {
                    match method {
                        StatementType::Function(func) => {
                            let is_initializer = func.name.lexeme == "init".to_string();
                            let function =
                                LoxFunction::new(Rc::new(func), interpreter, is_initializer);
                            methods.insert(func.name.lexeme.clone(), function)
                        }
                        _ => unreachable!(),
                    };
                }
                let class = LoxClass::new(class_prop.name.clone(), methods, superclass);
                if class_prop.superclass.is_some() {
                    interpreter.env = previous;
                }
                interpreter.env.borrow_mut().assign(
                    class_prop.name.clone(),
                    Literal::LoxCallable(Rc::new(class)),
                )?;
                Ok(())
            } // _ => Ok(()),
        }
    }

    pub fn evaluate_blocks(
        statements: &[StatementType],
        interpreter: &mut Interpreter,
    ) -> LoxResult<()> {
        let previous = Rc::clone(&interpreter.env);

        interpreter.env = Rc::new(RefCell::new(crate::environment::Environment::new(Some(
            previous.clone(),
        ))));

        for statement in statements {
            statement.evaluate(interpreter)?;
        }

        interpreter.env = previous;
        Ok(())
    }

    pub fn evaluate_func_block(
        statement: &StatementType,
        closure: Rc<RefCell<crate::environment::Environment>>,
        interpreter: &mut Interpreter,
    ) -> LoxResult<()> {
        let previous = Rc::clone(&interpreter.env);

        interpreter.env = closure;

        let result = statement.evaluate(interpreter);

        interpreter.env = previous;
        result
    }

    pub fn evaluate_if(ifinput: &IfProps, interpreter: &mut Interpreter) -> LoxResult<()> {
        let comparison = ifinput.comparison.evaluate(interpreter)?;

        if is_truthy(&comparison) {
            ifinput.ifcase.evaluate(interpreter)?;
        } else if let Some(elsecase) = &ifinput.elsecase {
            elsecase.evaluate(interpreter)?;
        }
        Ok(())
    }

    pub fn evaluate_while(wild: &WhileProps, interpreter: &mut Interpreter) -> LoxResult<()> {
        while {
            let cond = wild.condition.evaluate(interpreter)?;
            is_truthy(&cond)
        } {
            wild.statement.evaluate(interpreter)?;
        }
        Ok(())
    }
}
