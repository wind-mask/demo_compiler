
use super::{expr, types::Type};
pub mod declaration;
pub mod loop_statement;
#[derive(Debug, Clone)]
pub enum StatementError {}

/// 各类语句
/// `StatementEnum` 枚举包含了所有可能的语句类型，包括：
/// - `Declaration`：变量声明。
/// - `Assignment`：赋值语句。
/// - `FunctionDeclaration`：函数声明。
/// - `ExprCall`：函数调用。
/// - `ExprStatement`：表达式语句。
/// - `StatementChunk`：代码块。
#[derive(Debug, Clone)]
pub enum StatementEnum<'input> {
    /// 变量声明
    Declaration(declaration::Declaration<'input>),
    /// 函数声明
    FunctionDeclaration(declaration::FunctionDeclaration<'input>),
    // // 函数调用
    // ExprCall(expr::ExprEnum<'input>),
    /// 表达式语句
    ExprStatement(expr::ExprEnum<'input>),
    /// 代码块
    // StatementChunk(StatementChunk<'input>),
    /// 循环语句
    LoopStatement(loop_statement::LoopStatementEnum<'input>),
}

/// 代码块
/// `StatementChunk` 结构体包含了一个代码块的所有信息，包括：
/// - `statements`：这个代码块的所有语句。
/// - `type_`：这个代码块的返回值类型。
#[derive(Debug, Clone)]
pub struct StatementChunk<'input> {
    /// 代码块的语句
    pub statements: Vec<StatementEnum<'input>>,
    pub type_: Type<'input>,
}
