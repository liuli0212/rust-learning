//! Unsafe 代码示例的测试

use rust_learning::unsafe_examples;

#[test]
fn test_raw_pointer_examples() {
    // 这个测试主要是确保代码能运行，不崩溃
    unsafe_examples::raw_pointer_examples();
}

#[test]
fn test_unsafe_function_examples() {
    unsafe_examples::unsafe_function_examples();
}

#[test]
fn test_slice_examples() {
    unsafe_examples::slice_examples();
}

#[test]
fn test_union_examples() {
    unsafe_examples::union_examples();
}

#[test]
fn test_memory_operations() {
    unsafe_examples::memory_operations();
}

#[test]
fn test_static_variable_examples() {
    unsafe_examples::static_variable_examples();
}

#[test]
fn test_custom_vec_example() {
    unsafe_examples::custom_vec_example();
}

#[test]
fn test_ffi_examples() {
    unsafe_examples::ffi_examples();
}

#[test]
fn test_inline_asm_examples() {
    unsafe_examples::inline_asm_examples();
}

#[test]
fn test_safe_wrapper_examples() {
    unsafe_examples::safe_wrapper_examples();
}

#[test]
fn test_alignment_examples() {
    unsafe_examples::alignment_examples();
}

#[test]
fn test_type_transmutation_examples() {
    unsafe_examples::type_transmutation_examples();
}

#[test]
fn test_run_examples() {
    // 测试运行所有示例
    unsafe_examples::run_examples();
}

#[test]
fn test_custom_vec_functionality() {
    use rust_learning::unsafe_examples::custom_vec_example;
    
    // 这个测试确保自定义 Vec 的基本功能正常
    custom_vec_example();
}

#[test]
fn test_safe_vec_wrapper() {
    use rust_learning::unsafe_examples::safe_wrapper_examples;
    
    // 测试安全封装的 Vec
    safe_wrapper_examples();
}
