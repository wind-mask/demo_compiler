use self::const_expr::ConstExpr;

use super::types;

pub mod const_expr;
pub mod identity_expr;

#[derive(Debug, Clone)]
pub enum Op2 {
    Add,
    Sub,
    Mul,
    Div,
    // And,
    // Or,
    // Xor,
    // Shl,
    // Shr,
}
#[derive(Debug, Clone)]
pub struct Expr {
    pub type_: types::Type,
    pub expr: ExprEnum,
}

#[derive(Debug, Clone)]
pub enum ExprEnum {
    ConstExpr(ConstExpr),
}
