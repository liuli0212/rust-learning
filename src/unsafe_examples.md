# Unsafe 代码示例

这个模块展示了 Rust 中 unsafe 代码的使用场景和注意事项。

## 目录

1. [原始指针](#原始指针)
2. [Unsafe 函数](#unsafe-函数)
3. [切片操作](#切片操作)
4. [联合体](#联合体)
5. [内存操作](#内存操作)
6. [静态变量](#静态变量)
7. [自定义 Vec](#自定义-vec)
8. [FFI](#ffi)
9. [内联汇编](#内联汇编)
10. [安全封装](#安全封装)
11. [内存对齐](#内存对齐)
12. [类型转换](#类型转换)

## 原始指针

原始指针 (`*const T` 和 `*mut T`) 可以绕过 Rust 的借用检查器。

```rust
let mut num = 42;
let raw_ptr: *const i32 = &num as *const i32;
let raw_mut_ptr: *mut i32 = &mut num as *mut i32;

unsafe {
    println!("值: {}", *raw_ptr);
    *raw_mut_ptr = 100;
}
```

**注意**：
- 解引用原始指针需要 `unsafe` 块
- 必须确保指针指向有效的内存
- 必须确保没有数据竞争

## Unsafe 函数

Unsafe 函数可以执行不安全的操作。

```rust
unsafe fn dangerous_function(x: i32, y: i32) -> i32 {
    x + y
}

let result = unsafe { dangerous_function(10, 20) };
```

**实际应用**：
- 调用 C 语言函数
- 执行需要绕过 Rust 安全检查的操作

## 切片操作

使用原始指针创建切片。

```rust
let arr = [1, 2, 3, 4, 5];

unsafe {
    let ptr = arr.as_ptr();
    let slice = slice::from_raw_parts(ptr.add(1), 3);
    // slice = [2, 3, 4]
}
```

**注意事项**：
- 指针必须指向有效的内存
- 长度不能超过数组边界
- 内存不能在切片使用期间被释放

## 联合体

联合体的字段访问需要 `unsafe`。

```rust
union Data {
    integer: i32,
    float: f32,
}

let data = Data { integer: 42 };
unsafe {
    println!("整数: {}", data.integer);
    println!("浮点: {}", data.float);
}
```

**注意**：Rust 的联合体不像 C 那样自动管理，需要手动确保类型安全。

## 内存操作

手动管理内存分配和释放。

```rust
let layout = std::alloc::Layout::from_size_align(10 * 4, 4).unwrap();

unsafe {
    let ptr = std::alloc::alloc(layout) as *mut i32;
    
    // 使用内存...
    
    // 释放内存
    std::alloc::dealloc(ptr as *mut u8, layout);
}
```

**警告**：手动内存管理容易出错，应尽量使用标准库的智能指针。

## 静态变量

静态可变变量需要 `unsafe` 访问。

```rust
static mut COUNTER: i32 = 0;

unsafe {
    COUNTER += 1;
}
```

**更好的替代**：
- 使用 `std::sync::atomic::AtomicI32`
- 使用 `std::sync::Mutex`

## 自定义 Vec

通过 unsafe 实现自定义的 Vec 类型。

```rust
struct CustomVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}
```

**关键点**：
- 手动内存管理
- 需要正确处理 Drop
- 需要处理内存分配失败

## FFI (外部函数接口)

调用 C 语言函数。

```rust
extern "C" {
    fn strlen(s: *const i8) -> usize;
}

let c_string = b"Hello\0";
unsafe {
    let len = strlen(c_string.as_ptr() as *const i8);
}
```

**注意**：实际使用中，建议使用 `libc` crate 或其他安全的绑定。

## 内联汇编

在某些平台上可以使用内联汇编。

```rust
// 仅在 x86_64 上支持
unsafe {
    asm!(
        "mov rax, 42",
        out("rax") _,
    );
}
```

**注意**：内联汇编是平台特定的，需要 `#![feature(asm)]`。

## 安全封装

将 unsafe 代码封装在安全的 API 中。

```rust
pub struct SafeVec<T> {
    // 私有字段，使用 unsafe 实现
}

impl<T> SafeVec<T> {
    pub fn push(&mut self, value: T) {
        // 公共接口是安全的
        self.push_internal(value);
    }
    
    fn push_internal(&mut self, value: T) {
        // 内部使用 unsafe
    }
}
```

**原则**：尽量将 unsafe 代码限制在最小范围内。

## 内存对齐

控制内存对齐。

```rust
#[repr(align(64))]
struct AlignedData {
    data: [u8; 128],
}

let aligned = AlignedData { data: [0; 128] };
```

**用途**：
- SIMD 指令需要特定对齐
- 提高性能
- 与硬件交互

## 类型转换

使用 `std::mem::transmute` 进行类型转换。

```rust
// 安全的位模式转换
let float = 1.0f32;
let bits: u32 = float.to_bits();
let back: f32 = f32::from_bits(bits);

// 复杂的类型转换（需要 unsafe）
unsafe {
    let bytes: [u8; 8] = std::mem::transmute(3.14f64);
}
```

**警告**：类型转换可能导致未定义行为，必须确保类型兼容。

## Unsafe 的使用原则

### 1. 最小化原则
- 只在必要时使用 unsafe
- 将 unsafe 代码限制在最小范围内

### 2. 封装原则
- 将 unsafe 代码封装在安全的 API 中
- 为 unsafe 代码编写详细的文档

### 3. 验证原则
- 为 unsafe 代码编写充分的测试
- 使用 Miri 等工具验证内存安全

### 4. 注释原则
- 为每个 unsafe 块添加注释
- 说明为什么需要 unsafe
- 说明如何保证安全

## 常见的 unsafe 使用场景

1. **FFI**：与 C/C++ 代码交互
2. **性能优化**：手动优化关键路径
3. **底层系统编程**：操作系统、驱动程序
4. **自定义智能指针**：实现特殊的内存管理
5. **SIMD 指令**：使用向量化指令
6. **硬件交互**：直接操作硬件

## 安全检查清单

在编写 unsafe 代码时，问自己：

1. [ ] 是否真的需要 unsafe？
2. [ ] 是否有更安全的替代方案？
3. [ ] 是否确保了内存安全？
4. [ ] 是否避免了数据竞争？
5. [ ] 是否正确处理了生命周期？
6. [ ] 是否有充分的测试？
7. [ ] 是否有清晰的文档？

## 工具推荐

- **Miri**：检测未定义行为
- **Clippy**：代码风格检查
- **Valgrind**：内存错误检测
- **AddressSanitizer**：地址消毒剂

## 总结

Unsafe 代码是 Rust 的强大特性，但也是危险的。记住：

> "With great power comes great responsibility."

使用 unsafe 时要格外小心，确保：
1. 内存安全
2. 数据竞争安全
3. 生命周期正确
4. 有充分的测试和文档
