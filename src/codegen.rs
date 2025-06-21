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

pub fn generate_code(function: Box<dyn Function>) {
    unsafe {
        let context: LLVMContextRef = llvm::core::LLVMContextCreate();
        let module_id = c"sum".as_ptr();
        let module = llvm::core::LLVMModuleCreateWithNameInContext(module_id, context);
        let ir_builder = llvm::core::LLVMCreateBuilderInContext(context);

        // symbol table

        let mut codegen_context = CodeGenContext {
            context,
            module,
            ir_builder,
            named_values: HashMap::new(),
        };

        function.generate_code(&codegen_context);
        LLVMDumpModule(codegen_context.module); // dump module as IR to stdout
        LLVMDisposeBuilder(codegen_context.ir_builder);
        LLVMDisposeModule(codegen_context.module);
        LLVMContextDispose(codegen_context.context);
    }
}
