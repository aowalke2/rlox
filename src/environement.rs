use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    interpreter::Exit,
    report,
    token::{LiteralKind, Token},
};

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, LiteralKind>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralKind) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<LiteralKind, Exit> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else if self.enclosing.is_some() {
            Ok(self.enclosing.as_ref().unwrap().borrow().get(name)?)
        } else {
            report(name.line, &format!("Undefined variable '{}'.", name.lexeme));
            Err(Exit::RuntimeError)
        }
    }

    pub fn assign(&mut self, name: &Token, value: LiteralKind) -> Result<(), Exit> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)?;
            Ok(())
        } else {
            report(name.line, &format!("Undefined variable '{}'.", name.lexeme));
            Err(Exit::RuntimeError)
        }
    }
}
