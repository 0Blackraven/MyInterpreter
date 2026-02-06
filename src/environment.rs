
use crate::token::Literal;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    pub enclosing : Option<Box<Environment>>,
    variables : HashMap<String, Literal>
}

impl Environment {
    pub fn new (enclosing: Option<Box<Environment>>) -> Self {
        Self {
            enclosing: enclosing,
            variables: HashMap::new(),
        }
    }

    pub fn define (&mut self, name: String, value: Literal) {
        self.variables.insert(name, value);
    }

    pub fn get (&self , name:&String) -> Literal {
        let value_option = self.variables.get(name);
        match value_option {
            Some(value) => return value.to_owned(),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    return enclosing.get(name);
                } else {
                    panic!("not found variable get method {}", name);
                }
            }
        }
    }

    pub fn assign (&mut self, name:String, value: &Literal){
        let x_result = self.variables.get_mut(&name);
        match x_result {
            Some(x) => *x = value.clone(),
            None =>{
                if let Some(enclosing) = &mut self.enclosing {
                    return enclosing.assign(name, value);
                } else {
                    panic!("variable not defined {}", name);
                }
            }
        }
    }
}