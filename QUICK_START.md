# 快速开始指南

## 1. 配置清华源（推荐）

```bash
# 进入项目目录
cd rust-learning

# 运行快速配置脚本
./config-tuna.sh

# 使配置生效
source ~/.bashrc
# 或者重新打开终端
```

## 2. 验证配置

```bash
# 运行测试脚本
./test-tuna.sh

# 或者手动测试
cargo fetch
cargo build
```

## 3. 运行示例

### 运行所有示例

```bash
cargo run --bin rust-learning
```

### 运行特定示例

```bash
# 基础语法示例
cargo run --bin basic

# C++对比示例
cargo run --bin cpp-comparison

# 并发编程示例
cargo run --bin concurrency

# 高级特性示例
cargo run --bin advanced

# 宏演示示例
cargo run --bin macro-demo

# 错误处理示例
cargo run --bin error-handling
```

## 4. 学习重点

### Box 用法（basics.rs）

```rust
// 基本Box使用
let boxed_number = Box::new(42);

// 递归类型
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 多态（trait对象）
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];
```

### dyn 语法（basics.rs）

```rust
// 动态分发
trait Animal {
    fn speak(&self);
}

// Box<dyn Animal> 允许运行时多态
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];
```

### Unsafe 代码（unsafe_examples.rs）

```rust
// 原始指针
let ptr: *const i32 = &42 as *const i32;

// 内存操作
unsafe {
    let layout = Layout::from_size_align(100, 4).unwrap();
    let ptr = alloc(layout);
    // ...
    dealloc(ptr, layout);
}
```

## 5. 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test unsafe_examples_test

# 运行库测试
cargo test --lib
```

## 6. 查看文档

### 主要文档

- **README.md** - 项目说明和清华源配置
- **TUNA_SETUP.md** - 清华源详细配置指南
- **UNSAFE_GUIDE.md** - Unsafe代码使用指南
- **SETUP_SUMMARY.md** - 配置完成总结
- **INSTALL.md** - 安装和使用说明

### 代码文档

```bash
# 生成并打开文档
cargo doc --open
```

## 7. 项目结构

```
rust-learning/
├── src/
│   ├── basics.rs              # 基础语法（包含Box和dyn示例）
│   ├── unsafe_examples.rs     # Unsafe代码示例
│   ├── concurrency.rs         # 并发编程
│   ├── advanced.rs            # 高级特性
│   ├── macro_demo.rs          # 宏演示
│   ├── error_handling.rs      # 错误处理
│   ├── cpp_comparison.rs      # C++对比
│   ├── async_programming.rs   # 异步编程
│   ├── database.rs            # 数据库操作
│   ├── web_server.rs          # Web服务器
│   └── lib.rs                 # 库入口
├── tests/
│   └── unsafe_examples_test.rs # Unsafe示例测试
├── .cargo/
│   └── config.toml            # 清华源配置
├── config-tuna.sh             # 快速配置脚本
├── setup-tuna.sh              # 交互式配置脚本
├── test-tuna.sh               # 测试脚本
└── 文档文件                   # 详细说明文档
```

## 8. 常用命令速查

```bash
# 构建
cargo build              # 调试构建
cargo build --release    # 发布构建

# 运行
cargo run                # 运行主程序
cargo run --bin <name>   # 运行特定二进制文件

# 测试
cargo test               # 运行所有测试
cargo test --lib         # 运行库测试

# 文档
cargo doc                # 生成文档
cargo doc --open         # 生成并打开文档

# 清理
cargo clean              # 清理构建产物

# 依赖管理
cargo update             # 更新依赖
cargo add <package>      # 添加依赖
cargo remove <package>   # 移除依赖

# 检查
cargo check              # 快速检查代码
cargo clippy             # 代码质量检查（如果安装了clippy）
cargo fmt                # 代码格式化（如果安装了rustfmt）
```

## 9. 学习路径建议

### 第一阶段：基础语法
1. 运行 `cargo run --bin basic`
2. 阅读 `basics.rs` 中的注释
3. 重点关注：
   - Box 用法
   - dyn 语法
   - 所有权和借用
   - 错误处理

### 第二阶段：并发编程
1. 运行 `cargo run --bin concurrency`
2. 阅读 `concurrency.rs`
3. 学习：
   - 线程创建
   - 数据共享
   - 通道通信
   - 锁机制

### 第三阶段：高级特性
1. 运行 `cargo run --bin advanced`
2. 阅读 `advanced.rs`
3. 学习：
   - 高级Trait系统
   - 泛型编程
   - 宏编程
   - 自定义智能指针

### 第四阶段：Unsafe 代码
1. 运行 `cargo run --bin rust-learning`（包含Unsafe示例）
2. 阅读 `unsafe_examples.rs` 和 `UNSAFE_GUIDE.md`
3. 学习：
   - 原始指针
   - 内存管理
   - FFI
   - 安全封装

## 10. 常见问题

### Q: 如何配置清华源？
A: 运行 `./config-tuna.sh` 并 `source ~/.bashrc`

### Q: 如何运行特定示例？
A: 使用 `cargo run --bin <示例名>`

### Q: 如何查看Box和dyn的详细示例？
A: 查看 `src/basics.rs` 中的 `box_examples()` 和 `dyn_examples()` 函数

### Q: 如何查看Unsafe代码示例？
A: 查看 `src/unsafe_examples.rs` 和 `UNSAFE_GUIDE.md`

### Q: 如何运行测试？
A: 使用 `cargo test` 或 `cargo test --lib`

## 11. 下一步

1. ✅ 配置清华源
2. ✅ 运行所有示例
3. ✅ 阅读相关文档
4. 📝 动手修改示例代码
5. 🚀 开始自己的Rust项目

## 12. 资源链接

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rust 语言圣经](https://course.rs/)
- [Rust 程序设计语言](https://doc.rust-lang.org/book/)
- [Rustnomicon](https://doc.rust-lang.org/nomicon/) - Unsafe代码指南

---

**祝你 Rust 学习愉快！** 🦀
