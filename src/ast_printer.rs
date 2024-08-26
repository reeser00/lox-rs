use crate::token::Token;
use crate::literal::Literal;
use crate::expr::{Expr, Visitor};
use crate::error::Error;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: Expr) -> Result<String, Error> {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: String, expressions: Vec<&Expr>) -> Result<String, Error> {
        let mut result = String::new();
        result.push('(');
        result.push_str(&name);
        for expr in expressions {
            result.push(' ');
            result.push_str(&expr.accept(self)?);
        }
        result.push(')');
        Ok(result)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<String, Error> {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }

    fn visit_grouping_expr(&mut self, expression: &Expr ) -> Result<String, Error> {
        self.parenthesize(String::from("group"), vec![expression])
    }

    fn visit_literal_expr(&mut self, literal: &Literal ) -> Result<String, Error> {
        Ok(literal.to_string())
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<String, Error> {
        self.parenthesize(operator.lexeme.clone(), vec![right])
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;
    use crate::token_type::TokenType;
    use crate::ast_printer::AstPrinter;
    use crate::literal::Literal;
    use crate::expr::Expr;
    
    #[test]
    fn test_ast_printer() {
        let expression: Expr = Expr::Binary {
            left: Box::new(Expr::Unary { operator: Token::from( TokenType::Minus, String::from("-"), Literal::Empty, 1), right: Box::new(Expr::Literal { literal: Literal::Number(123f64),}) }),
            operator: Token::from( TokenType::Star, String::from("*"), Literal::Empty, 1),
            right: Box::new(Expr::Grouping { expression: Box::new(Expr::Literal { literal: Literal::Number(45.67f64), }), }),
        };

        let mut printer = AstPrinter;

        assert_eq!(printer.print(expression).unwrap(), "(* (- 123) (group 45.67))".to_string());
    }
}
