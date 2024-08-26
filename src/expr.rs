use crate::token::Token;
use crate::literal::Literal;
use crate::error::Error;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>
    },
    Literal {
        literal: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> Result<R, Error> {
        match self {
            Expr::Binary { left, operator, right } => visitor.visit_binary_expr( left, operator, right ),
            Expr::Grouping { expression } => visitor.visit_grouping_expr( expression ),
            Expr::Literal { literal } => visitor.visit_literal_expr( literal ),
            Expr::Unary { operator, right } => visitor.visit_unary_expr( operator, right ),
        }
    }
}

pub trait Visitor<R> {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, Error>;
    fn visit_grouping_expr(&mut self, expression: &Expr ) -> Result<R, Error>;
    fn visit_literal_expr(&mut self, literal: &Literal ) -> Result<R, Error>;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<R, Error>;
}
