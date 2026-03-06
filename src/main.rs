mod basics;
mod cpp_comparison;
mod concurrency;
mod advanced;
mod macro_demo;
mod error_handling;
mod unsafe_examples;

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

    // 3. 并发编程（基础部分，不依赖tokio）
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

    // 8. 异步编程
    println!("\n8. 异步编程演示:");
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

    // 3. 并发编程（基础部分，不依赖tokio）
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

    // 8. 异步编程（需要tokio运行时）
    println!("\n8. 异步编程演示:");
    println!("  注意：异步示例需要tokio运行时，这里仅展示概念");
    println!("  实际运行需要: cargo run --features async");

    // 9. Web服务器（需要axum和tokio）
    println!("\n9. Web服务器演示:");
    println!("  注意：Web示例需要axum和tokio，这里仅展示概念");
    println!("  实际运行需要: cargo run --features web");

    // 10. 数据库操作（需要sqlx）
    println!("\n10. 数据库操作演示:");
    println!("  注意：数据库示例需要sqlx，这里仅展示概念");
    println!("  实际运行需要: cargo run --features db");

    println!("\n=== 学习完成 ===");
    println!("\n提示：要运行异步、Web和数据库示例，请使用:");
    println!("  cargo run --features all");
}