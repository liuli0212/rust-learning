//! 异步编程示例
//!
//! 演示Rust中的async/await语法和Tokio运行时

#[cfg(feature = "async")]
use std::time::Duration;

/// 基本async/await语法
#[cfg(feature = "async")]
pub async fn basic_async_await() {
    println!("  === 基本async/await语法 ===");

    // 1. 基本异步函数
    async fn hello() -> String {
        "Hello, async!".to_string()
    }

    let result = hello().await;
    println!("  异步函数结果: {}", result);

    // 2. 异步代码块
    let async_block = async {
        println!("  异步代码块执行中...");
        "异步代码块结果".to_string()
    };
    let block_result = async_block.await;
    println!("  异步代码块结果: {}", block_result);
}

/// 并发执行
#[cfg(feature = "async")]
pub async fn concurrent_execution() {
    println!("  === 并发执行 ===");

    use tokio::time::sleep;

    // 1. 并行执行多个异步任务
    let task1 = async {
        sleep(Duration::from_millis(100)).await;
        println!("  任务1完成");
        "任务1结果".to_string()
    };

    let task2 = async {
        sleep(Duration::from_millis(200)).await;
        println!("  任务2完成");
        "任务2结果".to_string()
    };

    // 使用join!同时执行
    let (result1, result2) = tokio::join!(task1, task2);
    println!("  结果1: {}, 结果2: {}", result1, result2);

    // 2. 使用join_all执行多个任务
    let tasks = vec![
        tokio::spawn(async {
            sleep(Duration::from_millis(50)).await;
            "任务A".to_string()
        }),
        tokio::spawn(async {
            sleep(Duration::from_millis(100)).await;
            "任务B".to_string()
        }),
        tokio::spawn(async {
            sleep(Duration::from_millis(150)).await;
            "任务C".to_string()
        }),
    ];

    let results: Vec<String> = futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    println!("  所有任务结果: {:?}", results);
}

/// 异步通道
#[cfg(feature = "async")]
pub async fn async_channels() {
    println!("  === 异步通道 ===");

    use tokio::sync::mpsc;

    let (tx, mut rx) = mpsc::channel(10);

    // 发送任务
    let sender = tokio::spawn(async move {
        for i in 0..5 {
            tx.send(format!("消息 {}", i)).await.unwrap();
            println!("  发送: 消息 {}", i);
        }
    });

    // 接收任务
    let receiver = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("  接收: {}", msg);
        }
    });

    let _ = tokio::join!(sender, receiver);
}

/// 异步超时控制
#[cfg(feature = "async")]
pub async fn timeout_control() {
    println!("  === 异步超时控制 ===");

    use tokio::time::{sleep, timeout};

    // 1. 成功的情况
    let fast_task = async {
        sleep(Duration::from_millis(50)).await;
        "快速任务完成".to_string()
    };

    match timeout(Duration::from_millis(100), fast_task).await {
        Ok(result) => println!("  快速任务: {}", result),
        Err(_) => println!("  快速任务超时"),
    }

    // 2. 超时的情况
    let slow_task = async {
        sleep(Duration::from_millis(200)).await;
        "慢任务完成".to_string()
    };

    match timeout(Duration::from_millis(100), slow_task).await {
        Ok(result) => println!("  慢任务: {}", result),
        Err(_) => println!("  慢任务超时"),
    }
}

/// 异步锁
#[cfg(feature = "async")]
pub async fn async_locks() {
    println!("  === 异步锁 ===");

    use tokio::sync::Mutex;
    use std::sync::Arc;

    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let mut lock = data_clone.lock().await;
            *lock += 1;
            println!("  线程{} 增加后: {}", i, *lock);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let final_value = *data.lock().await;
    println!("  最终值: {}", final_value);
}

