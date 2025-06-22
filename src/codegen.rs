extern crate llvm_sys as llvm;
use crate::{ast::*, logger::*};
use std::collections::HashMap;

use llvm::core::*;
use llvm::prelude::LLVMBuilderRef;
use llvm::prelude::LLVMContextRef;
use llvm::prelude::LLVMModuleRef;
use llvm::prelude::LLVMValueRef;
use llvm_sys::target::*;
use llvm_sys::target_machine::LLVMCodeGenFileType;
use llvm_sys::target_machine::LLVMCreateTargetMachine;
use llvm_sys::target_machine::LLVMGetDefaultTargetTriple;
use llvm_sys::target_machine::LLVMGetTargetFromTriple;
use llvm_sys::target_machine::LLVMTarget;
use llvm_sys::target_machine::LLVMTargetMachineEmitToFile;
use llvm_sys::target_machine::LLVMTargetRef;

pub struct CodeGenContext {
    pub context: LLVMContextRef,
    pub module: LLVMModuleRef,
    pub ir_builder: LLVMBuilderRef,
    pub named_values: HashMap<String, u32>,
    pub current_function: Option<LLVMValueRef>,
}

pub fn create_context() -> CodeGenContext {
    unsafe {
        log_verbose("Create code gen context".to_string());
        let context: LLVMContextRef = LLVMContextCreate();
        let module_id = c"module".as_ptr();
        let module = LLVMModuleCreateWithNameInContext(module_id, context);
        let ir_builder = LLVMCreateBuilderInContext(context);

        CodeGenContext {
            context,
            module,
            ir_builder,
            named_values: HashMap::new(),
            current_function: None,
        }
    }
}

pub fn generate_code(codegen_context: &mut CodeGenContext, function: Box<dyn Function>) {
    log_verbose("===Start generate code===".to_string());
    function.generate_code(codegen_context);
    log_verbose("===End generate code===".to_string());
}

pub fn dump(codegen_context: &mut CodeGenContext) {
    println!();
    println!();
    unsafe {
        LLVMDumpModule(codegen_context.module); // dump module as IR to stdout
    }
    println!();
    println!();
}

pub fn dispose_context(codegen_context: &mut CodeGenContext) {
    unsafe {
        log_verbose("Code gen context dispose".to_string());
        LLVMDisposeBuilder(codegen_context.ir_builder);
        LLVMDisposeModule(codegen_context.module);
        LLVMContextDispose(codegen_context.context);
    }
}

pub fn emit_to_file(codegen_context: &mut CodeGenContext) {
    unsafe {
        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmParsers();
        LLVM_InitializeAllAsmPrinters();

        // target
        // https://clang.llvm.org/docs/CrossCompilation.html#target-triple
        let mut target_triple = LLVMGetDefaultTargetTriple();
        // TODO deprecated
        let mut target: LLVMTargetRef = std::mem::uninitialized();
        let mut error_msg = c"".as_ptr() as *mut i8;
        if LLVMGetTargetFromTriple(target_triple, &mut target, &mut error_msg) != 0 {
            println!("Something happened");
            return;
        }

        // target machine
        let cpu = c"generic".as_ptr();
        let features = c"".as_ptr();
        let target_machine = LLVMCreateTargetMachine(
            target,
            target_triple,
            cpu,
            features,
            llvm_sys::target_machine::LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
            llvm_sys::target_machine::LLVMRelocMode::LLVMRelocPIC,
            llvm_sys::target_machine::LLVMCodeModel::LLVMCodeModelDefault,
        );

        LLVMSetDataLayout(
            codegen_context.module,
            LLVMGetDataLayoutStr(codegen_context.module),
        );
        // TODO setTargetTriple?

        // emit object code
        let filename = c"output.o".as_ptr();
        //let pm = LLVMCreatePassManager();
        //LLVMRunPassManager(pm, codegen_context.module);
        let file_type = LLVMCodeGenFileType::LLVMObjectFile;
        let mut error_msg = c"".as_ptr() as *mut i8;
        LLVMTargetMachineEmitToFile(
            target_machine,
            codegen_context.module,
            filename,
            file_type,
            &mut error_msg,
        );
    }
}
