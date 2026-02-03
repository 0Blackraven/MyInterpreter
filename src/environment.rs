use crate::token::Literal;
use std::collections::HashMap;

pub struct Environment {
    enclosing : Option<Box<Environment>>,
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

    pub fn get (& mut self , name:&String) -> Literal {
        let value_option = self.variables.get(name);
        match value_option {
            Some(value) => return value.to_owned(),
            None => panic!("Not defined variable")
        }
    }

    pub fn assign (&mut self, name:String, value: &Literal){
        let x_result = self.variables.get_mut(&name);
        match x_result {
            Some(x) => *x = value.clone(),
            None => panic!("variable not defined")
        }
    }
}