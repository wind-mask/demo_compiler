use demo_isa::Inst;

use crate::ast::{
    expr::const_expr::primitive::{PrimitiveExpr, UsizeExpr},
    types::{self, primitive::Type},
};

trait DeclarationProcess {
    fn declaration_process(&self) -> Result<Vec<Inst>, Err>; //TODO: declaration_process and its SymbolTable
}
pub enum Err {}

pub struct Declaration {
    pub type_: types::Type,
    pub name: String,
}
impl Declaration {
    pub fn declaration_compile(&self) -> Result<Vec<Inst>, Err> {
        match &self.type_ {
            types::Type::Primitive(t) => match t {
                Type::Usize => return Ok(vec![Inst::Nop]),
                Type::F64 => return Ok(vec![Inst::Nop]),
            },
            types::Type::Array(t, usize) => {}
        };
        todo!()
    }
}
