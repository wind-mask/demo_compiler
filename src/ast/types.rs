#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Err {
    WrongType,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Usize,
    Bool,
}
