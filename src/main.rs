extern crate libc;
extern crate llvm_sys;
extern crate rustc_llvm_proxy;

use std::ffi::CString;

mod llvm;
use llvm::*;

fn main() {
    let mut lb = LlvmBuilder::new("my_module");

    lb.setup_main();

    let a = lb.create_variable("a", 35, int32_type());
    let b = lb.create_variable("b", 16, int32_type());
    let res = lb.multiple_variable(a, b, CString::new("ab_val").unwrap());

    let mut bf = BuilderFunctions {};
    bf.set_up(lb.builder, lb.context, lb.module);

    lb.return_variable(res);

    lb.dump();
    lb.emit_file("nyan.ll");
}
