//! 数据库操作示例
//!
//! 演示使用SQLx进行数据库操作

#[cfg(feature = "db")]
use std::time::Duration;

/// 基本数据库连接
#[cfg(feature = "db")]
pub async fn basic_database_connection() {
    println!("  === 基本数据库连接 ===");

    // 模拟数据库连接
    println!("  连接到数据库...");
    println!("  数据库连接成功");
    println!("  执行查询...");
    println!("  查询完成");
}

/// 数据库CRUD操作
#[cfg(feature = "db")]
pub async fn crud_operations() {
    println!("  === 数据库CRUD操作 ===");

    // 1. 创建（Create）
    println!("  创建记录:");
    println!("    INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com')");

    // 2. 读取（Read）
    println!("  读取记录:");
    println!("    SELECT * FROM users WHERE id = 1");

    // 3. 更新（Update）
    println!("  更新记录:");
    println!("    UPDATE users SET email = 'new@example.com' WHERE id = 1");

    // 4. 删除（Delete）
    println!("  删除记录:");
    println!("    DELETE FROM users WHERE id = 1");
}

/// 事务处理
#[cfg(feature = "db")]
pub async fn transaction_example() {
    println!("  === 事务处理 ===");

    println!("  开始事务");
    println!("    BEGIN");

    println!("  执行操作1");
    println!("    INSERT INTO accounts (user_id, balance) VALUES (1, 1000)");

    println!("  执行操作2");
    println!("    UPDATE accounts SET balance = balance - 100 WHERE user_id = 1");

    println!("  提交事务");
    println!("    COMMIT");

    println!("  或者回滚（如果出错）");
    println!("    ROLLBACK");
}

/// 连接池
#[cfg(feature = "db")]
pub async fn connection_pool() {
    println!("  === 连接池 ===");

    println!("  创建连接池:");
    println!("    PgPoolOptions::new()");
    println!("      .max_connections(5)");
    println!("      .connect(\"postgres://user:pass@localhost/db\")");

    println!("  从池中获取连接");
    println!("  使用连接执行查询");
    println!("  连接自动返回到池中");
}

/// 错误处理
#[cfg(feature = "db")]
pub async fn database_error_handling() {
    println!("  === 数据库错误处理 ===");

    // 常见错误类型
    println!("  1. 连接错误 - 数据库服务未启动");
    println!("  2. 查询错误 - SQL语法错误");
    println!("  3. 数据错误 - 违反约束");
    println!("  4. 事务错误 - 死锁");

    println!("  错误处理模式:");
    println!("    match query.execute(&pool).await {{");
    println!("        Ok(_) => println!(\"成功\"),");
    println!("        Err(e) => println!(\"错误: {{:?}}\", e),");
    println!("    }}");
}

/// 数据库迁移
#[cfg(feature = "db")]
pub async fn database_migrations() {
    println!("  === 数据库迁移 ===");

    println!("  创建迁移:");
    println!("    sqlx migrate add create_users_table");

    println!("  迁移文件内容:");
    println!("    -- Up");
    println!("    CREATE TABLE users (");
    println!("        id SERIAL PRIMARY KEY,");
    println!("        name VARCHAR(100) NOT NULL,");
    println!("        email VARCHAR(255) UNIQUE NOT NULL,");
    println!("        created_at TIMESTAMP DEFAULT NOW()");
    println!("    );");

    println!("    -- Down");
    println!("    DROP TABLE users;");

    println!("  执行迁移:");
    println!("    sqlx migrate run");

    println!("  回滚迁移:");
    println!("    sqlx migrate revert");
}

/// 复杂查询
#[cfg(feature = "db")]
pub async fn complex_queries() {
    println!("  === 复杂查询 ===");

    println!("  1. JOIN查询:");
    println!("    SELECT u.name, o.total");
    println!("    FROM users u");
    println!("    JOIN orders o ON u.id = o.user_id");
    println!("    WHERE o.status = 'completed'");

    println!("  2. 聚合查询:");
    println!("    SELECT user_id, COUNT(*) as order_count, SUM(total) as total_spent");
    println!("    FROM orders");
    println!("    GROUP BY user_id");
    println!("    HAVING COUNT(*) > 5");

    println!("  3. 子查询:");
    println!("    SELECT name FROM users");
    println!("    WHERE id IN (SELECT user_id FROM orders WHERE total > 1000)");

    println!("  4. 分页查询:");
    println!("    SELECT * FROM users");
    println!("    ORDER BY created_at DESC");
    println!("    LIMIT 10 OFFSET 0");
}

/// 数据库索引
#[cfg(feature = "db")]
pub async fn database_indexes() {
    println!("  === 数据库索引 ===");

    println!("  创建索引:");
    println!("    CREATE INDEX idx_users_email ON users(email);");

    println!("  复合索引:");
    println!("    CREATE INDEX idx_orders_user_status ON orders(user_id, status);");

    println!("  唯一索引:");
    println!("    CREATE UNIQUE INDEX idx_users_email_unique ON users(email);");

    println!("  索引使用场景:");
    println!("    - WHERE条件中的列");
    println!("    - JOIN条件中的列");
    println!("    - ORDER BY中的列");
}

/// 数据库性能优化
#[cfg(feature = "db")]
pub async fn performance_optimization() {
    println!("  === 数据库性能优化 ===");

    println!("  1. 查询优化:");
    println!("    - 使用EXPLAIN分析查询");
    println!("    - 避免SELECT *");
    println!("    - 使用LIMIT限制结果集");

    println!("  2. 连接优化:");
    println!("    - 使用连接池");
    println!("    - 及时关闭连接");
    println!("    - 设置合适的连接数");

    println!("  3. 索引优化:");
    println!("    - 为常用查询字段创建索引");
    println!("    - 避免过多索引");
    println!("    - 定期分析索引使用情况");

    println!("  4. 事务优化:");
    println!("    - 保持事务简短");
    println!("    - 避免长事务");
    println!("    - 合理使用隔离级别");
}

/// 数据库备份与恢复
#[cfg(feature = "db")]
pub async fn backup_and_restore() {
    println!("  === 数据库备份与恢复 ===");

    println!("  PostgreSQL备份:");
    println!("    pg_dump -U username -d dbname -f backup.sql");

    println!("  PostgreSQL恢复:");
    println!("    psql -U username -d dbname -f backup.sql");

    println!("  定期备份策略:");
    println!("    - 每日全量备份");
    println!("    - 每小时增量备份");
    println!("    - 备份验证");

    println!("  备份存储:");
    println!("    - 本地存储");
    println!("    - 云存储（S3等）");
    println!("    - 异地备份");
}

/// 运行所有数据库示例
#[cfg(feature = "db")]
pub async fn run_examples() {
    basic_database_connection().await;
    crud_operations().await;
    transaction_example().await;
    connection_pool().await;
    database_error_handling().await;
    database_migrations().await;
    complex_queries().await;
    database_indexes().await;
    performance_optimization().await;
    backup_and_restore().await;
}