/// 异步文件操作
#[cfg(feature = "async")]
pub async fn async_file_operations() {
    println!("  === 异步文件操作 ===");

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::fs;

    // 1. 读取文件
    match fs::read_to_string("Cargo.toml").await {
        Ok(content) => println!("  文件读取成功，长度: {}", content.len()),
        Err(e) => println!("  文件读取失败: {}", e),
    }

    // 2. 写入文件
    let write_result = fs::write("/tmp/test_async.txt", "异步写入测试").await;
    match write_result {
        Ok(_) => println!("  文件写入成功"),
        Err(e) => println!("  文件写入失败: {}", e),
    }

    // 3. 流式读取
    match fs::File::open("Cargo.toml").await {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).await.unwrap();
            println!("  流式读取完成，字节数: {}", buffer.len());
        }
        Err(e) => println!("  打开文件失败: {}", e),
    }
}

/// 异步HTTP请求
#[cfg(feature = "async")]
pub async fn async_http_requests() {
    println!("  === 异步HTTP请求 ===");

    // 1. 模拟异步HTTP请求
    let mock_request = async {
        println!("  发送HTTP请求...");
        tokio::time::sleep(Duration::from_millis(100)).await;
        println!("  收到响应");
        "HTTP响应数据".to_string()
    };

    let result = mock_request.await;
    println!("  HTTP请求结果: {}", result);

    // 2. 并行发送多个HTTP请求
    println!("\n  并行发送多个请求:");
    let requests = vec![
        tokio::spawn(async {
            println!("  请求1: 发送...");
            tokio::time::sleep(Duration::from_millis(50)).await;
            println!("  请求1: 完成");
            "响应1".to_string()
        }),
        tokio::spawn(async {
            println!("  请求2: 发送...");
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("  请求2: 完成");
            "响应2".to_string()
        }),
        tokio::spawn(async {
            println!("  请求3: 发送...");
            tokio::time::sleep(Duration::from_millis(150)).await;
            println!("  请求3: 完成");
            "响应3".to_string()
        }),
    ];

    let results: Vec<String> = futures::future::join_all(requests)
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    println!("  所有请求结果: {:?}", results);

    // 3. 带超时的HTTP请求
    println!("\n  带超时的请求:");
    use tokio::time::timeout;

    let fast_request = async {
        println!("  快速请求: 发送...");
        tokio::time::sleep(Duration::from_millis(50)).await;
        println!("  快速请求: 完成");
        "快速响应".to_string()
    };

    match timeout(Duration::from_millis(100), fast_request).await {
        Ok(result) => println!("  快速请求成功: {}", result),
        Err(_) => println!("  快速请求超时"),
    }

    let slow_request = async {
        println!("  慢速请求: 发送...");
        tokio::time::sleep(Duration::from_millis(200)).await;
        println!("  慢速请求: 完成");
        "慢速响应".to_string()
    };

    match timeout(Duration::from_millis(100), slow_request).await {
        Ok(result) => println!("  慢速请求成功: {}", result),
        Err(_) => println!("  慢速请求超时"),
    }

    // 4. 错误处理示例
    println!("\n  错误处理示例:");
    async fn fetch_with_retry(url: &str, max_retries: u32) -> Result<String, String> {
        for attempt in 0..max_retries {
            println!("  尝试 {} (URL: {})", attempt + 1, url);

            // 模拟可能失败的请求
            if attempt < 2 {
                println!("  请求失败，准备重试...");
                tokio::time::sleep(Duration::from_millis(50)).await;
                continue;
            }

            println!("  请求成功");
            return Ok(format!("从 {} 获取的数据", url));
        }

        Err(format!("请求 {} 失败，超过最大重试次数", url))
    }

    match fetch_with_retry("https://api.example.com/data", 3).await {
        Ok(data) => println!("  成功: {}", data),
        Err(e) => println!("  失败: {}", e),
    }

    // 5. 并发限制（模拟并发请求）
    println!("\n  并发限制示例:");
    use futures::stream::{self, StreamExt};

    let urls = vec!["url1", "url2", "url3", "url4", "url5"];

    let results: Vec<String> = stream::iter(urls)
        .map(|url| async move {
            println!("  处理: {}", url);
            tokio::time::sleep(Duration::from_millis(30)).await;
            format!("{} 的响应", url)
        })
        .buffer_unordered(2)  // 最多同时处理2个请求
        .collect()
        .await;

    println!("  并发处理结果: {:?}", results);
}

