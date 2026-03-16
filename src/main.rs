mod basics;
mod cpp_comparison;
mod concurrency;
mod advanced;
mod macro_demo;
mod error_handling;
mod unsafe_examples;
mod memory_layout;
mod advanced_types;
mod smart_pointers;
mod ffi_cxx;
mod observability;

#[cfg(feature = "async")]
mod async_programming;

#[cfg(feature = "web")]
mod web_server;

#[cfg(feature = "db")]
mod database;

#[cfg(feature = "async")]
#[tokio::main]
async fn async_main() {
    println!("=== Rust学习之旅（异步模式）===");
    println!("基于C++高阶编程经验的渐进式学习\n");

    // 1. 基础语法
    println!("1. 基础语法演示:");
    basics::run_examples();

    // 2. C++对比
    println!("\n2. C++对比演示:");
    cpp_comparison::run_examples();

    // 3. 并发编程
    println!("\n3. 并发编程演示:");
    concurrency::run_examples();

    // 4. 高级特性
    println!("\n4. 高级特性演示:");
    advanced::run_examples();

    // 5. 宏演示
    println!("\n5. 宏演示:");
    macro_demo::run_examples();
    macro_demo::advanced_examples();

    // 6. 错误处理
    println!("\n6. 错误处理演示:");
    error_handling::run_examples();

    // 7. Unsafe 代码示例
    println!("\n7. Unsafe 代码演示:");
    unsafe_examples::run_examples();

    // 8. 内存布局与对齐
    println!("\n8. 内存布局与对齐演示:");
    memory_layout::run_examples();

    // 9. 高级类型系统
    println!("\n9. 高级类型系统演示:");
    advanced_types::run_examples();

    // 10. 智能指针与零拷贝
    println!("\n10. 智能指针与零拷贝演示:");
    smart_pointers::run_examples();

    // 11. FFI与C++互操作
    println!("\n11. FFI与C++互操作演示:");
    ffi_cxx::run_examples();

    // 12. 工程化与可观测性
    println!("\n12. 工程化与可观测性演示:");
    observability::run_examples();

    // 13. 异步编程
    println!("\n13. 异步编程演示:");
    async_programming::run_examples().await;

    println!("\n=== 学习完成 ===");
}

fn main() {
    #[cfg(feature = "async")]
    {
        // 使用异步main
        async_main();
        return;
    }

    println!("=== Rust学习之旅 ===");
    println!("基于C++高阶编程经验的渐进式学习\n");

    // 1. 基础语法
    println!("1. 基础语法演示:");
    basics::run_examples();

    // 2. C++对比
    println!("\n2. C++对比演示:");
    cpp_comparison::run_examples();

    // 3. 并发编程
    println!("\n3. 并发编程演示:");
    concurrency::run_examples();

    // 4. 高级特性
    println!("\n4. 高级特性演示:");
    advanced::run_examples();

    // 5. 宏演示
    println!("\n5. 宏演示:");
    macro_demo::run_examples();
    macro_demo::advanced_examples();

    // 6. 错误处理
    println!("\n6. 错误处理演示:");
    error_handling::run_examples();

    // 7. Unsafe 代码示例
    println!("\n7. Unsafe 代码演示:");
    unsafe_examples::run_examples();

    // 8. 内存布局与对齐
    println!("\n8. 内存布局与对齐演示:");
    memory_layout::run_examples();

    // 9. 高级类型系统
    println!("\n9. 高级类型系统演示:");
    advanced_types::run_examples();

    // 10. 智能指针与零拷贝
    println!("\n10. 智能指针与零拷贝演示:");
    smart_pointers::run_examples();

    // 11. FFI与C++互操作
    println!("\n11. FFI与C++互操作演示:");
    ffi_cxx::run_examples();

    // 12. 工程化与可观测性
    println!("\n12. 工程化与可观测性演示:");
    observability::run_examples();

    // 13. 异步编程
    println!("\n13. 异步编程演示:");
    println!("  注意：异步示例需要tokio运行时，这里仅展示概念");
    println!("  实际运行需要: cargo run --features async");

    // 14. Web服务器
    println!("\n14. Web服务器演示:");
    println!("  注意：Web示例需要axum和tokio，这里仅展示概念");
    println!("  实际运行需要: cargo run --features web");

    // 15. 数据库操作
    println!("\n15. 数据库操作演示:");
    println!("  注意：数据库示例需要sqlx，这里仅展示概念");
    println!("  实际运行需要: cargo run --features db");

    println!("\n=== 学习完成 ===");
    println!("\n提示：要运行异步、Web和数据库示例，请使用:");
    println!("  cargo run --features all");
}