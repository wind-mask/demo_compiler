use self::primitive::{PrimitiveExpr, UsizeExpr};

pub(crate) mod primitive;
pub enum ConstExpr {
    Primitive(PrimitiveExpr),
}
