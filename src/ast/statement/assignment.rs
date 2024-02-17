use crate::ast::expr::Expr;

pub struct Assignment {
    pub name: String,
    pub expr: Expr,
}
