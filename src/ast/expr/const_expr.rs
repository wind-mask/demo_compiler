use self::primitive::{PrimitiveExpr, UsizeExpr};

pub(crate) mod primitive;
#[derive(Debug, Clone)]
pub enum ConstExpr {
    Primitive(PrimitiveExpr),
}
