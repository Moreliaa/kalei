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

        let indent_lvl_lhs = match &self.lhs {
            BinaryExprAst => indent_lvl - 2,
            _ => indent_lvl - 1,
        };
        let indent_lvl_rhs = match &self.rhs {
            BinaryExprAst => indent_lvl + 2,
            _ => indent_lvl + 1,
        };
        self.lhs.print(treeprinter, indent_lvl_lhs, depth + 1);
        self.rhs.print(treeprinter, indent_lvl_rhs, depth + 1);
    }
}

pub struct VariableExprAst {
    name: String,
}
impl VariableExprAst {
    pub fn new(name: String) -> VariableExprAst {
        VariableExprAst { name }
    }
}
impl Expr for VariableExprAst {
    fn print(&self, treeprinter: &mut TreePrinter, indent_lvl: i32, depth: i32) {
        treeprinter.add_print_item(self.name.clone(), depth, indent_lvl);
    }
}

pub struct FunctionCallExprAst {
    callee: String,
    args: Vec<Box<dyn Expr>>,
}
impl FunctionCallExprAst {
    pub fn new(callee: String, args: Vec<Box<dyn Expr>>) -> FunctionCallExprAst {
        FunctionCallExprAst { callee, args }
    }
}
impl Expr for FunctionCallExprAst {
    fn print(&self, treeprinter: &mut TreePrinter, indent_lvl: i32, depth: i32) {
        // TODO args
        treeprinter.add_print_item(self.callee.clone(), depth, indent_lvl);
    }
}

pub struct PrototypeAst {
    name: String,
    args: Vec<String>,
}
impl PrototypeAst {
    pub fn new(name: String, args: Vec<String>) -> PrototypeAst {
        PrototypeAst { name, args }
    }
}

pub struct FunctionAst {
    proto: PrototypeAst,
    body: Box<dyn Expr>,
}
impl FunctionAst {
    pub fn new(proto: PrototypeAst, body: Box<dyn Expr>) -> FunctionAst {
        FunctionAst { proto, body }
    }
}
