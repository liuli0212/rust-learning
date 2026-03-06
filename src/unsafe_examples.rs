//! Unsafe 代码示例
//!
//! 演示 Rust 中 unsafe 代码块的使用场景和注意事项
//!
//! # 安全原则
//! - 只在必要时使用 unsafe
//! - 尽量将 unsafe 代码封装在安全的 API 中
//! - 为 unsafe 代码添加详细的文档和注释
//! - 确保 unsafe 代码的正确性

use std::slice;

/// 1. 解引用原始指针
pub fn raw_pointer_examples() {
    println!("  === 原始指针示例 ===");

    // 创建一个可变的整数
    let mut num = 42;
    
    // 创建原始指针
    let raw_ptr: *const i32 = &num as *const i32;
    let raw_mut_ptr: *mut i32 = &mut num as *mut i32;

    println!("  原始指针地址: {:?}", raw_ptr);
    println!("  可变原始指针地址: {:?}", raw_mut_ptr);

    // 解引用原始指针（需要 unsafe）
    unsafe {
        println!("  通过原始指针读取值: {}", *raw_ptr);
        println!("  通过可变原始指针读取值: {}", *raw_mut_ptr);

        // 修改值
        *raw_mut_ptr = 100;
        println!("  修改后 num 的值: {}", num);
    }

    // 空指针示例
    let null_ptr: *const i32 = std::ptr::null();
    println!("  空指针: {:?}", null_ptr);
    
    // 检查指针是否为空（is_null 是安全的）
    if null_ptr.is_null() {
        println!("  指针为空");
    }
}

/// 2. 调用 unsafe 函数
pub fn unsafe_function_examples() {
    println!("  === Unsafe 函数示例 ===");

    // 定义一个 unsafe 函数
    unsafe fn dangerous_function(x: i32, y: i32) -> i32 {
        // 这里可以执行不安全的操作
        x + y
    }

    // 调用 unsafe 函数
    let result = unsafe { dangerous_function(10, 20) };
    println!("  unsafe 函数结果: {}", result);

    // 实际应用：调用 C 语言函数
    // extern "C" {
    //     fn abs(input: i32) -> i32;
    // }
    // let result = unsafe { abs(-42) };
    // println!("  C 函数 abs(-42) = {}", result);
}

/// 3. 创建切片的不安全方式
pub fn slice_examples() {
    println!("  === 切片示例 ===");

    let arr = [1, 2, 3, 4, 5];
    
    // 安全的方式创建切片
    let safe_slice = &arr[1..4];
    println!("  安全切片: {:?}", safe_slice);

    // 不安全的方式创建切片
    unsafe {
        let ptr = arr.as_ptr();
        let unsafe_slice = slice::from_raw_parts(ptr.add(1), 3);
        println!("  不安全切片: {:?}", unsafe_slice);
    }

    // 注意：不安全切片需要确保：
    // 1. 指针指向有效的内存
    // 2. 长度不超过数组边界
    // 3. 内存不会在切片使用期间被释放
}

/// 4. 联合体（Union）的不安全访问
pub fn union_examples() {
    println!("  === 联合体示例 ===");

    // 定义联合体
    #[derive(Copy, Clone)]
    union Data {
        integer: i32,
        float: f32,
        bytes: [u8; 4],
    }

    let mut data = Data { integer: 42 };
    
    // 读取联合体字段（需要 unsafe）
    unsafe {
        println!("  整数形式: {}", data.integer);
        println!("  浮点形式: {}", data.float);
        println!("  字节数组: {:?}", data.bytes);
    }

    // 写入联合体字段
    unsafe {
        data.float = 3.14;
        println!("  写入浮点后，整数形式: {}", data.integer);
    }
}

/// 5. 内存操作示例
pub fn memory_operations() {
    println!("  === 内存操作示例 ===");

    // 分配内存（类似 C 的 malloc）
    let size = 10;
    let layout = std::alloc::Layout::from_size_align(size * std::mem::size_of::<i32>(), 4).unwrap();
    
    unsafe {
        // 分配内存
        let ptr = std::alloc::alloc(layout) as *mut i32;
        
        if ptr.is_null() {
            panic!("内存分配失败");
        }

        // 写入数据
        for i in 0..size {
            *ptr.add(i) = (i as i32) * 2;
        }

        // 读取数据
        println!("  分配的内存内容:");
        for i in 0..size {
            print!("  {}", *ptr.add(i));
        }
        println!();

        // 释放内存
        std::alloc::dealloc(ptr as *mut u8, layout);
    }
}

/// 6. 静态变量的不安全访问
pub fn static_variable_examples() {
    println!("  === 静态变量示例 ===");

    // 静态可变变量（需要 unsafe）
    static mut COUNTER: i32 = 0;

    // 修改静态变量
    unsafe {
        COUNTER += 1;
        // 注意：这里创建了共享引用，这是不推荐的
        // 更好的做法是直接使用值，或者使用 Atomic 类型
        let counter_value = COUNTER;
        println!("  计数器: {}", counter_value);
    }

    // 多线程中使用静态变量（需要特别小心）
    // 通常应该使用 Mutex 或 Atomic 类型代替
}

