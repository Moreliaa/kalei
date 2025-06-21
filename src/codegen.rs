extern crate llvm_sys as llvm;
use crate::ast::*;
use std::collections::HashMap;

use llvm::prelude::LLVMBuilderRef;
use llvm::prelude::LLVMContextRef;
use llvm::prelude::LLVMValueRef;
use llvm_sys::prelude::LLVMModuleRef;

// use llvm::core::*;
// use llvm::disassembler::*;
// use llvm::execution_engine::*;
// use llvm::target::*;

pub struct CodeGenContext {
    pub context: LLVMContextRef,
    pub module: LLVMModuleRef,
    pub ir_builder: LLVMBuilderRef,
    pub named_values: HashMap<String, LLVMValueRef>,
}

pub fn generate_code(some_expr: Box<dyn Expr>) {
    unsafe {
        let context: LLVMContextRef = llvm::core::LLVMContextCreate();
        let module_id = b"sum\0".as_ptr() as *const _;
        let module = llvm::core::LLVMModuleCreateWithNameInContext(module_id, context);
        let ir_builder = llvm::core::LLVMCreateBuilderInContext(context);

        // symbol table
        let mut named_values: HashMap<String, LLVMValueRef> = HashMap::new();

        let mut codegen_context = CodeGenContext {
            context,
            module,
            ir_builder,
            named_values,
        };

        let value: LLVMValueRef = some_expr.generate_code(&codegen_context);
    }
}
