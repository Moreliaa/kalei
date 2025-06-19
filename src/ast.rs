use std::collections::HashMap;

pub trait Expr {}

struct BinOpPrecedence {
    mapping: HashMap<char, u8>,
}

fn init_bin_op() -> HashMap<char, u8> {
    let map = HashMap::new();
    return map;
}

pub struct NumberExprAst {
    val: f64,
}

impl NumberExprAst {
    pub fn new(val: f64) -> NumberExprAst {
        NumberExprAst { val }
    }
}

impl Expr for NumberExprAst {}

pub struct BinaryExprAst {
    op: char,
    lhs: Box<dyn Expr>,
    rhs: Box<dyn Expr>,
}

impl BinaryExprAst {
    pub fn new(op: char, lhs: Box<dyn Expr>, rhs: Box<dyn Expr>) -> BinaryExprAst {
        BinaryExprAst { op, lhs, rhs }
    }
}
impl Expr for BinaryExprAst {}
