use std::collections::{HashMap, HashSet};

use crate::ast::types::Type;

#[derive(Debug)]
pub struct SymbolTable {
    pub symbols: HashMap<String, SymbolsInfo>,
    pub parent: Option<Box<SymbolTable>>,
    pub children: HashSet<Box<SymbolTable>>,
}
#[derive(Debug)]
pub struct SymbolsInfo {
    pub name: String,
    pub type_: Type, //TODO: SymbolInfo
}
