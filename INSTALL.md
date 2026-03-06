# Rust安装和配置指南

## 已完成的步骤

✅ Rust工具链已安装（版本 1.93.1）
✅ Cargo配置已使用清华大学镜像源
✅ 项目代码已创建

## 需要完成的步骤

### 1. 安装C编译器

由于系统缺少C编译器，需要安装build-essential：

```bash
sudo apt update
sudo apt install -y build-essential
```

### 2. 配置环境变量

每次打开新终端时，需要加载Rust环境：

```bash
source $HOME/.cargo/env
```

或者将其添加到你的shell配置文件中（如 `~/.bashrc` 或 `~/.zshrc`）：

```bash
echo 'source $HOME/.cargo/env' >> ~/.bashrc
```

### 3. 验证安装

```bash
# 检查Rust版本
rustc --version

# 检查Cargo版本
cargo --version

# 进入项目目录
cd /home/liuli/src/learn-rust/rust-learning

# 编译项目
cargo check

# 运行项目
cargo run
```

## 项目结构说明

```
rust-learning/
├── Cargo.toml          # 项目配置和依赖
├── INSTALL.md          # 本文件
├── README.md          # 学习指南
├── .gitignore
└── src/
    ├── main.rs        # 主程序入口
    ├── lib.rs         # 库模块
    ├── basics.rs      # 基础语法（变量、所有权、借用等）
    ├── cpp_comparison.rs  # C++对比（智能指针、RAII等）
    ├── concurrency.rs # 并发编程（线程、通道、锁等）
    └── advanced.rs    # 高级特性（Trait、泛型、宏等）
```

## 学习路径

### 第1步：基础语法
运行程序后，首先关注 `basics.rs` 中的内容：
- 变量和可变性
- 所有权系统（Rust最独特的特性）
- 借用和生命周期
- 结构体、枚举、错误处理

### 第2步：C++对比
对于有C++经验的开发者，`cpp_comparison.rs` 特别有用：
- 智能指针对比
- RAII机制对比
- 内存安全保证
- 并发安全对比

### 第3步：并发编程
`concurrency.rs` 展示了：
- 线程创建和管理
- 线程间通信（通道）
- 锁和原子操作
- 工作池模式

### 第4步：高级特性
`advanced.rs` 包含：
- 高级Trait系统
- 泛型编程
- 宏编程
- 自定义智能指针

## 常见问题

### Q: 为什么需要C编译器？
A: Rust编译器会生成中间代码，最终需要链接器（通常是cc）来生成可执行文件。

### Q: 如何启用异步编程示例？
A: 编辑 `Cargo.toml`，取消注释以下依赖：
```toml
tokio = { version = "1.35", features = ["full"] }
futures = "0.3"
```
然后在 `src/main.rs` 中启用异步示例的调用。

### Q: 编译警告是什么？
A: 警告是正常的，表示有些代码未使用或有改进空间。这不会影响程序运行。

## 下一步

1. 安装build-essential
2. 运行 `cargo run` 查看所有示例输出
3. 逐个阅读源代码，理解每个概念
4. 尝试修改代码，添加自己的示例
5. 遇到问题随时提问！

## 参考资源

- [Rust官方文档](https://doc.rust-lang.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)