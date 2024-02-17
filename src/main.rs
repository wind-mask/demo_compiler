#![allow(unused, unnameable_test_items)]
pub mod ast;
pub mod lexer;
pub mod name_space;
pub mod pre_processor;
mod test;
pub fn main() {}
use crate::ast::statement::{StatementChunk, StatementEnum, StatementError};
fn x() {
    Box::new(StatementEnum::StatementChunk(StatementChunk {
        statements: vec![],
    }));
}