/// 异步流处理
#[cfg(feature = "async")]
pub async fn async_stream_processing() {
    println!("  === 异步流处理 ===");

    use futures::stream::{self, StreamExt};

    // 1. 创建异步流
    let stream = stream::iter(vec![1, 2, 3, 4, 5])
        .map(|x| {
            println!("  处理数字: {}", x);
            x * 2
        })
        .filter(|x| {
            let result = *x > 5;
            async move { result }
        });

    // 2. 收集结果
    let results: Vec<i32> = stream.collect().await;
    println!("  过滤后的结果: {:?}", results);

    // 3. 使用for_each
    let stream2 = stream::iter(vec!["a", "b", "c"]);
    stream2.for_each(|item| async move {
        println!("  处理项目: {}", item);
    }).await;
}

/// 异步错误处理
#[cfg(feature = "async")]
pub async fn async_error_handling() {
    println!("  === 异步错误处理 ===");

    use tokio::time::timeout;

    // 异步函数返回Result
    async fn fetch_data(delay_ms: u64) -> Result<String, String> {
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
        if delay_ms > 100 {
            Err("超时".to_string())
        } else {
            Ok("数据获取成功".to_string())
        }
    }

    // 使用?操作符
    async fn process_data() -> Result<String, String> {
        let data = fetch_data(50).await?;
        Ok(format!("处理: {}", data))
    }

    match process_data().await {
        Ok(result) => println!("  处理结果: {}", result),
        Err(e) => println!("  处理失败: {}", e),
    }

    // 超时处理
    match tokio::time::timeout(Duration::from_millis(100), fetch_data(200)).await {
        Ok(Ok(data)) => println!("  成功: {}", data),
        Ok(Err(e)) => println!("  错误: {}", e),
        Err(_) => println!("  超时"),
    }
}

/// 异步定时任务
#[cfg(feature = "async")]
pub async fn async_scheduled_tasks() {
    println!("  === 异步定时任务 ===");

    use tokio::time::{interval, sleep};

    // 1. 间隔执行
    let mut interval = interval(Duration::from_millis(200));

    for i in 0..3 {
        interval.tick().await;
        println!("  定时任务执行第 {} 次", i + 1);
    }

    // 2. 延迟执行
    println!("  开始延迟任务...");
    sleep(Duration::from_millis(300)).await;
    println!("  延迟任务完成");
}

/// 异步共享状态
#[cfg(feature = "async")]
pub async fn async_shared_state() {
    println!("  === 异步共享状态 ===");

    use tokio::sync::RwLock;
    use std::sync::Arc;

    let data = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // 多个读取者
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let read_lock = data_clone.read().await;
            println!("  读取者{} 读取到: {}", i, *read_lock);
        });
        handles.push(handle);
    }

    // 一个写入者
    let data_clone = Arc::clone(&data);
    let handle = tokio::spawn(async move {
        let mut write_lock = data_clone.write().await;
        *write_lock = 42;
        println!("  写入者写入: {}", *write_lock);
    });
    handles.push(handle);

    for handle in handles {
        handle.await.unwrap();
    }

    let final_value = *data.read().await;
    println!("  最终值: {}", final_value);
}

/// 运行所有异步示例
#[cfg(feature = "async")]
pub async fn run_examples() {
    basic_async_await().await;
    concurrent_execution().await;
    async_channels().await;
    timeout_control().await;
    async_locks().await;
    async_file_operations().await;
    async_http_requests().await;
    async_stream_processing().await;
    async_error_handling().await;
    async_scheduled_tasks().await;
    async_shared_state().await;
}
