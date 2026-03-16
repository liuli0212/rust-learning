//! FFI 与跨语言互操作 (C/C++ Interop)
//!
//! 展示如何使用标准 FFI 与 C/C++ 进行交互。
//! 对于大型项目，通常建议使用 `cxx` 或 `autocxx` 来处理 C++，
//! 使用 `bindgen` 处理 C 头文件。这里演示底层基础。

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

// 1. 声明外部 C 函数 (通常来自 libc 或链接的 C/C++ 库)
// 这里我们声明标准 C 库中的 abs 和 strlen 作为演示
extern "C" {
    fn abs(i: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

pub fn standard_c_ffi() {
    println!("  === 标准 C FFI 调用 ===");

    // 调用 C 语言的 abs
    let x = -42;
    // 所有对外部 C 函数的调用都必须在 unsafe 块中，因为 Rust 无法验证 C 代码的安全性
    let abs_x = unsafe { abs(x) };
    println!("    C abs({}) = {}", x, abs_x);

    // 调用 C 语言的 strlen
    // 我们必须将 Rust 的字符串转换为 C 风格的以 null 结尾的字符串
    let rust_string = "Hello FFI!";
    let c_string = CString::new(rust_string).expect("CString::new failed");
    
    // as_ptr() 返回 *const c_char
    let len = unsafe { strlen(c_string.as_ptr()) };
    println!("    C strlen(\"{}\") = {}", rust_string, len);
}

/// 2. 暴露 Rust 函数给 C/C++ 调用
/// 使用 #[no_mangle] 阻止 Rust 编译器重命名该函数，并指定 extern "C" ABI
#[no_mangle]
pub extern "C" fn rust_hello_for_c(name: *const c_char) {
    if name.is_null() {
        return;
    }

    // 安全地将 C 字符串转换为 Rust 的 &str
    let c_str = unsafe { CStr::from_ptr(name) };
    match c_str.to_str() {
        Ok(s) => println!("    [Rust 内部]: Hello, C++ Developer {}!", s),
        Err(_) => println!("    [Rust 内部]: Invalid UTF-8 received from C!"),
    }
}

pub fn call_rust_from_c_mock() {
    println!("\n  === 模拟 C 调用 Rust 函数 ===");
    // 在纯 Rust 环境中，我们模拟 C 端传入一个指针
    let mock_c_str = b"Bjarne Stroustrup\0";
    let ptr = mock_c_str.as_ptr() as *const c_char;
    
    // 这就相当于 C 代码调用了 rust_hello_for_c(ptr)
    rust_hello_for_c(ptr);
}

pub fn run_examples() {
    println!("=== FFI 与 C/C++ 互操作 ===");
    standard_c_ffi();
    call_rust_from_c_mock();
    println!("===========================\n");
}
