extern crate llvm_sys as llvm;
use crate::{codegen::*, logger::*};
use llvm::core::*;
use llvm::prelude::LLVMValueRef;
use llvm_sys::analysis::LLVMVerifyFunction;

pub trait Expr {
    // fn print(&self, treeprinter: &mut TreePrinter, indent_lvl: i32, depth: i32);
    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> Result<LLVMValueRef, String>;
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
    // fn print(&self, treeprinter: &mut TreePrinter, indent_lvl: i32, depth: i32) {
    //     treeprinter.add_print_item(self.val.to_string(), depth, indent_lvl);
    // }

    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> Result<LLVMValueRef, String> {
        log_verbose(format!("Generate number expr {:?}", self.val));
        unsafe {
            let ft = LLVMDoubleTypeInContext(codegen_context.context);
            Ok(LLVMConstReal(ft, self.val))
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
    // fn print(&self, treeprinter: &mut TreePrinter, indent_lvl: i32, depth: i32) {
    //     treeprinter.add_print_item(self.op.clone(), depth, indent_lvl);
    //     self.lhs.print(treeprinter, indent_lvl - 1, depth + 1);
    //     self.rhs.print(treeprinter, indent_lvl + 1, depth + 1);
    // }

    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> Result<LLVMValueRef, String> {
        unsafe {
            let lhs_value = self.lhs.generate_code(codegen_context)?;
            let rhs_value = self.rhs.generate_code(codegen_context)?;

            log_verbose(format!("Generate binary expr {:?}", self.op));
            let name = c"op".as_ptr() as *const _;

            match self.op.as_str() {
                "+" => Ok(LLVMBuildFAdd(
                    codegen_context.ir_builder,
                    lhs_value,
                    rhs_value,
                    name,
                )),
                "-" => Ok(LLVMBuildFSub(
                    codegen_context.ir_builder,
                    lhs_value,
                    rhs_value,
                    name,
                )),
                "*" => Ok(LLVMBuildFMul(
                    codegen_context.ir_builder,
                    lhs_value,
                    rhs_value,
                    name,
                )),
                //"/" => Err("Division not implemented")?,
                _ => Err(format!("Invalid binary operator {}", self.op)),
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
    // fn print(&self, treeprinter: &mut TreePrinter, indent_lvl: i32, depth: i32) {
    //     treeprinter.add_print_item(self.name.clone(), depth, indent_lvl);
    // }

    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> Result<LLVMValueRef, String> {
        log_verbose(format!("Generate variable expr {:?}", self.name));
        unsafe {
            if codegen_context.named_values.contains_key(&self.name) {
                let index = codegen_context.named_values.get(&self.name).unwrap();
                Ok(LLVMGetParam(
                    codegen_context.current_function.unwrap(),
                    *index,
                ))
            } else {
                Err(format!("Unknown variable name {}", self.name))
            }
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
    // fn print(&self, treeprinter: &mut TreePrinter, indent_lvl: i32, depth: i32) {
    //     // TODO args
    //     treeprinter.add_print_item(self.callee.clone(), depth, indent_lvl);
    // }

    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> Result<LLVMValueRef, String> {
        unsafe {
            // convert rust *const u8 pointer to C-compatible *const i8 pointer
            let name = (self.callee.clone() + "\0").into_bytes();
            let ptr = name.as_ptr() as *const i8;
            let callee_nf = LLVMGetNamedFunction(codegen_context.module, ptr);
            let callee_t = LLVMGlobalGetValueType(callee_nf);
            let args_v: Vec<Result<LLVMValueRef, String>> = self
                .args
                .iter()
                .map(|expr| expr.generate_code(codegen_context))
                .collect();
            for arg in args_v.iter() {
                match arg {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e.clone());
                    }
                };
            }
            let mut args_v: Vec<LLVMValueRef> =
                args_v.into_iter().map(|val| val.unwrap()).collect();

            log_verbose(format!("Generate function call {:?}", self.callee));
            Ok(LLVMBuildCall2(
                codegen_context.ir_builder,
                callee_t,
                callee_nf,
                args_v.as_mut_ptr(),
                args_v.len() as u32,
                ptr,
            ))
        }
    }
}

pub trait Function {
    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> Result<LLVMValueRef, String>;
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
    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> Result<LLVMValueRef, String> {
        unsafe {
            let dt = LLVMDoubleTypeInContext(codegen_context.context);
            let mut args_t: Vec<llvm::prelude::LLVMTypeRef> =
                self.args.iter().map(|_| dt).collect();
            let ft = LLVMFunctionType(
                dt,                  // return type
                args_t.as_mut_ptr(), // argument types
                args_t.len() as u32,
                false as i32, // whether the function is variadic
            );
            let name = (self.name.clone() + "\0").into_bytes();
            let ptr = name.as_ptr() as *const i8;

            log_verbose(format!("Generate function prototype {:?}", self.name));
            let result = LLVMAddFunction(codegen_context.module, ptr, ft);

            // set function parameter names
            for i in 0..args_t.len() {
                let param = LLVMGetParam(result, i as u32);
                let param_name = (self.args.get(i).unwrap().clone()).into_bytes();
                LLVMSetValueName2(param, param_name.as_ptr() as *const i8, param_name.len());
            }

            Ok(result)
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
    fn generate_code(&self, codegen_context: &mut CodeGenContext) -> Result<LLVMValueRef, String> {
        unsafe {
            let function = self.proto.generate_code(codegen_context);
            match function {
                Ok(_) => {}
                Err(e) => {
                    return Err(e.clone());
                }
            };
            let function = function.unwrap();
            codegen_context.current_function = Some(function);

            //let ptr = self.proto.name.as_ptr() as *const i8;
            let bb =
                LLVMAppendBasicBlockInContext(codegen_context.context, function, c"entry".as_ptr());

            // insert instructions into the end of the basic block
            LLVMPositionBuilderAtEnd(codegen_context.ir_builder, bb);

            codegen_context.named_values.clear();
            for i in 0..self.proto.args.len() {
                match self.proto.args.get(i) {
                    Some(name) => codegen_context.named_values.insert(name.clone(), i as u32),
                    None => {
                        codegen_context.current_function = None;
                        LLVMDeleteFunction(function);
                        return Err(String::from("Invalid function param"));
                    }
                };
            }

            let return_value = self.body.generate_code(codegen_context);
            match return_value {
                Ok(_) => {}
                Err(e) => {
                    codegen_context.current_function = None;
                    LLVMDeleteFunction(function);
                    return Err(e.clone());
                }
            };
            let return_value = return_value.unwrap();
            LLVMBuildRet(codegen_context.ir_builder, return_value);
            LLVMVerifyFunction(
                function,
                llvm_sys::analysis::LLVMVerifierFailureAction::LLVMPrintMessageAction,
            );

            // TODO fix extern function precedence over local

            log_verbose(format!(
                "Generate function definition {:?}",
                self.proto.name
            ));
            codegen_context.current_function = None;
            Ok(function)
        }
    }
}
