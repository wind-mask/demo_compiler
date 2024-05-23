use crate::ast::expr::ExprEnum;

use super::StatementChunk;




#[derive(Debug, Clone)]
pub enum Err{}

/// 循环语句
/// `LoopStatementEnum` 枚举包含了所有可能的循环语句类型，包括：
/// - `LoopInArray`：数组循环。
/// - `LoopWhile`：条件循环。
#[derive(Debug, Clone)]
pub enum LoopStatementEnum<'input> {
    LoopInArray(LoopInArrayStatement<'input>),
    LoopWhile(LoopWhileStatement<'input>),
}
/// 循环语句
/// `LoopInArrayStatement` 结构体包含了一个循环语句的所有信息，包括：
/// - `i`：循环变量的名称。
/// - `array`：循环的数组。
/// - `body`：循环体。
#[derive(Debug, Clone)]
pub struct LoopInArrayStatement<'input> {
    pub i: &'input str,
    pub array: &'input str,
    pub body: StatementChunk<'input>,
}

/// 循环语句
/// `LoopWhileStatement` 结构体包含了一个循环语句的所有信息，包括：
/// - `condition`：循环的条件。
/// - `body`：循环体。
#[derive(Debug, Clone)]
pub struct LoopWhileStatement<'input> {
    pub condition: ExprEnum<'input>,
    pub body: StatementChunk<'input>,
}