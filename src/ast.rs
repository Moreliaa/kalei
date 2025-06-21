use crate::treeprinter::*;
extern crate llvm_sys as llvm;
use crate::codegen::CodeGenContext;
use llvm::core::*;
use llvm::prelude::LLVMValueRef;

pub trait Expr {
    fn print(&self, treeprinter: &mut TreePrinter, indent_lvl: i32, depth: i32);
    fn generate_code(&self, codegen_context: &CodeGenContext) -> LLVMValueRef;
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

    fn generate_code(&self, codegen_context: &CodeGenContext) -> LLVMValueRef {
        unsafe {
            let ft = llvm::core::LLVMFloatTypeInContext(codegen_context.context);
            llvm::core::LLVMConstReal(ft, self.val)
        }
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

    fn generate_code(&self, codegen_context: &CodeGenContext) -> LLVMValueRef {
        unsafe {
            let lhs_value = self.lhs.generate_code(codegen_context);
            let rhs_value = self.rhs.generate_code(codegen_context);

            match self.op.as_str() {
                "+" => LLVMConstAdd(lhs_value, rhs_value),
                "-" => LLVMConstSub(lhs_value, rhs_value),
                "*" => LLVMConstMul(lhs_value, rhs_value),
                "/" => todo!(),
                _ => panic!("Invalid binary operator {}", self.op),
            }
        }
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

    fn generate_code(&self, codegen_context: &CodeGenContext) -> LLVMValueRef {
        if codegen_context.named_values.contains_key(&self.name) {
            *codegen_context.named_values.get(&self.name).unwrap()
        } else {
            panic!("Unknown variable name {}", self.name);
        }
    }
}

pub struct FunctionCallExprAst {
    pub callee: String,
    pub args: Vec<Box<dyn Expr>>,
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

    fn generate_code(&self, codegen_context: &CodeGenContext) -> LLVMValueRef {
        unsafe {
            // convert rust *const u8 pointer to C-compatible *const i8 pointer
            let ptr = self.callee.as_ptr() as *const i8;
            let callee_nf = LLVMGetNamedFunction(codegen_context.module, ptr);
            let callee_t = LLVMGlobalGetValueType(callee_nf);
            let mut args_v: Vec<LLVMValueRef> = vec![];
            for i in 0..self.args.len() {
                args_v.push(self.args.get(i).unwrap().generate_code(codegen_context));
            }
            LLVMBuildCall2(
                codegen_context.ir_builder,
                callee_t,
                callee_nf,
                args_v.as_mut_ptr(),
                args_v.len() as u32,
                ptr,
            )
        }
    }
}

pub trait Function {}

pub struct PrototypeAst {
    pub name: String,
    pub args: Vec<String>,
}
impl PrototypeAst {
    pub fn new(name: String, args: Vec<String>) -> PrototypeAst {
        PrototypeAst { name, args }
    }
}
impl Function for PrototypeAst {}

pub struct FunctionAst {
    pub proto: PrototypeAst,
    pub body: Box<dyn Expr>,
}
impl FunctionAst {
    pub fn new(proto: PrototypeAst, body: Box<dyn Expr>) -> FunctionAst {
        FunctionAst { proto, body }
    }
}
impl Function for FunctionAst {}