/// 7. 实际应用：实现一个简单的 Vec
pub fn custom_vec_example() {
    println!("  === 自定义 Vec 示例 ===");

    // 简化的自定义 Vec 实现
    struct CustomVec<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }

    impl<T> CustomVec<T> {
        fn new() -> Self {
            Self {
                ptr: std::ptr::null_mut(),
                len: 0,
                capacity: 0,
            }
        }

        fn push(&mut self, value: T) {
            if self.len == self.capacity {
                // 需要扩容
                let new_capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
                let new_layout = std::alloc::Layout::from_size_align(
                    new_capacity * std::mem::size_of::<T>(),
                    std::mem::align_of::<T>(),
                ).unwrap();

                unsafe {
                    let new_ptr = if self.ptr.is_null() {
                        std::alloc::alloc(new_layout) as *mut T
                    } else {
                        std::alloc::realloc(self.ptr as *mut u8, self.layout(), new_layout.size()) as *mut T
                    };

                    if new_ptr.is_null() {
                        panic!("内存分配失败");
                    }

                    self.ptr = new_ptr;
                    self.capacity = new_capacity;
                }
            }

            // 写入值
            unsafe {
                std::ptr::write(self.ptr.add(self.len), value);
                self.len += 1;
            }
        }

        fn layout(&self) -> std::alloc::Layout {
            std::alloc::Layout::from_size_align(
                self.capacity * std::mem::size_of::<T>(),
                std::mem::align_of::<T>(),
            ).unwrap()
        }

        fn get(&self, index: usize) -> Option<&T> {
            if index >= self.len {
                return None;
            }
            unsafe { Some(&*self.ptr.add(index)) }
        }

        fn len(&self) -> usize {
            self.len
        }
    }

    impl<T> Drop for CustomVec<T> {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                unsafe {
                    // 释放所有元素
                    for i in 0..self.len {
                        std::ptr::drop_in_place(self.ptr.add(i));
                    }
                    // 释放内存
                    std::alloc::dealloc(self.ptr as *mut u8, self.layout());
                }
            }
        }
    }

    // 使用自定义 Vec
    let mut vec = CustomVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("  自定义 Vec 长度: {}", vec.len());
    for i in 0..vec.len() {
        if let Some(value) = vec.get(i) {
            println!("  vec[{}] = {}", i, value);
        }
    }
}

/// 8. FFI（外部函数接口）示例
pub fn ffi_examples() {
    println!("  === FFI 示例 ===");

    // 声明 C 函数
    extern "C" {
        fn strlen(s: *const i8) -> usize;
    }

    // 创建 C 字符串
    let c_string = b"Hello, FFI!\0"; // null 结尾的字节数组
    
    unsafe {
        let len = strlen(c_string.as_ptr() as *const i8);
        println!("  C 字符串长度: {}", len);
    }

    // 注意：实际使用中，应该使用 libc crate 或更安全的绑定
}

/// 9. 内联汇编（仅在某些平台上支持）
pub fn inline_asm_examples() {
    println!("  === 内联汇编示例 ===");

    // 注意：内联汇编是平台特定的，这里仅展示概念
    // 在 x86_64 上，可以这样写：
    
    // unsafe {
    //     asm!(
    //         "mov rax, 42",
    //         "mov rdi, rax",
    //         out("rdi") _,
    //     );
    // }

    println!("  内联汇编需要特定平台支持");
    println!("  在 x86_64 上可以使用 asm! 宏");
}

