use crate::{operators::Op, Number};

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub operator: Op,
}

impl Expr {
    pub fn new(s: &str) -> Self {
        let lhs = Number::new(s);
        let rhs = Number::new(s);
        let operator = Op::new(s);

        Self { lhs, rhs, operator }
    }
}
