pub mod base;
pub mod primitive;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Err {}

/// 类型
/// `Type` 枚举包含了所有可能的类型，包括：
/// - `Identifier`：标识符类型。
/// - `Primitive`：基本类型。
/// - `Base`：基本类型。
/// - `Array`：数组类型。
/// - `Void`：空类型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type<'input> {
    Identifier(&'input str),
    Primitive(primitive::Type),
    Base(base::Type),
    Array(Box<Type<'input>>, usize),
    Void,
}
