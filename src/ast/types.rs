pub mod primitive;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Err {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Primitive(primitive::Type),
    Array(Box<Type>, usize),
}
