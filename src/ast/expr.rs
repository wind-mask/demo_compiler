use demo_isa::isa::{Inst, Reg, RegType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Err {
    UsizeTooLarge,
}
#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug)]
pub enum Expr {
    Num(usize),
    Op(Box<Expr>, Op, Box<Expr>),
}
impl Expr {
    pub fn eval_compile(
        &self,
        reg1: Reg,
        reg2: Reg,
        d_reg: Reg,
        code: &mut Vec<Inst>,
    ) -> Option<Err> {
        match self {
            Expr::Num(n) => {
                code.push(Inst::M(d_reg, RegType::Usize(*n)));
            }
            Expr::Op(l, op, r) => {
                l.eval_compile(d_reg, reg2, reg1, code);
                r.eval_compile(d_reg, reg2, reg2, code);
                match op {
                    Op::Add => {
                        code.push(Inst::AddU(d_reg, reg2, reg1));
                    }
                    Op::Sub => {
                        code.push(Inst::SubU(d_reg, reg2, reg1));
                    }
                    Op::Mul => {
                        code.push(Inst::MulU(d_reg, reg2, reg1));
                    }
                    Op::Div => {
                        code.push(Inst::DivU(d_reg, reg2, reg1));
                    }
                }
            }
        }
        None
    }
}
#[cfg(test)]
mod tests {
    use demo_isa::isa::ISAErr;

    use super::*;
    #[test]
    fn test_expr_eval_compile() {
        use lalrpop_util::lalrpop_mod;
        lalrpop_mod!(pub usizeparse,concat!("/",stringify!(usizeparse),".rs")); // synthesized by LALRPOP
        let expr: Box<Expr> = usizeparse::ExprParser::new()
            .parse("89*(64+29)*52")
            .unwrap();
        let mut code = vec![];
        if let Some(err) = expr.eval_compile(Reg::R1, Reg::R2, Reg::R3, &mut code) {
            panic!("error {:?}", err);
        };
    }
}
