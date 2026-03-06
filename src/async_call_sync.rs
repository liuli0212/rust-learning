//!
//! 演示 async 函数调用同步函数

#[cfg(feature = "async")]
use std::time::Duration;

/// 同步函数 - 简单计算
fn calculate_price(quantity: i32, unit_price: f64) -> f64 {
    println!("  同步计算: {} * {}", quantity, unit_price);
    quantity as f64 * unit_price
}

/// 同步函数 - CPU 密集型
fn heavy_computation(data: Vec<i32>) -> i32 {
    println!("  同步计算: 处理 {} 个数据", data.len());
    data.into_iter().sum()
}

/// 同步函数 - 简单字符串处理
fn format_name(first: &str, last: &str) -> String {
    format!("{} {}", first, last)
}

/// async 函数调用同步函数
#[cfg(feature = "async")]
pub async fn async_calls_sync() {
    println!("  === async 函数调用同步函数 ===");

    // 1. 调用简单同步函数
    let price = calculate_price(5, 10.0);
    println!("  计算结果: {}", price);

    // 2. 调用 CPU 密集型同步函数
    let data = vec![1, 2, 3, 4, 5];
    let sum = heavy_computation(data);
    println!("  求和结果: {}", sum);

    // 3. 调用字符串处理同步函数
    let name = format_name("张", "三");
    println!("  格式化结果: {}", name);

    // 4. 混合使用：先调用同步函数，再调用异步函数
    let base_price = calculate_price(10, 20.0);
    println!("  基础价格: {}", base_price);

    // 模拟异步 I/O 操作
    tokio::time::sleep(Duration::from_millis(10)).await;
    println!("  异步操作完成");

    // 再次调用同步函数
    let total = calculate_price(5, base_price);
    println!("  最终价格: {}", total);
}

/// async 函数中调用多个同步函数
#[cfg(feature = "async")]
pub async fn async_calls_multiple_sync() {
    println!("  === async 函数调用多个同步函数 ===");

    // 1. 数据预处理（同步）
    let raw_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filtered_data: Vec<i32> = raw_data.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("  过滤后数据: {:?}", filtered_data);

    // 2. 异步操作（模拟网络请求）
    tokio::time::sleep(Duration::from_millis(50)).await;
    println!("  网络请求完成");

    // 3. 结果处理（同步）
    let result = heavy_computation(filtered_data);
    println!("  最终结果: {}", result);

    // 4. 格式化输出（同步）
    let output = format_name("结果", &result.to_string());
    println!("  格式化输出: {}", output);
}

/// 对比：同步函数调用 async 函数（需要 await）
#[cfg(feature = "async")]
pub async fn sync_calls_async() {
    println!("  === 同步函数调用 async 函数 ===");

    // 同步函数不能直接调用 async 函数，需要在 async 上下文中
    async fn inner_async() -> i32 {
        tokio::time::sleep(Duration::from_millis(10)).await;
        42
    }

    // 在 async 函数中调用 async 函数
    let result = inner_async().await;
    println!("  async 函数调用 async 函数结果: {}", result);
}

/// 性能对比示例
#[cfg(feature = "async")]
pub async fn performance_comparison() {
    println!("  === 性能对比 ===");

    // 同步函数：直接调用，无开销
    let start = std::time::Instant::now();
    let sum: i32 = (0..1000).sum();
    let sync_duration = start.elapsed();
    println!("  同步求和 (1000次): {:?}, 结果: {}", sync_duration, sum);

    // async 函数：有 Future 开销
    let start = std::time::Instant::now();
    let async_sum = async {
        let mut total = 0;
        for i in 0..1000 {
            total += i;
        }
        total
    };
    let result = async_sum.await;
    let async_duration = start.elapsed();
    println!("  async 求和 (1000次): {:?}, 结果: {}", async_duration, result);

    println!("  同步函数更快，因为没有 Future 开销");
}

/// 实际应用场景：Web API 处理
#[cfg(feature = "async")]
pub async fn web_api_example() {
    println!("  === Web API 处理示例 ===");

    // 1. 解析请求（同步）
    fn parse_request(body: &str) -> (String, i32) {
        println!("  解析请求: {}", body);
        ("product".to_string(), 5)
    }

    // 2. 验证参数（同步）
    fn validate_params(product: &str, quantity: i32) -> bool {
        println!("  验证参数: {}, {}", product, quantity);
        quantity > 0
    }

    // 3. 计算价格（同步）
    fn calculate_total(price: f64, quantity: i32) -> f64 {
        println!("  计算总价: {} * {}", price, quantity);
        price * quantity as f64
    }

    // 4. 保存到数据库（异步）
    async fn save_to_db(order_id: &str, total: f64) -> Result<(), String> {
        println!("  保存订单到数据库: {} = {}", order_id, total);
        tokio::time::sleep(Duration::from_millis(10)).await;
        Ok(())
    }

    // 模拟请求体
    let request_body = "product:widget,quantity:5";

    // 处理流程
    let (product, quantity) = parse_request(request_body);

    if validate_params(&product, quantity) {
        let total = calculate_total(10.0, quantity);

        // 异步保存
        let order_id = format!("ORDER-{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        save_to_db(&order_id, total).await.unwrap();

        println!("  订单处理完成: {} = {}", order_id, total);
    } else {
        println!("  参数验证失败");
    }
}

/// 运行所有示例
#[cfg(feature = "async")]
pub async fn run_examples() {
    println!("\n=== async_call_sync 模块示例 ===\n");

    async_calls_sync().await;
    println!();

    async_calls_multiple_sync().await;
    println!();

    sync_calls_async().await;
    println!();

    performance_comparison().await;
    println!();

    web_api_example().await;
    println!();

    println!("=== async_call_sync 模块示例完成 ===\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_calls_sync() {
        async_calls_sync().await;
    }

    #[tokio::test]
    async fn test_async_calls_multiple_sync() {
        async_calls_multiple_sync().await;
    }

    #[tokio::test]
    async fn test_performance_comparison() {
        performance_comparison().await;
    }

    #[tokio::test]
    async fn test_web_api_example() {
        web_api_example().await;
    }
}
