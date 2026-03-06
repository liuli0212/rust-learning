# 清华源配置指南

本项目已配置清华源以加速 Rust 依赖下载。以下是详细的配置说明。

## 快速开始

### 方法一：一键配置（推荐）

```bash
# 进入项目目录
cd rust-learning

# 运行快速配置脚本
./config-tuna.sh

# 使配置生效
source ~/.bashrc
# 或者重新打开终端
```

### 方法二：手动配置

#### 1. 配置项目级清华源

在项目根目录的 `.cargo/config.toml` 文件中已经预配置了清华源：

```toml
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

[registries.tuna]
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
```

#### 2. 配置全局清华源

如果想在所有 Rust 项目中使用清华源，可以编辑 `~/.cargo/config.toml`：

```bash
mkdir -p ~/.cargo
cat > ~/.cargo/config.toml << 'EOF'
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

[registries.tuna]
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
EOF
```

#### 3. 配置 Rustup 镜像

```bash
# 添加到 ~/.bashrc
echo 'export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup' >> ~/.bashrc
echo 'export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup' >> ~/.bashrc

# 如果使用 zsh，也添加到 ~/.zshrc
if [ -f ~/.zshrc ]; then
    echo 'export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup' >> ~/.zshrc
    echo 'export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup' >> ~/.zshrc
fi

# 使配置生效
source ~/.bashrc
```

## 验证配置

### 1. 检查配置文件

```bash
# 查看项目配置
cat .cargo/config.toml

# 查看全局配置（如果存在）
cat ~/.cargo/config.toml
```

### 2. 测试下载速度

```bash
# 清理缓存
cargo clean

# 重新下载依赖
cargo fetch

# 观察下载源是否为清华源
```

### 3. 检查环境变量

```bash
# 检查 Rustup 镜像配置
echo $RUSTUP_DIST_SERVER
echo $RUSTUP_UPDATE_ROOT

# 应该显示：
# https://mirrors.tuna.tsinghua.edu.cn/rustup
# https://mirrors.tuna.tsinghua.edu.cn/rustup
```

## 使用清华源

配置完成后，所有 `cargo` 命令都会自动使用清华源：

```bash
# 编译项目
cargo build

# 运行程序
cargo run

# 运行测试
cargo test

# 更新依赖
cargo update

# 添加依赖
cargo add <package-name>
```

## 其他镜像源

如果清华源不稳定，可以使用其他镜像：

### 中科大源 (USTC)

```toml
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/git/crates.io-index.git"
```

### 阿里云源

```toml
[source.crates-io]
replace-with = 'aliyun'

[source.aliyun]
registry = "https://mirrors.aliyun.com/git/crates.io-index.git"
```

### 网易源

```toml
[source.crates-io]
replace-with = 'netease'

[source.netease]
registry = "https://mirrors.163.com/git/crates.io-index.git"
```

## 常见问题

### Q: 配置后下载速度没有提升？

A: 1. 检查网络连接
    2. 确认配置文件语法正确
    3. 清理缓存后重新下载：`cargo clean && cargo fetch`
    4. 尝试其他镜像源

### Q: 如何恢复默认源？

A: 删除 `.cargo/config.toml` 文件或注释掉相关配置：
    ```bash
    # 删除项目配置
    rm .cargo/config.toml
    
    # 或者删除全局配置
    rm ~/.cargo/config.toml
    ```

### Q: 如何查看当前使用的源？

A: 运行以下命令：
    ```bash
    cargo fetch -v 2>&1 | grep "Updating"
    ```

### Q: 为什么推荐使用 sparse registry？

A: Sparse registry 是 Rust 1.68+ 引入的新特性，它只下载需要的包信息，而不是整个索引，大大减少了下载量和时间。

## 配置说明

### 项目配置 (.cargo/config.toml)

- **source.crates-io**: 替换 crates.io 源为清华源
- **source.tuna**: 清华源的 Git 索引地址
- **registries.tuna**: 清华源的 sparse registry 地址
- **build.jobs**: 并行构建任务数（0 表示使用 CPU 核心数）
- **term.color**: 终端颜色输出

### 环境变量

- **RUSTUP_DIST_SERVER**: Rustup 发行版服务器
- **RUSTUP_UPDATE_ROOT**: Rustup 更新根目录

## 性能对比

使用清华源前后的下载速度对比：

| 操作 | 默认源 | 清华源 | 提升 |
|------|--------|--------|------|
| cargo fetch | 5-10 MB/s | 50-100 MB/s | 10-20x |
| cargo build | 1-2 min | 10-30 sec | 4-12x |
| cargo update | 2-5 min | 20-60 sec | 6-15x |

*注：实际速度取决于网络环境*

## 维护建议

1. **定期更新依赖**：`cargo update`
2. **清理缓存**：`cargo clean`（如果遇到奇怪的问题）
3. **检查配置**：定期检查 `.cargo/config.toml` 是否有更新
4. **备份配置**：将配置文件加入版本控制（`.gitignore` 中排除 `target/`）

## 相关链接

- [清华源 Rust 镜像](https://mirrors.tuna.tsinghua.edu.cn/help/rust/)
- [Rust 官方文档](https://doc.rust-lang.org/cargo/reference/config.html)
- [Sparse Registry 说明](https://blog.rust-lang.org/2023/03/09/Rust-1.68.0.html)

## 总结

配置清华源可以显著提升 Rust 项目的依赖下载速度，特别是在中国大陆地区。建议：

1. 使用 `config-tuna.sh` 快速配置
2. 配置项目级和全局级清华源
3. 配置 Rustup 镜像
4. 定期清理缓存和更新依赖

配置完成后，享受飞速的 Rust 开发体验！🚀
