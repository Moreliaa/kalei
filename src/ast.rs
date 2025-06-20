use crate::treeprinter::*;

pub trait Expr {
    fn print(&self, treeprinter: &mut TreePrinter, indent_lvl: i32, depth: i32);
}

pub struct NumberExprAst {
    val: f64,
}

impl NumberExprAst {
    pub fn new(val: f64) -> NumberExprAst {
        NumberExprAst { val }
    }
}

impl Expr for NumberExprAst {
    fn print(&self, treeprinter: &mut TreePrinter, indent_lvl: i32, depth: i32) {
        treeprinter.add_print_item(self.val.to_string(), depth, indent_lvl);
    }
}

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
impl Expr for BinaryExprAst {
    fn print(&self, treeprinter: &mut TreePrinter, indent_lvl: i32, depth: i32) {
        treeprinter.add_print_item(self.op.clone(), depth, indent_lvl);

        self.lhs.print(treeprinter, indent_lvl - 1, depth + 1);
        self.rhs.print(treeprinter, indent_lvl + 1, depth + 1);
    }
}
