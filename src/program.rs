

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub statement_parse,"/ast/statement_parse.rs"); 
pub fn compile_2_ast(input: &str) -> crate::ast::statement::StatementChunk {
    use crate::{ast::statement, lexer::Lexer_};
    let le = Lexer_::new(&input);
    let parser = statement_parse::StatementChunkParser::new();
    let ast: statement::StatementChunk = parser.parse(&input, le).unwrap();
    ast
}
#[cfg(test)]
#[test]
fn test_st() {
    use lalrpop_util::lalrpop_mod;

    use crate::{ast::statement, lexer::Lexer_};
    // let x = "xx";
    lalrpop_mod!(pub statement_parse,"/ast/statement_parse.rs"); // synthesized by LALRPOP
    let path = std::path::Path::new("R:\\CODE\\Compile_demo\\demo_compiler\\src\\test\\src.dsrc");
    let input = std::fs::read_to_string(path).unwrap();
    // let input="main(1,2);";
    let le = Lexer_::new(&input);
    let parser = statement_parse::StatementChunkParser::new();
    let ast: statement::StatementChunk = parser.parse(&input, le).unwrap();
    println!("{:?}", ast);
    // let ex=statement::declaration::Declaration{
    //     const_: false,
    //     mut_: false,
    //     type_: types::Type::Base(types::base::Type::I64),
    //     name: "x".to_string(),
    //     expr: expr::Expr{
    //         type_: types::Type::Base(types::base::Type::I64),
    //         expr: expr::ExprEnum::Value(expr::value::Value::Number(3)),
    //     },
    // };
}
