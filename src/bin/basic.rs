use rust_learning::{basics, cpp_comparison, concurrency, advanced, macro_demo, error_handling};

fn main() {
    println!("=== Rust学习之旅（基础模式）===");
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

    println!("\n=== 学习完成 ===");
}
