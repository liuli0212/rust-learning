//! 错误处理示例
//!
//! 演示Rust中Result和Option的使用，以及自定义错误类型

use std::fmt;

/// 自定义错误类型
#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    ParseError(String),
    ConfigError(String),
    NetworkError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO错误: {}", e),
            AppError::ParseError(e) => write!(f, "解析错误: {}", e),
            AppError::ConfigError(e) => write!(f, "配置错误: {}", e),
            AppError::NetworkError(e) => write!(f, "网络错误: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

// 从标准错误类型转换
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

// serde_json错误转换（可选）
// impl From<serde_json::Error> for AppError {
//     fn from(err: serde_json::Error) -> Self {
//         AppError::ParseError(err.to_string())
//     }
// }

/// 基本Result使用
pub fn basic_result_usage() {
    println!("  === 基本Result使用 ===");

    // 1. 创建Result
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("除数不能为零"))
        } else {
            Ok(a / b)
        }
    }

    // 2. match处理
    match divide(10, 2) {
        Ok(result) => println!("  10 / 2 = {}", result),
        Err(e) => println!("  错误: {}", e),
    }

    // 3. unwrap/expect
    let result1 = divide(10, 2).unwrap();
    println!("  unwrap结果: {}", result1);

    // let result2 = divide(10, 0).expect("除法失败");
    // println!("  expect结果: {}", result2);
}

/// ? 操作符使用
pub fn question_mark_operator() {
    println!("  === ? 操作符使用 ===");

    // 使用?操作符传播错误
    fn read_file_content(path: &str) -> Result<String, AppError> {
        let content = std::fs::read_to_string(path)?;  // 自动传播IO错误
        Ok(content)
    }

    // 链式使用?
    fn process_config(path: &str) -> Result<String, AppError> {
        let content = read_file_content(path)?;
        // 模拟解析
        if content.is_empty() {
            return Err(AppError::ConfigError("配置文件为空".to_string()));
        }
        Ok(content)
    }

    // 测试
    match process_config("Cargo.toml") {
        Ok(content) => println!("  配置内容长度: {}", content.len()),
        Err(e) => println!("  读取配置失败: {}", e),
    }
}

/// 自定义错误类型
pub fn custom_error_types() {
    println!("  === 自定义错误类型 ===");

    fn validate_age(age: i32) -> Result<(), AppError> {
        if age < 0 {
            Err(AppError::ParseError("年龄不能为负数".to_string()))
        } else if age > 150 {
            Err(AppError::ConfigError("年龄超过合理范围".to_string()))
        } else {
            Ok(())
        }
    }

    match validate_age(25) {
        Ok(_) => println!("  年龄验证通过"),
        Err(e) => println!("  验证失败: {}", e),
    }

    match validate_age(-5) {
        Ok(_) => println!("  年龄验证通过"),
        Err(e) => println!("  验证失败: {}", e),
    }
}

/// Option和Result的转换
pub fn option_result_conversion() {
    println!("  === Option和Result的转换 ===");

    // Option转Result
    let some_value: Option<i32> = Some(42);
    let result1: Result<i32, &str> = some_value.ok_or("值不存在");
    println!("  Option转Result: {:?}", result1);

    let none_value: Option<i32> = None;
    let result2: Result<i32, &str> = none_value.ok_or("值不存在");
    println!("  None转Result: {:?}", result2);

    // Result转Option
    let ok_result: Result<i32, &str> = Ok(42);
    let option1: Option<i32> = ok_result.ok();
    println!("  Ok转Option: {:?}", option1);

    let err_result: Result<i32, &str> = Err("错误");
    let option2: Option<i32> = err_result.ok();
    println!("  Err转Option: {:?}", option2);
}

/// 错误处理组合器
pub fn error_combinators() {
    println!("  === 错误处理组合器 ===");

    // 1. map_err - 转换错误类型
    fn divide_with_custom_error(a: i32, b: i32) -> Result<i32, AppError> {
        if b == 0 {
            Err(AppError::ConfigError("除数不能为零".to_string()))
        } else {
            Ok(a / b)
        }
    }

    let result = divide_with_custom_error(10, 2);
    println!("  除法结果: {:?}", result);

    // 2. and_then - 链式操作
    fn process_number(n: i32) -> Result<i32, AppError> {
        if n < 0 {
            Err(AppError::ParseError("负数".to_string()))
        } else {
            Ok(n * 2)
        }
    }

    let result2 = divide_with_custom_error(10, 2)
        .and_then(process_number);
    println!("  链式处理结果: {:?}", result2);

    // 3. or_else - 错误处理
    let result3: Result<i32, AppError> = divide_with_custom_error(10, 0)
        .or_else(|_| Ok(999));  // 出错时返回默认值
    println!("  orElse结果: {:?}", result3);
}

/// 实际应用：配置文件解析
pub fn config_parsing_example() {
    println!("  === 配置文件解析示例 ===");

    #[derive(Debug)]
    struct Config {
        host: String,
        port: u16,
    }

    fn parse_config(content: &str) -> Result<Config, AppError> {
        let lines: Vec<&str> = content.lines().collect();
        
        let mut host = None;
        let mut port = None;

        for line in lines {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                match parts[0].trim() {
                    "host" => host = Some(parts[1].trim().to_string()),
                    "port" => {
                        port = Some(parts[1].trim().parse::<u16>()
                            .map_err(|e| AppError::ParseError(e.to_string()))?);
                    }
                    _ => {}
                }
            }
        }

        Ok(Config {
            host: host.ok_or(AppError::ConfigError("缺少host配置".to_string()))?,
            port: port.ok_or(AppError::ConfigError("缺少port配置".to_string()))?,
        })
    }

    let config_content = "host=localhost\nport=8080";
    match parse_config(config_content) {
        Ok(config) => println!("  配置解析成功: {}:{}", config.host, config.port),
        Err(e) => println!("  配置解析失败: {}", e),
    }

    let invalid_config = "host=localhost\nport=invalid";
    match parse_config(invalid_config) {
        Ok(config) => println!("  配置解析成功: {}:{}", config.host, config.port),
        Err(e) => println!("  配置解析失败: {}", e),
    }
}

/// 错误处理最佳实践
pub fn error_handling_best_practices() {
    println!("  === 错误处理最佳实践 ===");

    // 1. 使用自定义错误类型
    // 2. 实现Display和Error trait
    // 3. 使用?操作符传播错误
    // 4. 在顶层处理错误
    // 5. 提供有意义的错误信息

    fn process_data(data: &str) -> Result<String, AppError> {
        if data.is_empty() {
            return Err(AppError::ParseError("数据不能为空".to_string()));
        }
        Ok(format!("处理后的数据: {}", data))
    }

    // 顶层错误处理
    fn main_logic() -> Result<(), AppError> {
        let result = process_data("test data")?;
        println!("  {}", result);
        Ok(())
    }

    match main_logic() {
        Ok(_) => println!("  执行成功"),
        Err(e) => println!("  执行失败: {}", e),
    }
}

/// 运行所有错误处理示例
pub fn run_examples() {
    basic_result_usage();
    question_mark_operator();
    custom_error_types();
    option_result_conversion();
    error_combinators();
    config_parsing_example();
    error_handling_best_practices();
}
