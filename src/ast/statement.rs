pub mod assignment;
pub mod declaration;
pub enum StatementEnum {
    Declaration(declaration::Declaration),
    Assignment(assignment::Assignment),
}
