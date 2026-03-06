//! 简单的 println! 宏实现演示
//!
//! 展示如何实现一个基本的格式化输出宏

/// 简单版本的 println! 宏
/// 
/// 功能：
/// 1. 支持无参数：println!()
/// 2. 支持单个参数：println!("Hello")
/// 3. 支持格式化：println!("Value: {}", value)
/// 4. 支持多个参数：println!("{} + {} = {}", a, b, a + b)
macro_rules! my_println {
    // 1. 无参数的情况
    () => {
        println!();
    };
    
    // 2. 只有字符串，没有占位符
    ($($arg:tt)*) => {
        {
            use std::io::{self, Write};
            let mut stdout = io::stdout();
            write!(stdout, $($arg)*).unwrap();
            writeln!(stdout).unwrap();
        }
    };
}


/// 演示宏的模式匹配
macro_rules! my_println_demo {
    // 模式1: 无参数
    () => {
        println!("无参数调用");
    };
    
    // 模式2: 只有一个字符串参数
    ($msg:expr) => {
        println!("单参数: {}", $msg);
    };
    
    // 模式3: 有格式字符串和参数
    ($fmt:expr, $($arg:tt)*) => {
        println!("格式化: {}", format!($fmt, $($arg)*));
    };
}

/// 演示如何处理不同的 token 类型
macro_rules! debug_print {
    // 打印任意表达式的值
    ($expr:expr) => {
        println!("{} = {:?}", stringify!($expr), $expr);
    };
    
    // 打印多个表达式
    ($($expr:expr),*) => {
        $(
            println!("{} = {:?}", stringify!($expr), $expr);
        )*
    };
}

/// 演示宏的递归
macro_rules! sum {
    // 基础情况：单个数字
    ($num:expr) => {
        $num
    };
    
    // 递归情况：多个数字相加
    ($first:expr, $($rest:expr),*) => {
        $first + sum!($($rest),*)
    };
}

/// 演示宏的重复模式
macro_rules! create_vec {
    // 创建包含重复值的向量
    ($value:expr; $count:expr) => {
        {
            let mut vec = Vec::new();
            for _ in 0..$count {
                vec.push($value);
            }
            vec
        }
    };
    
    // 创建包含多个不同值的向量
    ($($value:expr),*) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($value);
            )*
            vec
        }
    };
}

/// 演示宏的条件逻辑
macro_rules! check_value {
    ($value:expr) => {
        if $value > 0 {
            println!("{} 是正数", $value);
        } else if $value < 0 {
            println!("{} 是负数", $value);
        } else {
            println!("{} 是零", $value);
        }
    };
}

/// 演示宏的类型检查
macro_rules! type_check {
    // 检查是否是整数类型
    ($value:expr, i32) => {
        {
            let _: i32 = $value;
            println!("{} 是 i32 类型", stringify!($value));
        }
    };
    
    // 检查是否是字符串类型
    ($value:expr, &str) => {
        {
            let _: &str = $value;
            println!("{} 是 &str 类型", stringify!($value));
        }
    };
}


/// 运行所有宏演示
pub fn run_examples() {
    println!("=== 简单版本的 println! 宏演示 ===\n");
    
    // 1. 无参数
    println!("1. 无参数:");
    my_println!();
    
    // 2. 只有字符串
    println!("2. 只有字符串:");
    my_println!("Hello, world!");
    
    // 3. 格式化输出
    println!("3. 格式化输出:");
    let name = "Rust";
    let year = 2024;
    my_println!("语言: {}, 年份: {}", name, year);
    
    // 4. 多个参数
    println!("4. 多个参数:");
    let a = 10;
    let b = 20;
    my_println!("{} + {} = {}", a, b, a + b);
    
    println!("\n=== 模式匹配演示 ===\n");
    
    // 5. 不同模式的调用
    println!("5. 不同模式的调用:");
    my_println_demo!();
    my_println_demo!("单个参数");
    my_println_demo!("{} + {} = {}", 5, 3, 5 + 3);
    
    println!("\n=== 调试打印演示 ===\n");
    
    // 6. 调试打印
    println!("6. 调试打印:");
    let x = 42;
    let s = "hello";
    debug_print!(x);
    debug_print!(s);
    debug_print!(x, s, 3.14);
    
    println!("\n=== 递归宏演示 ===\n");
    
    // 7. 递归求和
    println!("7. 递归求和:");
    let sum1 = sum!(5);
    let sum2 = sum!(1, 2, 3, 4, 5);
    let sum3 = sum!(10, 20, 30);
    my_println!("sum!(5) = {}", sum1);
    my_println!("sum!(1, 2, 3, 4, 5) = {}", sum2);
    my_println!("sum!(10, 20, 30) = {}", sum3);
    
    println!("\n=== 重复模式演示 ===\n");
    
    // 8. 创建向量
    println!("8. 创建向量:");
    let vec1 = create_vec![1; 5];  // 重复5次
    let vec2 = create_vec![1, 2, 3, 4, 5];  // 不同值
    my_println!("重复值: {:?}", vec1);
    my_println!("不同值: {:?}", vec2);
    
    println!("\n=== 条件逻辑演示 ===\n");
    
    // 9. 条件检查
    println!("9. 条件检查:");
    check_value!(10);
    check_value!(-5);
    check_value!(0);
    
    println!("\n=== 类型检查演示 ===\n");
    
    // 10. 类型检查
    println!("10. 类型检查:");
    type_check!(42, i32);
    type_check!("hello", &str);
    
    println!("\n=== 宏展开示例 ===\n");

    // 11. 展开示例
    println!("11. 宏展开示例:");
    println!("原始代码: my_println!(\"Value: {{}}\", 42)");
    println!("展开后大致相当于:");
    println!("  {{");
    println!("      use std::io::{{self, Write}};");
    println!("      let mut stdout = io::stdout();");
    println!("      write!(stdout, \"Value: {{}}\", 42).unwrap();");
    println!("      writeln!(stdout).unwrap();");
    println!("  }}");
}

/// 演示宏的高级特性
pub fn advanced_examples() {
    println!("\n=== 高级宏特性 ===\n");
    
    // 演示宏的捕获和重用
    macro_rules! capture_and_reuse {
        // 捕获表达式并多次使用
        ($expr:expr) => {
            {
                let value = $expr;
                println!("第一次使用: {}", value);
                println!("第二次使用: {}", value);
                println!("第三次使用: {}", value);
                value
            }
        };
    }
    
    println!("捕获和重用:");
    let result = capture_and_reuse!(2 + 3);
    println!("返回值: {}", result);
    
    // 演示宏的嵌套
    macro_rules! nested_macro {
        ($($inner:tt)*) => {
            {
                println!("外层宏开始");
                nested_macro_inner!($($inner)*);
                println!("外层宏结束");
            }
        };
    }
    
    macro_rules! nested_macro_inner {
        ($($inner:tt)*) => {
            println!("内层宏: {:?}", stringify!($($inner)*));
        };
    }
    
    println!("\n嵌套宏:");
    nested_macro!(hello world);
}
