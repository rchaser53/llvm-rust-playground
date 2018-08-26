extern crate libc;
extern crate llvm_sys;

use std::ffi::CString;

mod llvm;
use llvm::*;

fn main() {
    let mut builder = LlvmBuilder::new("my_module");

    builder.setup_main();

    let a = builder.create_variable("a", 35, int32_type());
    let b = builder.create_variable("b", 16, int32_type());
    let res = builder.multiple_variable(a, b, CString::new("ab_val").unwrap());

    builder.return_variable(res);

    builder.dump();
}
