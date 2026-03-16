//! 工程化与可观测性 (Observability)
//!
//! 在生产级系统中，println! 是远远不够的。
//! 我们通常使用 tracing 生态进行结构化的、带上下文的日志记录。

use tracing::{debug, error, info, instrument, span, warn, Level};

/// 1. 结构化日志基础
pub fn basic_tracing() {
    println!("  === Tracing 基础日志 ===");
    
    // 注意：在实际应用中， subscriber 的初始化应该在 main() 最开始做一次。
    // 这里为了演示，我们假设它已经被外层环境（或通过 run_examples）初始化。
    
    info!("系统启动成功，版本: {}", "1.0.0");
    warn!("检测到配置缺失，将使用默认回退配置");
    debug!("这行日志只有在开启 DEBUG 级别时才会显示");
    
    let user_id = 42;
    // tracing 允许直接绑定结构化字段（如 user_id），这对日志收集系统（如 ELK, Loki）极度友好
    error!(user_id = user_id, "用户请求处理失败: 数据库连接超时");
}

/// 2. 使用 #[instrument] 宏自动记录函数上下文
/// 这个宏会自动创建一个与函数名同名的 Span，并将所有参数记录为该 Span 的字段。
#[instrument]
fn process_transaction(tx_id: u64, amount: f64) {
    info!("开始处理交易");
    
    if amount > 10000.0 {
        warn!(amount = amount, "检测到大额交易，需人工复核");
    }
    
    // 模拟子操作，手动进入 Span
    let db_span = span!(Level::INFO, "db_update", table = "transactions");
    let _enter = db_span.enter(); // 进入 Span 的生命周期
    
    info!("正在将交易写入数据库...");
    // 离开作用域时 _enter 被 drop，Span 自动结束
}

pub fn span_examples() {
    println!("\n  === Tracing 上下文 (Spans) ===");
    process_transaction(9527, 250.0);
    process_transaction(9528, 50000.0);
}

pub fn run_examples() {
    println!("=== 生产级可观测性 (Tracing) ===");
    
    // 初始化全局 Subscriber（通常在 main 中进行）
    // 这里我们使用 try_init 避免在多次调用时引发 panic
    let _ = tracing_subscriber::fmt()
        .with_max_level(Level::INFO) // 仅显示 INFO 及以上级别
        .try_init();

    basic_tracing();
    span_examples();
    println!("==============================\n");
}
