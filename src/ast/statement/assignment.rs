use crate::ast::expr::Expr;

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: String,
    pub expr: Expr,
}
