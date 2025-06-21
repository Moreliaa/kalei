extern crate llvm_sys as llvm;
use crate::ast::*;
use std::collections::HashMap;

use llvm::prelude::LLVMBuilderRef;
use llvm::prelude::LLVMContextRef;
use llvm::prelude::LLVMValueRef;
use llvm_sys::core::LLVMContextDispose;
use llvm_sys::core::LLVMDisposeBuilder;
use llvm_sys::core::LLVMDisposeModule;
use llvm_sys::core::LLVMDumpModule;
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

pub fn create_context() -> CodeGenContext {
    unsafe {
        println!("Create code gen context");
        let context: LLVMContextRef = llvm::core::LLVMContextCreate();
        let module_id = c"module".as_ptr();
        let module = llvm::core::LLVMModuleCreateWithNameInContext(module_id, context);
        let ir_builder = llvm::core::LLVMCreateBuilderInContext(context);

        CodeGenContext {
            context,
            module,
            ir_builder,
            named_values: HashMap::new(),
        }
    }
}

pub fn generate_code(codegen_context: &mut CodeGenContext, function: Box<dyn Function>) {
    unsafe {
        println!("Gen code");
        function.generate_code(codegen_context);
        println!("Code gen context dispose");
    }
}

pub fn dispose_context(codegen_context: &mut CodeGenContext) {
    unsafe {
        LLVMDumpModule(codegen_context.module); // dump module as IR to stdout
        LLVMDisposeBuilder(codegen_context.ir_builder);
        LLVMDisposeModule(codegen_context.module);
        LLVMContextDispose(codegen_context.context);
    }
}
