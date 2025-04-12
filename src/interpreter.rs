use crate::{
    expr::{self, *},
    report,
    token::{LiteralKind, TokenKind},
};

pub enum Exit {
    RuntimeError,
    Return(LiteralKind),
}

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&mut self, expr: Expr) -> Result<String, Exit> {
        match self.evaluate(&expr) {
            Ok(literal) => Ok(self.stringify(literal)),
            Err(exit) => match exit {
                Exit::RuntimeError => Err(Exit::RuntimeError),
                Exit::Return(_literal_kind) => todo!(),
            },
        }
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

    fn evaluate(&mut self, expr: &Expr) -> Result<LiteralKind, Exit> {
        expr.accept(self)
    }

    fn is_truthy(&self, literal: LiteralKind) -> bool {
        match literal {
            LiteralKind::Bool(boolean) => boolean,
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
}

impl ExpressionVisitor<Result<LiteralKind, Exit>> for Interpreter {
    fn visit_assignment(&mut self, expr: &Assignment) -> Result<LiteralKind, Exit> {
        todo!()
    }

    fn visit_binary(&mut self, expr: &Binary) -> Result<LiteralKind, Exit> {
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

    fn visit_grouping(&mut self, expr: &Grouping) -> Result<LiteralKind, Exit> {
        self.evaluate(&expr.expr)
    }

    fn visit_literal(&self, expr: &Literal) -> Result<LiteralKind, Exit> {
        Ok(expr.value.clone())
    }

    fn visit_logical(&mut self, expr: &Logical) -> Result<LiteralKind, Exit> {
        todo!()
    }

    fn visit_unary(&mut self, expr: &Unary) -> Result<LiteralKind, Exit> {
        let right = self.evaluate(&expr.right)?;
        match expr.operator.kind {
            TokenKind::Minus => match right {
                LiteralKind::Number(number) => Ok(LiteralKind::Number(-number)),
                _ => {
                    report(expr.operator.line, "Operand must be a number.");
                    Err(Exit::RuntimeError)
                }
            },
            TokenKind::Bang => Ok(LiteralKind::Bool(!self.is_truthy(right))),
            _ => unreachable!(),
        }
    }

    fn visit_variable(&mut self, expr: &Variable) -> Result<LiteralKind, Exit> {
        todo!()
    }

    fn visit_call(&mut self, expr: &Call) -> Result<LiteralKind, Exit> {
        todo!()
    }

    fn visit_get(&mut self, expr: &Get) -> Result<LiteralKind, Exit> {
        todo!()
    }

    fn visit_set(&mut self, expr: &Set) -> Result<LiteralKind, Exit> {
        todo!()
    }

    fn visit_this(&mut self, expr: &This) -> Result<LiteralKind, Exit> {
        todo!()
    }

    fn visit_super(&mut self, expr: &Super) -> Result<LiteralKind, Exit> {
        todo!()
    }
}
