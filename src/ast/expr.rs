use super::types;

pub mod value;

/// 一元运算符
/// `Op1` 枚举包含了所有可能的一元运算符，包括：
/// - `Neg`：负号。
/// - `Not`：逻辑非。
#[derive(Debug, Clone,Copy)]
pub enum Op1 {
    Neg,
    Not,
}
/// 二元运算符
/// `Op2` 枚举包含了所有可能的二元运算符，包括：
/// - `Add`：加法。
/// - `Sub`：减法。
/// - `Mul`：乘法。
/// - `Div`：除法。
/// - `And`：逻辑与。
/// - `BitAnd`：按位与。
/// - `Or`：逻辑或。
/// - `BitOr`：按位或。
/// - `Xor`：按位异或。
/// - `Mod`：取模。
/// - `Eq`：等于。
/// - `Ne`：不等于。
/// - `Lt`：小于。
/// - `Le`：小于等于。
/// - `Gt`：大于。
/// - `Ge`：大于等于。
/// - `Assign`：赋值。
/// - `AddAssign`：加法赋值。
/// - `SubAssign`：减法赋值。
/// - `MulAssign`：乘法赋值。
/// - `DivAssign`：除法赋值。
/// - `ModAssign`：取模赋值。
/// - `BitAndAssign`：按位与赋值。
/// - `BitOrAssign`：按位或赋值。
/// - `XorAssign`：按位异或赋值。
/// - `AndAssign`：逻辑与赋值。
/// - `OrAssign`：逻辑或赋值。
#[derive(Debug, Clone,Copy)]
pub enum Op2 {
    Add,
    Sub,
    Mul,
    Div,
    And,
    BitAnd,
    Or,
    BitOr,
    Xor,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    BitAndAssign,
    BitOrAssign,
    XorAssign,
    AndAssign,
    OrAssign,
    // Shl,
    // Shr,
}

/// 表达式
/// `Expr` 结构体包含了一个表达式的所有信息，包括：
/// - `type_`：这个表达式的类型。
/// - `expr`：这个表达式的内容。
#[derive(Debug, Clone)]
pub struct Expr<'input> {
    pub type_: types::Type<'input>,
    pub expr: ExprEnum<'input>,
}

/// 表达式内容
/// `ExprEnum` 枚举包含了所有可能的表达式类型，包括：
/// - `Identifier`：标识符。
/// - `Op2`：二元运算表达式。
/// - `Value`：值。
/// - `FunctionCall`：函数调用。
/// - `StatementChunk`：代码块。
#[derive(Debug, Clone)]
pub enum ExprEnum<'input> {
    Identifier(&'input str),
    Op1(Op1, Box<ExprEnum<'input>>),
    Op2(Box<ExprEnum<'input>>, Op2, Box<ExprEnum<'input>>),
    Value(value::Value),
    FunctionCall(Box<ExprEnum<'input>>, Vec<ExprEnum<'input>>),
    StatementChunk(super::statement::StatementChunk<'input>),
}