/// 10. 安全封装示例
pub fn safe_wrapper_examples() {
    println!("  === 安全封装示例 ===");

    // 不安全的内部实现，安全的外部接口
    struct SafeVec<T> {
        data: *mut T,
        len: usize,
        capacity: usize,
    }

    impl<T> SafeVec<T> {
        pub fn new() -> Self {
            Self {
                data: std::ptr::null_mut(),
                len: 0,
                capacity: 0,
            }
        }

        // 安全的公共接口
        pub fn push(&mut self, value: T) {
            // 内部使用 unsafe，但对外是安全的
            self.push_internal(value);
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            if index >= self.len {
                return None;
            }
            unsafe { Some(&*self.data.add(index)) }
        }

        // 私有的 unsafe 实现
        fn push_internal(&mut self, value: T) {
            if self.len == self.capacity {
                self.grow();
            }
            unsafe {
                std::ptr::write(self.data.add(self.len), value);
                self.len += 1;
            }
        }

        fn grow(&mut self) {
            let new_capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
            let new_layout = std::alloc::Layout::from_size_align(
                new_capacity * std::mem::size_of::<T>(),
                std::mem::align_of::<T>(),
            ).unwrap();

            unsafe {
                let new_ptr = if self.data.is_null() {
                    std::alloc::alloc(new_layout) as *mut T
                } else {
                    let old_layout = std::alloc::Layout::from_size_align(
                        self.capacity * std::mem::size_of::<T>(),
                        std::mem::align_of::<T>(),
                    ).unwrap();
                    std::alloc::realloc(self.data as *mut u8, old_layout, new_layout.size()) as *mut T
                };

                if new_ptr.is_null() {
                    panic!("内存分配失败");
                }

                self.data = new_ptr;
                self.capacity = new_capacity;
            }
        }
    }

    impl<T> Drop for SafeVec<T> {
        fn drop(&mut self) {
            if !self.data.is_null() {
                unsafe {
                    for i in 0..self.len {
                        std::ptr::drop_in_place(self.data.add(i));
                    }
                    let layout = std::alloc::Layout::from_size_align(
                        self.capacity * std::mem::size_of::<T>(),
                        std::mem::align_of::<T>(),
                    ).unwrap();
                    std::alloc::dealloc(self.data as *mut u8, layout);
                }
            }
        }
    }

    // 使用安全的封装
    let mut vec = SafeVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("  安全封装的 Vec:");
    for i in 0..3 {
        if let Some(value) = vec.get(i) {
            println!("    vec[{}] = {}", i, value);
        }
    }
}

/// 11. 内存对齐示例
pub fn alignment_examples() {
    println!("  === 内存对齐示例 ===");

    // 检查类型对齐
    println!("  i32 对齐: {} 字节", std::mem::align_of::<i32>());
    println!("  f64 对齐: {} 字节", std::mem::align_of::<f64>());
    println!("  [u8; 16] 对齐: {} 字节", std::mem::align_of::<[u8; 16]>());

    // 创建对齐的内存
    #[repr(align(64))]
    struct AlignedData {
        data: [u8; 128],
    }

    let aligned = AlignedData { data: [0; 128] };
    println!("  对齐结构体的地址: {:p}", &aligned);
    println!("  对齐大小: {} 字节", std::mem::size_of::<AlignedData>());

    // 检查地址是否对齐
    let addr = &aligned as *const AlignedData as usize;
    println!("  地址对齐检查: {}", addr % 64 == 0);
}

/// 12. 类型转换示例
pub fn type_transmutation_examples() {
    println!("  === 类型转换示例 ===");

    // 位模式转换（使用安全的方法）
    let float = 1.0f32;

    // 将 f32 的位模式解释为 u32
    let bits: u32 = float.to_bits();
    println!("  f32 1.0 的位模式: 0x{:08x}", bits);

    // 将 u32 的位模式解释为 f32
    let back_to_float: f32 = f32::from_bits(0x3f800000u32);
    println!("  位模式 0x3f800000 转换为 f32: {}", back_to_float);

    // 如果需要更复杂的类型转换，仍然可以使用 unsafe
    // 例如：将 f64 转换为 [u8; 8]
    let double = 3.14f64;
    let bytes: [u8; 8] = double.to_ne_bytes();
    println!("  f64 3.14 的字节表示: {:?}", bytes);

    // 更复杂的类型转换示例：将 [u8; 8] 转换为 f64
    let back_to_double: f64 = f64::from_ne_bytes(bytes);
    println!("  字节转换回 f64: {}", back_to_double);

    // 使用 unsafe 的 transmute 进行更复杂的类型转换
    // 例如：将 i32 转换为 [u8; 4]
    let number: i32 = 0x12345678;
    let bytes: [u8; 4] = number.to_ne_bytes();
    println!("  i32 0x12345678 的字节表示: {:?}", bytes);

    // 更复杂的类型转换示例：将 [u8; 4] 转换为 i32
    unsafe {
        let back_to_number: i32 = std::mem::transmute(bytes);
        println!("  字节转换回 i32: 0x{:08x}", back_to_number);
    }

    // 注意：类型转换需要确保类型大小相同且兼容
    // 否则可能导致未定义行为
}

/// 运行所有 unsafe 示例
pub fn run_examples() {
    println!("=== Unsafe 代码示例 ===\n");

    raw_pointer_examples();
    println!();

    unsafe_function_examples();
    println!();

    slice_examples();
    println!();

    union_examples();
    println!();

    memory_operations();
    println!();

    static_variable_examples();
    println!();

    custom_vec_example();
    println!();

    ffi_examples();
    println!();

    inline_asm_examples();
    println!();

    safe_wrapper_examples();
    println!();

    alignment_examples();
    println!();

    type_transmutation_examples();
    println!();

    println!("=== Unsafe 示例完成 ===");
    println!("\n重要提示：");
    println!("1. Unsafe 代码绕过了 Rust 的安全检查");
    println!("2. 必须确保内存安全、数据竞争安全等");
    println!("3. 尽量将 unsafe 代码封装在安全的 API 中");
    println!("4. 为 unsafe 代码编写详细的文档和测试");
}
