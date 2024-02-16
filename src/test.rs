#[cfg(test)]
mod test {

    use lalrpop_util::{lexer::Token, ParseError};

    use crate::ast::{assignment::Assignment, expr::Err, types};

    #[test]
    fn test_expr() {
        use lalrpop_util::lalrpop_mod;
        lalrpop_mod!(pub usizeparse,concat!("/",stringify!(usizeparse),".rs")); // synthesized by LALRPOP
        let expr = usizeparse::ExprParser::new().parse("22+33*55").unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "Op(Num(22), Add, Op(Num(33), Mul, Num(55)))"
        );
        let expr = usizeparse::ExprParser::new()
            .parse("(22+33)*55+66")
            .unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "Op(Op(Op(Num(22), Add, Num(33)), Mul, Num(55)), Add, Num(66))"
        );
        assert!(usizeparse::ExprParser::new()
            .parse(&usize::MAX.to_string())
            .is_ok());
        assert!(usizeparse::ExprParser::new()
            .parse(&(usize::MAX.to_string() + "1"))
            .is_err());
    }
    #[test]
    fn test_assignment() {
        use lalrpop_util::lalrpop_mod;
        lalrpop_mod!(pub usizeparse,concat!("/",stringify!(usizeparse),".rs")); // synthesized by LALRPOP
        let assignment = usizeparse::AssignmentParser::new()
            .parse("usize x = 22+33*55")
            .unwrap();
        let assignment = *assignment;
        match assignment {
            Assignment::Var(t, s, e) => {
                assert_eq!(t, types::Type::Usize);
                assert_eq!(s, "x");
            }
            _ => panic!("wrong type"),
        }
    }
}
