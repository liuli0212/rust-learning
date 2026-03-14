//!
//! 演示 async 函数调用同步函数
//!
//! ## async_trait 用途说明
//!
//! Rust 的 `async fn` 在 trait 中不能直接使用（因为 async 返回的是 `impl Future`，
//! 而 trait 方法要求返回具体类型）。`async_trait` 宏通过将 async 函数转换为返回
//! `Pin<Box<dyn Future>>` 来解决这个问题，使得 trait 可以定义异步方法。
//!
//! 适用场景：
//! - 在 trait 中定义异步方法（如数据库接口、HTTP 客户端接口）
//! - 需要动态分发的异步操作
//! - 跨 crate 的异步抽象

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

// ============================================================================
// async_trait 示例
// ============================================================================
//
// async_trait 宏解决了 Rust 原生不支持在 trait 中使用 async fn 的问题。
// 原因：async fn 返回 impl Future，但 trait 方法需要返回具体类型。
// async_trait 将方法转换为返回 Pin<Box<dyn Future + Send>>，实现动态分发。

/// 示例 1: 定义异步数据库接口 trait
///
/// 使用 #[async_trait] 标记 trait，使得其中的方法可以是 async fn。
/// 实现方只需在 impl 块上也加 #[async_trait] 即可。
#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait AsyncDatabase {
    /// 异步查询：根据 ID 获取数据
    async fn get(&self, id: u64) -> Option<String>;

    /// 异步插入：保存数据并返回新 ID
    async fn insert(&self, data: &str) -> u64;

    /// 异步删除：根据 ID 删除数据
    async fn delete(&self, id: u64) -> bool;
}

/// 示例 2: 实现异步数据库接口（内存版本）
///
/// 演示如何为具体类型实现 async_trait 定义的异步方法。
/// 注意：impl 块上也必须加 #[async_trait]。
#[cfg(feature = "async")]
pub struct InMemoryDb {
    data: std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<u64, String>>>,
    next_id: std::sync::Arc<tokio::sync::Mutex<u64>>,
}

