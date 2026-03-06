# Rust Unsafe 代码指南

## 概述

Unsafe 代码是 Rust 的一个强大特性，允许开发者绕过 Rust 的安全检查。虽然 Unsafe 代码提供了更大的灵活性，但也带来了更大的风险。

## 为什么需要 Unsafe？

Rust 的安全检查器（borrow checker）非常严格，有时会阻止一些合法的操作。Unsafe 代码允许我们在确保安全的前提下执行这些操作。

## Unsafe 的使用场景

### 1. 与 C/C++ 代码交互 (FFI)
```rust
extern "C" {
    fn c_function(arg: i32) -> i32;
}

unsafe {
    let result = c_function(42);
}
```

### 2. 实现自定义智能指针
```rust
struct MyBox<T> {
    ptr: *mut T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        unsafe {
            let ptr = Box::into_raw(Box::new(value));
            Self { ptr }
        }
    }
}
```

### 3. 性能优化
```rust
// 手动向量化操作
unsafe {
    let src: *const f32 = ...;
    let dst: *mut f32 = ...;
    // 使用 SIMD 指令优化
}
```

### 4. 底层系统编程
- 操作系统内核开发
- 设备驱动程序
- 嵌入式系统

## Unsafe 的核心概念

### 1. Unsafe 块
```rust
unsafe {
    // 在这里可以执行不安全的操作
    let ptr: *const i32 = std::ptr::null();
    // 解引用指针
    let value = *ptr;
}
```

### 2. Unsafe 函数
```rust
unsafe fn dangerous() {
    // 函数体中的所有代码都被视为 unsafe
}

unsafe {
    dangerous();
}
```

### 3. 原始指针
```rust
let mut x = 42;
let ptr: *const i32 = &x as *const i32;
let mut_ptr: *mut i32 = &mut x as *mut i32;
```

### 4. 联合体 (Union)
```rust
union Data {
    integer: i32,
    float: f32,
}

let data = Data { integer: 42 };
unsafe {
    println!("Integer: {}", data.integer);
}
```

## Unsafe 的责任

### 1. 内存安全
- 确保指针指向有效的内存
- 避免空指针解引用
- 正确管理内存生命周期

### 2. 数据竞争安全
- 避免多个线程同时访问可变数据
- 使用适当的同步机制

### 3. 类型安全
- 确保类型转换是合法的
- 避免未定义行为

## 最佳实践

### 1. 最小化原则
```rust
// ❌ 不好：整个函数都是 unsafe
unsafe fn process_data(data: *mut u8, len: usize) {
    // 很多安全代码...
    // 只有少数几行需要 unsafe
}

// ✅ 好：只在需要的地方使用 unsafe
fn process_data(data: *mut u8, len: usize) {
    // 安全代码...
    unsafe {
        // 只有这里需要 unsafe
        *data = 42;
    }
    // 更多安全代码...
}
```

### 2. 封装原则
```rust
// ❌ 不好：暴露 unsafe 接口
pub unsafe fn unsafe_operation() { ... }

// ✅ 好：封装在安全的 API 中
pub fn safe_operation() {
    unsafe {
        // 内部使用 unsafe
        unsafe_operation();
    }
}
```

### 3. 文档原则
```rust
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer.
/// The caller must ensure:
/// 1. The pointer points to valid memory
/// 2. The memory is properly aligned
/// 3. The memory is not accessed by other threads
unsafe fn unsafe_function(ptr: *const i32) {
    // Implementation
}
```

### 4. 测试原则
```rust
#[test]
fn test_unsafe_code() {
    // 测试正常情况
    // 测试边界情况
    // 测试错误情况
}
```

## 常见陷阱

### 1. 悬垂指针
```rust
// ❌ 错误：创建悬垂指针
let ptr: *const i32;
{
    let x = 42;
    ptr = &x as *const i32;
} // x 被释放
// unsafe { println!("{}", *ptr); } // 未定义行为！

// ✅ 正确：确保生命周期
let x = 42;
let ptr = &x as *const i32;
unsafe { println!("{}", *ptr); }
```

### 2. 数据竞争
```rust
// ❌ 错误：多个线程同时修改数据
static mut COUNTER: i32 = 0;

// ✅ 正确：使用原子类型
use std::sync::atomic::{AtomicI32, Ordering};
static COUNTER: AtomicI32 = AtomicI32::new(0);
```

### 3. 类型混淆
```rust
// ❌ 错误：错误的类型转换
let x: i32 = 42;
let y: f64 = unsafe { std::mem::transmute(x) }; // 可能导致未定义行为

// ✅ 正确：使用安全的转换
let x: i32 = 42;
let y: f64 = x as f64;
```

## 调试工具

### 1. Miri
Miri 是 Rust 的未定义行为检测器：
```bash
cargo miri test
cargo miri run
```

### 2. AddressSanitizer
检测内存错误：
```bash
RUSTFLAGS="-Z sanitizer=address" cargo +nightly run
```

### 3. Valgrind
检测内存泄漏和错误：
```bash
valgrind ./target/debug/your_program
```

## 实际案例

### 1. 标准库中的 Unsafe
Rust 标准库中大量使用了 unsafe：
- `Vec<T>`：手动内存管理
- `Box<T>`：堆分配
- `Rc<T>`：引用计数
- `Arc<T>`：原子引用计数

### 2. 常见库中的 Unsafe
- `libc`：C 语言绑定
- `winapi`：Windows API 绑定
- `socket2`：网络编程
- `memmap`：内存映射文件

## 安全检查清单

在编写 unsafe 代码前，问自己：

1. [ ] 是否真的需要 unsafe？
2. [ ] 是否有更安全的替代方案？
3. [ ] 是否理解所有可能的未定义行为？
4. [ ] 是否确保了内存安全？
5. [ ] 是否避免了数据竞争？
6. [ ] 是否正确处理了生命周期？
7. [ ] 是否有充分的测试？
8. [ ] 是否有清晰的文档？
9. [ ] 是否考虑了边界情况？
10. [ ] 是否考虑了错误处理？

## 总结

Unsafe 代码是 Rust 的强大特性，但也是危险的。记住：

> "With great power comes great responsibility."

### 核心原则

1. **能不用就不用**：优先使用安全的 Rust 代码
2. **最小化范围**：将 unsafe 限制在最小范围内
3. **充分测试**：为 unsafe 代码编写全面的测试
4. **详细文档**：说明为什么需要 unsafe 以及如何保证安全
5. **持续验证**：使用工具验证内存安全

### 推荐阅读

- [Rustonomicon](https://doc.rust-lang.org/nomicon/)：Rust 不安全代码指南
- [Rust Reference](https://doc.rust-lang.org/reference/)：语言参考
- [Miri](https://github.com/rust-lang/miri)：未定义行为检测器

## 示例代码

本项目中的 `unsafe_examples.rs` 文件提供了完整的 unsafe 代码示例，包括：

1. 原始指针操作
2. Unsafe 函数调用
3. 切片操作
4. 联合体使用
5. 内存管理
6. 静态变量访问
7. 自定义 Vec 实现
8. FFI 调用
9. 内联汇编
10. 安全封装
11. 内存对齐
12. 类型转换

运行示例：
```bash
cargo run --bin rust-learning
```

运行测试：
```bash
cargo test --lib
```

---

**记住**：Unsafe 代码不是敌人，而是工具。正确使用它，可以发挥 Rust 的最大潜力；错误使用它，会导致难以调试的 bug 和安全漏洞。谨慎使用，充分测试，详细文档！