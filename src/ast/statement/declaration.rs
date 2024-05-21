
use crate::ast::{
    expr::ExprEnum,
    types::{self},
};

use super::StatementChunk;

#[derive(Debug, Clone)]
pub enum Err {}

/// 变量声明
/// `Declaration` 结构体包含了一个声明的所有信息，包括：
/// - `const_`：这个声明是否是一个常量。
/// - `mut_`：这个声明是否是可变的。
/// - `type_`：这个声明的类型。
/// - `name`：这个声明的名称。
/// - `expr`：这个声明的表达式。
#[derive(Debug, Clone)]
pub struct Declaration<'input> {
    pub const_: bool,
    pub mut_: bool,
    pub type_: types::Type<'input>,
    pub name: &'input str,
    pub expr: ExprEnum<'input>,
}

/// 函数声明
/// `FunctionDeclaration` 结构体包含了一个函数声明的所有信息，包括：
/// - `name`：这个函数的名称。
/// - `args`：这个函数的参数列表。
/// - `ret`：这个函数的返回值类型。
/// - `body`：这个函数的函数体。
#[derive(Debug, Clone)]
pub struct FunctionDeclaration<'input> {
    pub name: &'input str,
    pub args: Vec<(&'input str, types::Type<'input>)>,
    pub ret: types::Type<'input>,
    pub body: StatementChunk<'input>,
}