#[cfg(feature = "async")]
impl InMemoryDb {
    pub fn new() -> Self {
        Self {
            data: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
            next_id: std::sync::Arc::new(tokio::sync::Mutex::new(1)),
        }
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl AsyncDatabase for InMemoryDb {
    /// 异步查询：获取指定 ID 的数据
    async fn get(&self, id: u64) -> Option<String> {
        let db = self.data.lock().await;
        db.get(&id).cloned()
    }

    /// 异步插入：分配新 ID 并保存数据
    async fn insert(&self, data: &str) -> u64 {
        let mut id_lock = self.next_id.lock().await;
        let id = *id_lock;
        *id_lock += 1;

        let mut db = self.data.lock().await;
        db.insert(id, data.to_string());
        println!("  [DB] 插入数据 id={}, data={}", id, data);
        id
    }

    /// 异步删除：移除指定 ID 的数据
    async fn delete(&self, id: u64) -> bool {
        let mut db = self.data.lock().await;
        let removed = db.remove(&id).is_some();
        println!("  [DB] 删除 id={}, 结果={}", id, removed);
        removed
    }
}

/// 示例 3: 异步 HTTP 客户端接口
///
/// 演示另一个 async_trait 使用场景：定义通用的 HTTP 客户端接口。
/// 不同的实现（如 reqwest、mock）可以互换使用。
#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait AsyncHttpClient {
    /// 异步 GET 请求
    async fn get(&self, url: &str) -> Result<String, String>;

    /// 异步 POST 请求
    async fn post(&self, url: &str, body: &str) -> Result<String, String>;
}

/// 示例 4: Mock HTTP 客户端实现
///
/// 用于测试的 mock 实现，不依赖真实网络。
#[cfg(feature = "async")]
pub struct MockHttpClient;

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl AsyncHttpClient for MockHttpClient {
    /// 模拟 GET 请求：延迟后返回模拟数据
    async fn get(&self, url: &str) -> Result<String, String> {
        tokio::time::sleep(Duration::from_millis(10)).await;
        println!("  [HTTP] GET {}", url);
        Ok(format!("{{\"url\": \"{}\", \"status\": \"ok\"}}", url))
    }

    /// 模拟 POST 请求：延迟后返回模拟数据
    async fn post(&self, url: &str, body: &str) -> Result<String, String> {
        tokio::time::sleep(Duration::from_millis(10)).await;
        println!("  [HTTP] POST {} body={}", url, body);
        Ok(format!("{{\"url\": \"{}\", \"received\": \"{}\"}}", url, body))
    }
}

/// 示例 5: 使用 async_trait 的动态分发
///
/// 演示如何通过 trait object（dyn AsyncDatabase）进行动态分发。
/// 这是 async_trait 的核心价值：允许在运行时选择不同的实现。
#[cfg(feature = "async")]
pub async fn use_database_dyn(db: &dyn AsyncDatabase) {
    println!("  === 动态分发示例 ===");

    // 插入数据
    let id1 = db.insert("hello").await;
    let id2 = db.insert("world").await;

    // 查询数据
    if let Some(val) = db.get(id1).await {
        println!("  查询到 id={}: {}", id1, val);
    }

    // 删除数据
    let deleted = db.delete(id2).await;
    println!("  删除 id={} 结果: {}", id2, deleted);
}

/// 示例 6: 泛型函数使用 async_trait
///
/// 演示如何编写接受实现了 AsyncDatabase 的泛型函数。
/// 这种方式支持静态分发，性能更好。
#[cfg(feature = "async")]
pub async fn use_database_generic<T: AsyncDatabase>(db: &T) {
    println!("  === 泛型示例 ===");

    let id = db.insert("generic test").await;
    if let Some(val) = db.get(id).await {
        println!("  泛型查询 id={}: {}", id, val);
    }
}

// ============================================================================
// RefCell 内部可变性示例（单线程场景）
// ============================================================================
//
// RefCell 与 Mutex 的区别：
// - Mutex: 线程安全，运行时加锁，适合多线程
// - RefCell: 单线程，编译期借用检查推迟到运行时，零开销
//
// RefCell 通过 borrow() / borrow_mut() 在运行时检查借用规则，
// 违反规则（如同时存在两个 mutable borrow）会 panic。

/// 示例 7: 使用 RefCell 实现内部可变性（单线程）
///
/// 演示在单线程中，通过 RefCell 实现 &self 方法修改内部状态。
/// 这是 C++ 中 `mutable` 关键字的 Rust 等价物，但更安全。
#[cfg(feature = "async")]
pub struct RefCellCounter {
    // RefCell 包装内部状态，允许在 &self 方法中修改
    count: std::cell::RefCell<i32>,
    history: std::cell::RefCell<Vec<String>>,
}

#[cfg(feature = "async")]
impl RefCellCounter {
    pub fn new() -> Self {
        Self {
            count: std::cell::RefCell::new(0),
            history: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// &self 方法，但可以修改内部状态
    /// 这在 C++ 中需要 `mutable` 字符或非 const 方法
    pub fn increment(&self) {
        // borrow_mut() 获取可变借用，运行时检查
        let mut count = self.count.borrow_mut();
        *count += 1;

        let mut history = self.history.borrow_mut();
        history.push(format!("increment -> {}", *count));
    }

    /// &self 方法，读取状态
    pub fn get_count(&self) -> i32 {
        *self.count.borrow()
    }

    /// &self 方法，读取历史记录
    pub fn get_history(&self) -> Vec<String> {
        self.history.borrow().clone()
    }

    /// 演示 RefCell 的运行时借用检查
    /// 如果同时存在两个 mutable borrow，会 panic
    pub fn demonstrate_borrow_rules(&self) {
        println!("  === RefCell 借用规则演示 ===");

        // 多个不可变借用是允许的
        let count1 = self.count.borrow();
        let count2 = self.count.borrow();
        println!("  两个不可变借用: count1={}, count2={}", *count1, *count2);
        drop(count1);
        drop(count2);

        // 一个可变借用
        {
            let mut count = self.count.borrow_mut();
            *count += 10;
            println!("  可变借用后: {}", *count);
        } // borrow_mut 在这里释放

        // 释放后可以再次借用
        let count = self.count.borrow();
        println!("  释放后再次借用: {}", *count);
    }
}

/// 示例 8: RefCell 与 async_trait 结合
///
/// 演示在异步 trait 实现中使用 RefCell（单线程场景）。
/// 注意：RefCell 不是 Send，不能跨线程传递，所以这里用同步方式演示。
#[cfg(feature = "async")]
pub struct RefCellCache {
    cache: std::cell::RefCell<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "async")]
impl RefCellCache {
    pub fn new() -> Self {
        Self {
            cache: std::cell::RefCell::new(std::collections::HashMap::new()),
        }
    }

