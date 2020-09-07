#![crate_type = "staticlib"]

use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn double_input(input: i32) -> i32 {
    input * 2
}

#[no_mangle]
pub extern "C" fn hello_world() -> *mut c_char {
    let hello = CString::new("Hello World!").unwrap().into_raw();
    hello
}
