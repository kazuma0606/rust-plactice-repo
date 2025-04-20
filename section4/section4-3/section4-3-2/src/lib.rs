// src/lib.rs
use std::ffi::{c_char, CStr, CString};
use std::os::raw::c_int;

// 基本的な整数計算関数
#[unsafe(no_mangle)]
pub extern "C" fn double(x: c_int) -> c_int {
    x * 2
}

// 文字列を受け取り、加工して返す関数
#[unsafe(no_mangle)]
pub extern "C" fn process_string(input: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        if input.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(input)
    };
    
    let rust_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    
    let result = format!("Processed: {}", rust_str);
    
    // CStringに変換してポインタを返す（呼び出し側で解放する必要あり）
    let c_result = CString::new(result).unwrap();
    c_result.into_raw()  // 所有権を手放す
}

// 文字列を解放する関数（呼び出し側でメモリリークを防ぐため）
#[unsafe(no_mangle)]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);  // 所有権を取り戻して解放
        }
    }
}