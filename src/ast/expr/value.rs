
/// 值 用于表示表达式的值
/// 例如 1, "hello", true
/// Number 表示数字
/// StringLiteral 表示字符串
/// Bool 表示布尔值
#[derive(Debug, Clone,PartialEq)]
pub enum Value {
    Number(i64),
    StringLiteral(String),
    Bool(bool),
    Never,
}
