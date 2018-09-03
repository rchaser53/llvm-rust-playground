use std::ffi::CString;

use llvm_sys::core::*;
// use llvm_sys::execution_engine::*;
use llvm_sys::*;

// const LLVM_ERROR: i32 = 1;

pub fn int32_type() -> *mut LLVMType {
    unsafe { LLVMInt32Type() }
}

pub struct LlvmBuilder {
    pub builder: *mut LLVMBuilder,
    pub context: *mut LLVMContext,
    pub module: *mut LLVMModule,
}

impl LlvmBuilder {
    pub fn new(module_name: &str) -> LlvmBuilder {
        unsafe {
            let context = LLVMGetGlobalContext();
            let mod_name = CString::new(module_name).unwrap();

            LlvmBuilder {
                builder: LLVMCreateBuilder(),
                module: LLVMModuleCreateWithName(mod_name.as_ptr()),
                context: context,
            }
        }
    }

    pub fn setup_main(&mut self) {
      self.add_function(int32_type(), &mut [], "main");
      let block = self.append_basic_block("main", "entry");
      self.end_basic_block(block);
    }

    pub fn add_function(
        &mut self,
        ret_type: *mut LLVMType,
        args: &mut [*mut LLVMType],
        fn_name: &str,
    ) -> *mut LLVMValue {
        unsafe {
            let fn_type = LLVMFunctionType(ret_type, args.as_mut_ptr(), args.len() as u32, 0);
            let cstring = CString::new(fn_name).unwrap();
            let ptr = cstring.as_ptr() as *mut _;
            LLVMAddFunction(self.module, ptr, fn_type)
        }
    }

    pub fn get_named_function(&mut self, name: &str) -> *mut LLVMValue {
        let func_name = CString::new(name).unwrap();
        unsafe { LLVMGetNamedFunction(self.module, func_name.as_ptr()) }
    }

    pub fn append_basic_block(&mut self, function_name: &str, name: &str) -> *mut LLVMBasicBlock {
        let label_name = CString::new(name).unwrap();
        let function = self.get_named_function(function_name);

        unsafe { LLVMAppendBasicBlock(function, label_name.as_ptr()) }
    }

    pub fn end_basic_block(&mut self, block: *mut LLVMBasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder, block);
        }
    }

    /* need refactoring below */

    pub fn create_variable(
        &mut self,
        name: &str,
        value: u64,
        llvm_type: *mut LLVMType,
    ) -> *mut LLVMValue {
        let val_name = CString::new(name).unwrap();
        let llvm_value = unsafe { LLVMBuildAlloca(self.builder, llvm_type, val_name.as_ptr()) };
        unsafe {
            LLVMBuildStore(self.builder, LLVMConstInt(llvm_type, value, 0), llvm_value);
        }
        unsafe { LLVMBuildLoad(self.builder, llvm_value, val_name.as_ptr()) }
    }

    pub fn multiple_variable(
        &mut self,
        var_a: *mut LLVMValue,
        var_b: *mut LLVMValue,
        c_str: CString,
    ) -> *mut LLVMValue {
        unsafe { LLVMBuildMul(self.builder, var_a, var_b, c_str.as_ptr()) }
    }

    pub fn return_variable(&mut self, res: *mut LLVMValue) {
        unsafe {
            LLVMBuildRet(self.builder, res);
        }
    }

    /* need refactoring above */

    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.module) }
    }
}

impl Drop for LlvmBuilder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
        }
    }
}

#[macro_export]
macro_rules! c_string {
  ($w:expr) => ( CString::new($w).unwrap() );
}

pub struct BuilderFunctions {}

impl BuilderFunctions {
  pub fn set_up(&mut self, builder: *mut LLVMBuilder, context: *mut LLVMContext, module: *mut LLVMModule) {
    unsafe {
      let print = self.create_printf(module);
      let mut printf_args = [
        codegen_string(module, context, "%d\n"),
        LLVMConstInt(LLVMInt32Type(), 15, 0)
      ];

      LLVMBuildCall(builder, print, printf_args.as_mut_ptr(), 2, c_string!("").as_ptr());
    } 
  }

  pub fn create_printf(&mut self, module: *mut LLVMModule) -> *mut LLVMValue {
    unsafe {
      let mut printf_args_type_list = [LLVMPointerType(LLVMInt8Type(), 0)];
      let printf_type = LLVMFunctionType(LLVMInt32Type(), printf_args_type_list.as_mut_ptr(), 0, 1);
      return LLVMAddFunction(module, CString::new("printf").unwrap().as_ptr() as *mut _, printf_type);
    }
  }
}

pub fn codegen_string(module: *mut LLVMModule, context: *mut LLVMContext, input_str: &str) -> *mut LLVMValue {
  let length = input_str.len() as u32;
  unsafe {
    let str_val = LLVMConstStringInContext(context, c_string!(input_str).as_ptr(), length, 0);
    let g_str = LLVMAddGlobal(module, LLVMTypeOf(str_val), c_string!("").as_ptr());
    LLVMSetLinkage(g_str, LLVMLinkage::LLVMPrivateLinkage);
    LLVMSetInitializer(g_str, str_val);

    let mut args = [
      LLVMConstInt(LLVMInt32Type(), 0, 0),
      LLVMConstInt(LLVMInt32Type(), 0, 0)
    ];

    return LLVMConstInBoundsGEP(g_str, args.as_mut_ptr(), 2);
  }
}

// pub fn run_function(
//     &mut self,
//     engine: *mut LLVMOpaqueExecutionEngine,
//     named_function: *mut LLVMValue,
//     params: &mut [*mut LLVMOpaqueGenericValue],
// ) -> u64 {
//     let func_result = unsafe {
//         LLVMRunFunction(
//             engine,
//             named_function,
//             params.len() as u32,
//             params.as_mut_ptr(),
//         )
//     };
//     unsafe { LLVMGenericValueToInt(func_result, 0) }
// }

// pub fn int64_type() -> *mut LLVMType {
//     unsafe { LLVMInt64Type() }
// }

// create our exe engine
// pub fn excute_module_by_interpreter(
//     engine_ref: *mut LLVMExecutionEngineRef,
//     module: *mut LLVMModule,
// ) -> Result<i32, String> {
//     let mut error = 0 as *mut ::libc::c_char;
//     let status = unsafe {
//         let buf: *mut *mut ::libc::c_char = &mut error;
//         LLVMLinkInInterpreter();
//         LLVMCreateInterpreterForModule(engine_ref, module, buf)
//     };

//     if status == LLVM_ERROR {
//         let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
//         return Err(err_msg);
//     }

//     Ok(status)
// }

// pub fn add_module(module_name: &str) -> *mut LLVMModule {
//     let mod_name = CString::new(module_name).unwrap();
//     unsafe { LLVMModuleCreateWithName(mod_name.as_ptr()) }
// }