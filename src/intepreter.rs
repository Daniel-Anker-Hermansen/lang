use crate::parser::*;

type Val = isize;

pub fn eval(exp: Ast) -> Val {
    match exp {
        Ast::Literal { value } => value,
        Ast::BinOp { leftexp, op, rightexp } => {
            let leftval = eval(*leftexp);
            let rightval = eval(*rightexp);
            match op {
                BinOp::Plus => leftval + rightval,
                BinOp::Minus => leftval - rightval,
                BinOp::Mult => leftval * rightval,
                BinOp::Div => leftval / rightval,
                BinOp::Modulo => ((leftval % rightval) + rightval) % rightval
            }
        }
        Ast::UnOp { op, exp } => {
            let val = eval(*exp);
            match op {
                UnOp::Neg =>  - val
            }
        }
    }
}
