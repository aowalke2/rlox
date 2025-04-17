use std::{cell::RefCell, rc::Rc};

use crate::{
    environement::Environment,
    expr::{self, Expr, ExpressionVisitor, Literal},
    report,
    stmt::{self, StatementVisitor, Stmt},
    token::{LiteralKind, Token, TokenKind},
};

pub enum Exit {
    RuntimeError,
    Return(LiteralKind),
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), Exit> {
        let mut has_error = false;
        for statement in statements.iter() {
            match self.execute(statement) {
                Ok(_) => (),
                Err(e) => {
                    if let Exit::RuntimeError = e {
                        has_error = true;
                        break;
                    }
                }
            }
        }

        match has_error {
            true => Err(Exit::RuntimeError),
            false => Ok(()),
        }
    }

    pub fn interpret_expression(&mut self, expr: &Expr) -> Result<String, Exit> {
        match self.evaluate(&expr) {
            Ok(literal) => Ok(self.stringify(literal)),
            Err(exit) => match exit {
                Exit::RuntimeError => Err(Exit::RuntimeError),
                Exit::Return(_literal_kind) => todo!(),
            },
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), Exit> {
        stmt.accept(self)
    }

    fn stringify(&self, literal: LiteralKind) -> String {
        match literal {
            LiteralKind::Nil => "nil".to_string(),
            LiteralKind::Number(num) => {
                let mut text = num.to_string();
                if text.ends_with(".0") {
                    text = text[0..text.len() - 2].to_string();
                }
                text
            }
            LiteralKind::String(s) => s.to_string(),
            LiteralKind::Bool(b) => b.to_string(),
        }
    }

    fn evaluate(&mut self, expr: &expr::Expr) -> Result<LiteralKind, Exit> {
        expr.accept(self)
    }

    fn is_truthy(&self, literal: &LiteralKind) -> bool {
        match literal {
            LiteralKind::Bool(boolean) => *boolean,
            LiteralKind::Nil => false,
            _ => true,
        }
    }

    fn is_equal(&self, a: LiteralKind, b: LiteralKind) -> bool {
        if a == LiteralKind::Nil && b == LiteralKind::Nil {
            return true;
        }
        if a == LiteralKind::Nil {
            return false;
        }

        match (a, b) {
            (LiteralKind::Number(a), LiteralKind::Number(b)) => a == b,
            (LiteralKind::String(a), LiteralKind::String(b)) => a == b,
            (LiteralKind::Bool(a), LiteralKind::Bool(b)) => a == b,
            _ => false,
        }
    }

    pub fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), Exit> {
        let previous = Rc::clone(&self.environment);
        self.environment = Rc::new(RefCell::new(environment));
        let result = statements.iter().try_for_each(|stat| self.execute(stat));
        self.environment = previous;
        result
    }
}

impl ExpressionVisitor<Result<LiteralKind, Exit>> for Interpreter {
    fn visit_assignment(&mut self, expr: &expr::Assignment) -> Result<LiteralKind, Exit> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_binary(&mut self, expr: &expr::Binary) -> Result<LiteralKind, Exit> {
        let right = self.evaluate(&expr.right)?;
        let left = self.evaluate(&expr.left)?;
        match expr.operator.kind {
            TokenKind::Minus => {
                if let (LiteralKind::Number(left), LiteralKind::Number(right)) = (left, right) {
                    Ok(LiteralKind::Number(left - right))
                } else {
                    report(expr.operator.line, "Operands must be numbers.");
                    Err(Exit::RuntimeError)
                }
            }
            TokenKind::Slash => {
                if let (LiteralKind::Number(left), LiteralKind::Number(right)) = (left, right) {
                    Ok(LiteralKind::Number(left / right))
                } else {
                    report(expr.operator.line, "Operands must be numbers.");
                    Err(Exit::RuntimeError)
                }
            }
            TokenKind::Star => {
                if let (LiteralKind::Number(left), LiteralKind::Number(right)) = (left, right) {
                    Ok(LiteralKind::Number(left * right))
                } else {
                    report(expr.operator.line, "Operands must be numbers.");
                    Err(Exit::RuntimeError)
                }
            }
            TokenKind::Plus => match (left, right) {
                (LiteralKind::Number(left), LiteralKind::Number(right)) => {
                    Ok(LiteralKind::Number(left + right))
                }
                (LiteralKind::String(left), LiteralKind::String(right)) => {
                    Ok(LiteralKind::String(format!("{left}{right}")))
                }
                _ => {
                    report(
                        expr.operator.line,
                        "Operands must be two numbers or two strings.",
                    );
                    Err(Exit::RuntimeError)
                }
            },
            TokenKind::Greater => {
                if let (LiteralKind::Number(left), LiteralKind::Number(right)) = (left, right) {
                    Ok(LiteralKind::Bool(left > right))
                } else {
                    report(expr.operator.line, "Operands must be numbers.");
                    Err(Exit::RuntimeError)
                }
            }
            TokenKind::GreaterEqual => {
                if let (LiteralKind::Number(left), LiteralKind::Number(right)) = (left, right) {
                    Ok(LiteralKind::Bool(left >= right))
                } else {
                    report(expr.operator.line, "Operands must be numbers.");
                    Err(Exit::RuntimeError)
                }
            }
            TokenKind::Less => {
                if let (LiteralKind::Number(left), LiteralKind::Number(right)) = (left, right) {
                    Ok(LiteralKind::Bool(left < right))
                } else {
                    report(expr.operator.line, "Operands must be numbers.");
                    Err(Exit::RuntimeError)
                }
            }
            TokenKind::LessEqual => {
                if let (LiteralKind::Number(left), LiteralKind::Number(right)) = (left, right) {
                    Ok(LiteralKind::Bool(left <= right))
                } else {
                    report(expr.operator.line, "Operands must be numbers.");
                    Err(Exit::RuntimeError)
                }
            }
            TokenKind::BangEqual => Ok(LiteralKind::Bool(!self.is_equal(left, right))),
            TokenKind::EqualEqual => Ok(LiteralKind::Bool(self.is_equal(left, right))),
            _ => unreachable!(),
        }
    }

