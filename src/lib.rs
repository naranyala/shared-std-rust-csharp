use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Adds two integers.
/// This is a simple function that can be easily called from C#.
#[unsafe(no_mangle)]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

/// Takes a string from C#, converts it to a Rust string, 
/// prepends "Hello, " to it, and returns it back to C#.
/// 
/// IMPORTANT: The caller (C#) is responsible for freeing the memory
/// allocated by Rust via the `free_string` function.
#[unsafe(no_mangle)]
pub extern "C" fn hello_rust(name: *const c_char) -> *mut c_char {
    if name.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(name) };
    let name_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let greeting = format!("Hello, {} from Rust!", name_str);
    let c_greeting = CString::new(greeting).expect("CString::new failed");
    
    c_greeting.into_raw()
}

/// Frees the memory allocated by `hello_rust`.
#[unsafe(no_mangle)]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}
