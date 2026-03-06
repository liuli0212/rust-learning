# Rust学习项目

基于C++高阶编程经验的Rust渐进式学习指南。

## 项目结构

```
rust-learning/
├── Cargo.toml          # 项目配置和依赖
├── README.md          # 本文件
└── src/
    ├── main.rs        # 主程序入口
    ├── lib.rs         # 库模块
    ├── basics.rs      # 基础语法
    ├── cpp_comparison.rs  # C++对比
    ├── concurrency.rs # 并发编程
    └── advanced.rs    # 高级特性
```

## 学习路径

### 1. 基础语法 (`basics.rs`)
- **变量和可变性**：不可变变量、可变变量、常量、阴影
- **所有权系统**：所有权转移、克隆、函数调用
- **借用机制**：不可变借用、可变借用、借用规则
- **生命周期**：基础生命周期、函数生命周期
- **结构体和枚举**：定义、模式匹配
- **错误处理**：Result、Option类型
- **泛型基础**：泛型函数、泛型结构体
- **Trait基础**：定义trait、实现trait、trait作为参数

### 2. C++对比 (`cpp_comparison.rs`)
- **智能指针对比**：Box vs unique_ptr, Rc vs shared_ptr, Arc vs atomic_shared_ptr
- **RAII对比**：Drop trait vs 析构函数
- **模式匹配**：match vs switch（更强大）
- **错误处理**：Result vs 异常/错误码
- **内存安全**：编译器保证 vs 手动管理
- **并发安全**：类型系统防止数据竞争
- **函数式编程**：闭包、迭代器 vs lambda
- **类型系统**：Trait对象 vs 虚函数，关联类型
- **编译时计算**：const fn vs constexpr，宏 vs 模板元编程

### 3. 并发编程 (`concurrency.rs`)
- **基础线程**：创建线程、join
- **线程间共享数据**：Arc + Mutex
- **通道通信**：mpsc通道
- **多生产者单消费者**：多个发送端
- **读写锁**：RwLock
- **条件变量**：Condvar
- **原子操作**：Atomic类型
- **异步编程**：async/await、tokio运行时
- **异步通道**：tokio::sync::mpsc
- **异步超时**：timeout机制
- **异步流**：Stream trait
- **工作池模式**：线程池实现

### 4. 高级特性 (`advanced.rs`)
- **高级Trait系统**：关联类型、默认泛型参数、扩展trait、动态分发
- **高级泛型**：const泛型、高阶生命周期、类型约束组合
- **操作符重载**：Add trait、自定义运算
- **智能指针自定义**：MyBox、MyRc实现
- **宏编程**：声明宏、模式匹配、可变参数、类型检查
- **过程宏概念**：派生宏、代码生成
- **闭包与捕获**：捕获模式、移动语义、闭包作为参数
- **迭代器高级用法**：自定义迭代器、惰性求值
- **类型状态模式**：用类型系统编码状态
- **编译时计算**：const函数、const泛型、编译时字符串处理

## 快速开始

### 安装Rust

```bash
# 安装rustup（Rust工具链管理器）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 或者通过包管理器（Ubuntu/Debian）
sudo apt install rustc cargo

# 或者通过包管理器（macOS）
brew install rust
```

### 配置清华源（推荐）

为了加速依赖下载，建议配置清华源。本项目已预配置了清华源，你也可以手动配置。

#### 方法一：使用快速配置脚本（推荐）

```bash
# 进入项目目录
cd rust-learning

# 运行快速配置脚本
./config-tuna.sh

# 使配置生效
source ~/.bashrc
# 或者重新打开终端
```

脚本会自动：
- 创建 `.cargo/config.toml` 配置清华源
- 配置 Rustup 镜像到 `~/.bashrc`
- 配置 Rustup 镜像到 `~/.zshrc`（如果存在）

#### 方法二：使用交互式配置脚本

```bash
# 进入项目目录
cd rust-learning

# 运行交互式配置脚本
./setup-tuna.sh

# 按照提示选择是否配置 Rustup 镜像和清理缓存
```

脚本会自动：
- 创建 `.cargo/config.toml` 配置清华源
- 询问是否配置 Rustup 镜像
- 询问是否清理缓存并重新下载依赖

#### 方法二：手动配置

##### 1. 配置 crates.io 镜像

在项目根目录创建或编辑 `.cargo/config.toml` 文件：

```toml
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# sparse registry 配置（更快）
[registries.tuna]
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
```

或者直接在 `~/.cargo/config.toml`（全局配置）中添加：

```toml
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

[registries.tuna]
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
```

##### 2. 配置 Rustup 镜像（可选）

如果 rustup 下载速度慢，可以设置环境变量：

