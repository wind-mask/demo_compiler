use crate::ast::statement::declaration::Err;
use demo_isa::Inst;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Usize,
    F64,
}
impl Type {
    pub fn declaration(&self) -> Result<Vec<Inst>, Err> {
        match self {
            Type::Usize => {}
            Type::F64 => return Ok(vec![Inst::Nop]),
        }
        todo!() //TODO: implement declaration
    }
}
