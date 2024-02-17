use crate::name_space::SymbolTable;
pub mod assignment;
pub mod declaration;

#[derive(Debug, Clone)]
pub enum StatementError {}
#[derive(Debug, Clone)]
pub enum StatementEnum {
    Declaration(declaration::Declaration),
    Assignment(assignment::Assignment),
    StatementChunk(StatementChunk),
}

#[derive(Debug, Clone)]
pub struct StatementChunk {
    pub statements: Vec<StatementEnum>,
}
