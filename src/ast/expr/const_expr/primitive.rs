use crate::ast::expr::Op2;
use demo_isa::reg::Reg;
use demo_isa::{Inst, RegType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Err {
    UsizeTooLarge,
    F64ParseError,
}

#[derive(Debug, Clone)]
pub enum PrimitiveExpr {
    Usize(UsizeExpr),
    F64(F64Expr),
}
#[derive(Debug, Clone)]
pub enum UsizeExpr {
    Usize(usize),
    Op2(Box<UsizeExpr>, Op2, Box<UsizeExpr>),
}
impl UsizeExpr {
    pub fn eval_compile(
        &self,
        reg1: Reg,
        reg2: Reg,
        d_reg: Reg,
        code: &mut Vec<Inst>,
    ) -> Option<Err> {
        match self {
            UsizeExpr::Usize(n) => {
                code.push(Inst::M(d_reg, RegType::Usize(*n)));
            }
            UsizeExpr::Op2(l, op, r) => {
                l.eval_compile(d_reg, reg2, reg1, code);
                r.eval_compile(d_reg, reg2, reg2, code);
                match op {
                    Op2::Add => {
                        code.push(Inst::AddU(d_reg, reg2, reg1));
                    }
                    Op2::Sub => {
                        code.push(Inst::SubU(d_reg, reg2, reg1));
                    }
                    Op2::Mul => {
                        code.push(Inst::MulU(d_reg, reg2, reg1));
                    }
                    Op2::Div => {
                        code.push(Inst::DivU(d_reg, reg2, reg1));
                    }
                }
            }
        }
        None
    }
}
#[derive(Debug, Clone)]
pub enum F64Expr {
    F64(f64),
    Op2(Box<F64Expr>, Op2, Box<F64Expr>),
}

#[cfg(test)]
mod test {
    #[test]
    fn test_usize_expr() {
        use lalrpop_util::lalrpop_mod;
        lalrpop_mod!(pub primitive_expr,"/ast/expr/const_expr/primitive_expr.rs"); // synthesized by LALRPOP
        let expr = primitive_expr::UsizeExprParser::new()
            .parse("22+33*55")
            .unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "Op2(Usize(22), Add, Op2(Usize(33), Mul, Usize(55)))"
        );
        let expr = primitive_expr::UsizeExprParser::new()
            .parse("(22+33)*55+66")
            .unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "Op2(Op2(Op2(Usize(22), Add, Usize(33)), Mul, Usize(55)), Add, Usize(66))"
        );
        assert!(primitive_expr::UsizeExprParser::new()
            .parse(&usize::MAX.to_string())
            .is_ok());
        assert!(primitive_expr::UsizeExprParser::new()
            .parse(&(usize::MAX.to_string() + "1"))
            .is_err());
    }

    #[test]
    fn test_f64_expr() {
        use lalrpop_util::lalrpop_mod;
        lalrpop_mod!(pub primitive_expr,"/ast/expr/const_expr/primitive_expr.rs"); // synthesized by LALRPOP
        let parser = primitive_expr::F64ExprParser::new();
        let expr = parser.parse("2.2+3.3*5.5").unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "Op2(F64(2.2), Add, Op2(F64(3.3), Mul, F64(5.5)))"
        );
        let expr = parser.parse("(2.2+3.3)*5.5+6.6").unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "Op2(Op2(Op2(F64(2.2), Add, F64(3.3)), Mul, F64(5.5)), Add, F64(6.6))"
        );
        assert!(parser.parse("-1e10").is_ok());
        assert!(parser.parse("-3.14").is_ok());
        assert!(parser.parse("2.5E-10").is_ok());
        assert!(parser.parse("0.5").is_ok());
        assert!(parser.parse("NaN").is_ok());
        assert!(parser.parse("-Infinity").is_ok());
        assert!(parser.parse("Infinity").is_ok());
        assert!(primitive_expr::F64ExprParser::new()
            .parse(&(f64::MAX.to_string() + "1"))
            .is_err());
    }
}
