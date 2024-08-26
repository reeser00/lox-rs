#[derive(Debug, Clone)]
pub enum Literal{
    String(String),
    Number(f64),
    Empty,
    Boolean(bool),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Empty => write!(f, "null"),
            Literal::Boolean(b) => write!(f, "{}", b),
        }
    }
}
