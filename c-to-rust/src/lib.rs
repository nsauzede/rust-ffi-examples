#![crate_type = "staticlib"]

use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn double_input(input: i32) -> i32 {
    input * 2
}

#[no_mangle]
pub extern "C" fn hello_world() -> *mut c_char {
    let hello = CString::new("Hello World!").unwrap().into_raw();
    hello
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            // This reclaim ownership of string, and will free it at the end of the present block
            CString::from_raw(ptr);
        }
    }
}