    /// &self 方法：查询缓存
    pub fn get(&self, key: &str) -> Option<String> {
        self.cache.borrow().get(key).cloned()
    }

    /// &self 方法：设置缓存（修改内部状态）
    pub fn set(&self, key: &str, value: &str) {
        self.cache.borrow_mut().insert(key.to_string(), value.to_string());
    }

    /// &self 方法：清空缓存
    pub fn clear(&self) {
        self.cache.borrow_mut().clear();
    }
}

/// 运行 RefCell 相关示例
#[cfg(feature = "async")]
pub async fn run_refcell_examples() {
    println!("\n=== RefCell 内部可变性示例 ===\n");

    // 1. RefCellCounter 示例
    println!("1. RefCellCounter:");
    let counter = RefCellCounter::new();

    // &self 方法，但能修改内部状态
    counter.increment();
    counter.increment();
    counter.increment();
    println!("  计数: {}", counter.get_count());
    println!("  历史: {:?}", counter.get_history());
    println!();

    // 2. 借用规则演示
    println!("2. 借用规则:");
    counter.demonstrate_borrow_rules();
    println!();

    // 3. RefCellCache 示例
    println!("3. RefCellCache:");
    let cache = RefCellCache::new();
    cache.set("name", "Alice");
    cache.set("age", "30");
    println!("  name: {:?}", cache.get("name"));
    println!("  age: {:?}", cache.get("age"));
    cache.clear();
    println!("  清空后 name: {:?}", cache.get("name"));
    println!();

    println!("=== RefCell 示例完成 ===\n");
}

/// 运行 async_trait 相关示例
#[cfg(feature = "async")]
pub async fn run_async_trait_examples() {
    println!("\n=== async_trait 示例 ===\n");

    // 1. 数据库接口示例
    println!("1. 异步数据库接口:");
    let db = InMemoryDb::new();
    use_database_dyn(&db).await;
    println!();

    // 2. 泛型方式使用数据库
    println!("2. 泛型方式:");
    use_database_generic(&db).await;
    println!();

    // 3. HTTP 客户端接口示例
    println!("3. 异步 HTTP 客户端接口:");
    let client = MockHttpClient;
    match client.get("https://api.example.com/data").await {
        Ok(resp) => println!("  GET 响应: {}", resp),
        Err(e) => println!("  GET 错误: {}", e),
    }
    match client.post("https://api.example.com/data", "{\"key\":\"value\"}").await {
        Ok(resp) => println!("  POST 响应: {}", resp),
        Err(e) => println!("  POST 错误: {}", e),
    }
    println!();

    println!("=== async_trait 示例完成 ===\n");
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

    // 新增：async_trait 示例
    run_async_trait_examples().await;

    // 新增：RefCell 示例
    run_refcell_examples().await;

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

    #[tokio::test]
    async fn test_async_trait_database() {
        let db = InMemoryDb::new();
        let id = db.insert("test data").await;
        assert_eq!(db.get(id).await, Some("test data".to_string()));
        assert!(db.delete(id).await);
        assert_eq!(db.get(id).await, None);
    }

    #[tokio::test]
    async fn test_async_trait_http_client() {
        let client = MockHttpClient;
        let resp = client.get("https://example.com").await.unwrap();
        assert!(resp.contains("example.com"));
    }

    #[tokio::test]
    async fn test_async_trait_dyn_dispatch() {
        let db = InMemoryDb::new();
        use_database_dyn(&db).await;
    }

    #[tokio::test]
    async fn test_async_trait_generic() {
        let db = InMemoryDb::new();
        use_database_generic(&db).await;
    }

    #[tokio::test]
    async fn test_refcell_counter() {
        let counter = RefCellCounter::new();
        counter.increment();
        counter.increment();
        assert_eq!(counter.get_count(), 2);
        let history = counter.get_history();
        assert_eq!(history.len(), 2);
    }

    #[tokio::test]
    async fn test_refcell_cache() {
        let cache = RefCellCache::new();
        cache.set("key1", "value1");
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.get("key2"), None);
        cache.clear();
        assert_eq!(cache.get("key1"), None);
    }

    #[tokio::test]
    async fn test_refcell_borrow_rules() {
        let counter = RefCellCounter::new();
        counter.demonstrate_borrow_rules();
    }
}
