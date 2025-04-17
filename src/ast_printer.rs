use crate::{expr::*, token::LiteralKind};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&mut self, expr: Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: String, exprs: &[Expr]) -> String {
        let mut string = String::new();
        string.push_str("(");
        string.push_str(&name);
        for expr in exprs.iter() {
            string.push(' ');
            let expression = expr.accept(self);
            string.push_str(&expression);
        }
        string.push_str(")");
        string
    }
}

impl ExpressionVisitor<String> for AstPrinter {
    fn visit_assignment(&mut self, _expr: &Assignment) -> String {
        todo!()
    }

    fn visit_binary(&mut self, expr: &Binary) -> String {
        self.parenthesize(
            expr.operator.lexeme.clone(),
            &[*expr.left.clone(), *expr.right.clone()],
        )
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> String {
        self.parenthesize("group".to_owned(), &[*expr.expr.clone()])
    }

    fn visit_literal(&self, expr: &Literal) -> String {
        if expr.value == LiteralKind::Nil {
            return "nil".to_string();
        }
        String::from(expr.value.clone())
    }

    fn visit_logical(&mut self, _expr: &Logical) -> String {
        todo!()
    }

    fn visit_unary(&mut self, expr: &Unary) -> String {
        self.parenthesize(expr.operator.lexeme.clone(), &[*expr.right.clone()])
    }

    fn visit_variable(&mut self, _expr: &Variable) -> String {
        todo!()
    }

    fn visit_call(&mut self, _expr: &Call) -> String {
        todo!()
    }

    fn visit_get(&mut self, _expr: &Get) -> String {
        todo!()
    }

    fn visit_set(&mut self, _expr: &Set) -> String {
        todo!()
    }

    fn visit_this(&mut self, _expr: &This) -> String {
        todo!()
    }

    fn visit_super(&mut self, _expr: &Super) -> String {
        todo!()
    }
}