    fn visit_grouping(&mut self, expr: &expr::Grouping) -> Result<LiteralKind, Exit> {
        self.evaluate(&expr.expr)
    }

    fn visit_literal(&self, expr: &expr::Literal) -> Result<LiteralKind, Exit> {
        Ok(expr.value.clone())
    }

    fn visit_logical(&mut self, expr: &expr::Logical) -> Result<LiteralKind, Exit> {
        let left = self.evaluate(&expr.left)?;
        if expr.operator.kind == TokenKind::Or {
            if self.is_truthy(&left) {
                return Ok(left);
            };
        } else if !self.is_truthy(&left) {
            return Ok(left);
        }

        self.evaluate(&expr.right)
    }

    fn visit_unary(&mut self, expr: &expr::Unary) -> Result<LiteralKind, Exit> {
        let right = self.evaluate(&expr.right)?;
        match expr.operator.kind {
            TokenKind::Minus => match right {
                LiteralKind::Number(number) => Ok(LiteralKind::Number(-number)),
                _ => {
                    report(expr.operator.line, "Operand must be a number.");
                    Err(Exit::RuntimeError)
                }
            },
            TokenKind::Bang => Ok(LiteralKind::Bool(!self.is_truthy(&right))),
            _ => unreachable!(),
        }
    }

    fn visit_variable(&mut self, expr: &expr::Variable) -> Result<LiteralKind, Exit> {
        self.environment.borrow().get(&expr.name)
    }

    fn visit_call(&mut self, expr: &expr::Call) -> Result<LiteralKind, Exit> {
        todo!()
    }

    fn visit_get(&mut self, expr: &expr::Get) -> Result<LiteralKind, Exit> {
        todo!()
    }

    fn visit_set(&mut self, expr: &expr::Set) -> Result<LiteralKind, Exit> {
        todo!()
    }

    fn visit_this(&mut self, expr: &expr::This) -> Result<LiteralKind, Exit> {
        todo!()
    }

    fn visit_super(&mut self, expr: &expr::Super) -> Result<LiteralKind, Exit> {
        todo!()
    }
}

impl StatementVisitor<Result<(), Exit>> for Interpreter {
    fn visit_expression(&mut self, stmt: &stmt::Expression) -> Result<(), Exit> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print(&mut self, stmt: &stmt::Print) -> Result<(), Exit> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", self.stringify(value));
        Ok(())
    }

    fn visit_var(&mut self, stmt: &stmt::Var) -> Result<(), Exit> {
        let value = if let Expr::Literal(Literal {
            value: LiteralKind::Nil,
        }) = *stmt.initializer
        {
            self.evaluate(&stmt.initializer)?
        } else {
            self.evaluate(&stmt.initializer)?
        };

        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block(&mut self, stmt: &stmt::Block) -> Result<(), Exit> {
        self.execute_block(
            &stmt.statements,
            Environment::new_with_enclosing(self.environment.clone()),
        )?;
        Ok(())
    }

    fn visit_if(&mut self, stmt: &stmt::If) -> Result<(), Exit> {
        let literal = self.evaluate(&stmt.condition)?;
        if self.is_truthy(&literal) {
            self.execute(&stmt.then_branch)?;
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(&else_branch)?;
        }

        Ok(())
    }

    fn visit_while(&mut self, stmt: &stmt::While) -> Result<(), Exit> {
        loop {
            let literal = self.evaluate(&stmt.condition)?;
            if !self.is_truthy(&literal) {
                break;
            }
            self.execute(&stmt.body)?;
        }

        Ok(())
    }

    fn visit_function(&mut self, stmt: &stmt::Function) -> Result<(), Exit> {
        todo!()
    }

    fn visit_return(&mut self, stmt: &stmt::Return) -> Result<(), Exit> {
        todo!()
    }

    fn visit_class(&mut self, stmt: &stmt::Class) -> Result<(), Exit> {
        todo!()
    }
}