```bash
# 临时设置
export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup

# 永久设置（添加到 ~/.bashrc 或 ~/.zshrc）
echo 'export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup' >> ~/.bashrc
echo 'export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup' >> ~/.bashrc
source ~/.bashrc
```

##### 3. 验证配置

```bash
# 查看当前配置
cargo --version

# 测试下载速度
cargo search tokio
```

##### 4. 清理并重新下载

如果之前已经下载过依赖，可以清理缓存重新下载：

```bash
# 清理缓存
cargo clean

# 重新编译（会从清华源下载依赖）
cargo build
```

#### 方法三：使用环境变量（临时）

如果你不想修改配置文件，可以使用环境变量：

```bash
# 临时设置（仅当前终端有效）
export CARGO_REGISTRIES_TUNA_INDEX="sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"

# 然后运行 cargo 命令
cargo build
```

#### 其他镜像源

如果清华源不稳定，也可以使用其他镜像：

**中科大源：**
```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/git/crates.io-index.git"
```

**阿里云源：**
```toml
[source.crates-io]
replace-with = 'aliyun'

[source.aliyun]
registry = "https://mirrors.aliyun.com/git/crates.io-index.git"
```

**网易源：**
```toml
[source.crates-io]
replace-with = 'netease'

[source.netease]
registry = "https://mirrors.163.com/git/crates.io-index.git"
```

**注意：** 建议优先使用清华源，因为它对 Rust 社区支持较好，更新及时。

详细配置说明请参考 [TUNA_SETUP.md](TUNA_SETUP.md)。

配置完成后，请查看 [SETUP_SUMMARY.md](SETUP_SUMMARY.md) 了解配置详情。

### 编译和运行

```bash
# 进入项目目录
cd rust-learning

# 编译项目
cargo build

# 运行程序
cargo run

# 发布版本（优化）
cargo run --release
```

### 运行特定模块

你可以修改 `src/main.rs` 中的 `main` 函数来只运行特定的模块：

```rust
fn main() {
    println!("=== 只运行基础语法 ===");
    basics::run_examples();
    
    // 注释掉其他模块的调用
    // cpp_comparison::run_examples();
    // concurrency::run_examples();
    // advanced::run_examples();
}
```

## 针对C++开发者的特别说明

### Rust vs C++ 关键差异

| 特性 | C++ | Rust |
|------|-----|------|
| 内存管理 | 手动/智能指针 | 所有权系统（编译时） |
| 并发安全 | 需要手动加锁 | 类型系统保证无数据竞争 |
| 错误处理 | 异常/错误码 | Result/Option类型 |
| 模板 | 编译时代码生成 | 泛型 + Trait约束 |
| 元编程 | 模板元编程 | 宏 + const泛型 |
| 生命周期 | 手动管理 | 编译器自动检查 |

### 核心概念映射

- **RAII** → Drop trait + 所有权
- **智能指针** → Box, Rc, Arc, RefCell, Mutex
- **模板** → 泛型 + Trait
- **虚函数** → Trait对象 (`dyn Trait`)
- **异常** → Result<T, E>
- **constexpr** → const fn
- **lambda** → 闭包
- **std::optional** → Option<T>
- **std::variant** → enum

## 学习建议

### 第一阶段：基础语法（1-2天）
1. 从 `basics.rs` 开始，理解所有权和借用
2. 重点掌握：所有权转移、借用规则、生命周期
3. 这是Rust最独特的部分，需要时间适应

### 第二阶段：C++对比（1天）
1. 阅读 `cpp_comparison.rs`
2. 理解Rust如何解决C++中的常见问题
3. 对比两种语言的设计哲学

### 第三阶段：并发编程（2-3天）
1. 学习线程、通道、锁
2. 理解Rust的并发安全保证
3. 尝试异步编程（需要更多时间）

### 第四阶段：高级特性（3-5天）
1. 深入Trait系统和泛型
2. 学习宏编程
3. 探索高级模式

## 常见问题

### Q: 为什么Rust没有垃圾回收？
A: Rust使用所有权系统在编译时管理内存，无需运行时GC，性能更接近C++。

### Q: Rust的学习曲线如何？
A: 对于C++开发者，所有权系统是主要挑战，但一旦掌握，其他部分相对容易。

### Q: Rust适合什么场景？
A: 系统编程、WebAssembly、CLI工具、嵌入式开发、高性能服务等。

### Q: 如何处理循环引用？
A: 使用Weak指针（`std::rc::Weak`）来打破循环引用。

## 扩展资源

- [官方Rust文档](https://doc.rust-lang.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust for C++ Developers](https://github.com/aminroosta/rust-for-cpp-developers)

## 贡献

欢迎提交PR或建议，让这个学习项目更加完善！

## 许可证

MIT License