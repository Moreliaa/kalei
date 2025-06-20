use crate::treeprinter::*;
extern crate llvm_sys as llvm;
use crate::codegen::CodeGenContext;
use llvm::core::*;
use llvm::prelude::LLVMValueRef;
use llvm_sys::analysis::LLVMVerifyFunction;
use llvm_sys::execution_engine::LLVMGetFunctionAddress;

pub trait Expr {
    fn print(&self, treeprinter: &mut TreePrinter, indent_lvl: i32, depth: i32);
    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> LLVMValueRef;
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

    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> LLVMValueRef {
        unsafe {
            let ft = LLVMDoubleTypeInContext(codegen_context.context);
            LLVMConstReal(ft, self.val)
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

    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> LLVMValueRef {
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

    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> LLVMValueRef {
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

    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> LLVMValueRef {
        unsafe {
            // convert rust *const u8 pointer to C-compatible *const i8 pointer
            let name = (self.callee.clone() + "\0").into_bytes();
            let ptr = name.as_ptr() as *const i8;
            let callee_nf = LLVMGetNamedFunction(codegen_context.module, ptr);
            let callee_t = LLVMGlobalGetValueType(callee_nf);
            let mut args_v: Vec<LLVMValueRef> = self
                .args
                .iter()
                .map(|expr| expr.generate_code(codegen_context))
                .collect();

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

pub trait Function {
    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> LLVMValueRef;
}

pub struct PrototypeAst {
    pub name: String,
    pub args: Vec<String>,
}
impl PrototypeAst {
    pub fn new(name: String, args: Vec<String>) -> PrototypeAst {
        PrototypeAst { name, args }
    }
}
impl Function for PrototypeAst {
    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> LLVMValueRef {
        unsafe {
            println!("===prototype===");
            println!("llvm double type");
            let dt = LLVMDoubleTypeInContext(codegen_context.context);
            let mut args_t: Vec<llvm::prelude::LLVMTypeRef> =
                self.args.iter().map(|_| dt).collect();
            println!("llvm function type");
            let ft = LLVMFunctionType(
                dt,                  // return type
                args_t.as_mut_ptr(), // argument types
                args_t.len() as u32,
                false as i32, // TODO what is this?
            );
            let name = (self.name.clone() + "\0").into_bytes();
            let ptr = name.as_ptr() as *const i8;

            // TODO setting arg names
            println!("create prototype function");
            LLVMAddFunction(codegen_context.module, ptr, ft)
        }
    }
}

pub struct FunctionAst {
    pub proto: PrototypeAst,
    pub body: Box<dyn Expr>,
}
impl FunctionAst {
    pub fn new(proto: PrototypeAst, body: Box<dyn Expr>) -> FunctionAst {
        FunctionAst { proto, body }
    }
}
impl Function for FunctionAst {
    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> LLVMValueRef {
        unsafe {
            println!("create prototype");
            let function = self.proto.generate_code(codegen_context); // TODO handle repeated calls
            let ptr = self.proto.name.as_ptr() as *const i8;
            println!("create block");
            let bb =
                LLVMAppendBasicBlockInContext(codegen_context.context, function, c"entry".as_ptr());

            // insert instructions into the end of the basic block
            LLVMPositionBuilderAtEnd(codegen_context.ir_builder, bb);

            codegen_context.named_values.clear();
            for i in 0..self.proto.args.len() {
                let param = LLVMGetParam(function, i as u32);
                match self.proto.args.get(i) {
                    Some(name) => codegen_context.named_values.insert(name.clone(), param),
                    None => panic!("Invalid function param"),
                };
            }

            println!("create function body");
            let return_value = self.body.generate_code(codegen_context);
            println!("create return value");
            LLVMBuildRet(codegen_context.ir_builder, return_value);
            println!("verify function");
            LLVMVerifyFunction(
                function,
                llvm_sys::analysis::LLVMVerifierFailureAction::LLVMPrintMessageAction,
            );
            // TODO delete invalid functions
            // TODO fix extern function precedence over local

            function
        }
    }
}
