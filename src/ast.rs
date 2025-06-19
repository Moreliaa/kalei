pub trait Expr {}

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
    op: String,
    lhs: Box<dyn Expr>,
    rhs: Box<dyn Expr>,
}

impl BinaryExprAst {
    pub fn new(op: String, lhs: Box<dyn Expr>, rhs: Box<dyn Expr>) -> BinaryExprAst {
        BinaryExprAst { op, lhs, rhs }
    }
}
impl Expr for BinaryExprAst {}
