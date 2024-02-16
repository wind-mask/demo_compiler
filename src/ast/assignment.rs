use super::{
    expr::Expr,
    types::{self, Type},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Err {
    IdentifierNotAllowed,
}

#[derive(Debug)]
pub enum Assignment {
    Var(Type, String, Box<Expr>),
    BoolVar(crate::ast::types::Type, String),
}

impl Assignment {
    pub fn get_type(&self) -> Type {
        match self {
            Assignment::Var(t, _, _) => *t,
            Assignment::BoolVar(b, _) => *b,
        }
    }
}
