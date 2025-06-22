extern crate llvm_sys as llvm;
use crate::ast::*;
use std::collections::HashMap;

use llvm::core::*;
use llvm::prelude::LLVMBuilderRef;
use llvm::prelude::LLVMContextRef;
use llvm::prelude::LLVMModuleRef;
use llvm::prelude::LLVMValueRef;

pub struct CodeGenContext {
    pub context: LLVMContextRef,
    pub module: LLVMModuleRef,
    pub ir_builder: LLVMBuilderRef,
    pub named_values: HashMap<String, LLVMValueRef>,
}

pub fn create_context() -> CodeGenContext {
    unsafe {
        println!("Create code gen context");
        let context: LLVMContextRef = LLVMContextCreate();
        let module_id = c"module".as_ptr();
        let module = LLVMModuleCreateWithNameInContext(module_id, context);
        let ir_builder = LLVMCreateBuilderInContext(context);

        CodeGenContext {
            context,
            module,
            ir_builder,
            named_values: HashMap::new(),
        }
    }
}

pub fn generate_code(codegen_context: &mut CodeGenContext, function: Box<dyn Function>) {
    println!("===Start generate code===");
    function.generate_code(codegen_context);
    println!("===End generate code===");
}

pub fn dispose_context(codegen_context: &mut CodeGenContext) {
    unsafe {
        println!("Code gen context dispose");
        LLVMDumpModule(codegen_context.module); // dump module as IR to stdout
        LLVMDisposeBuilder(codegen_context.ir_builder);
        LLVMDisposeModule(codegen_context.module);
        LLVMContextDispose(codegen_context.context);
    }
}
