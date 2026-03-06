#!/bin/bash

# 配置清华源脚本
# 用于加速 Rust 依赖下载

set -e

echo "=== 配置清华源 ==="
echo ""

# 检查是否在项目目录
if [ ! -f "Cargo.toml" ]; then
    echo "错误：请在项目根目录运行此脚本"
    exit 1
fi

# 创建 .cargo 目录
mkdir -p .cargo

# 配置 crates.io 镜像
cat > .cargo/config.toml << 'EOF'
# Cargo 配置文件
# 配置清华源以加速依赖下载

[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# sparse registry 配置（更快）
[registries.tuna]
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"

# 构建配置
[build]
# jobs = 0  # 使用默认值

# 配置警告
[term]
color = "always"

# 配置 sparse registry（推荐用于中国大陆）
[registries.crates-io]
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"

# 配置优化级别
[profile.dev]
opt-level = 0
debug = true
debug-assertions = true
overflow-checks = true
lto = false
panic = "unwind"

[profile.release]
opt-level = 3
debug = false
debug-assertions = false
overflow-checks = false
lto = true
panic = "abort"
codegen-units = 1
EOF

echo "✓ 已创建 .cargo/config.toml"

# 询问是否配置 Rustup 镜像
echo ""
echo "是否配置 Rustup 镜像？(y/n)"
read -p "选择: " choice

if [[ $choice == "y" || $choice == "Y" ]]; then
    # 配置 Rustup 镜像
    echo "export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup" >> ~/.bashrc
    echo "export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup" >> ~/.bashrc
    
    # 同时添加到 ~/.zshrc（如果存在）
    if [ -f ~/.zshrc ]; then
        echo "export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup" >> ~/.zshrc
        echo "export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup" >> ~/.zshrc
    fi
    
    echo "✓ 已添加 Rustup 镜像配置到 ~/.bashrc"
    echo "  请运行 'source ~/.bashrc' 或重新打开终端使配置生效"
fi

# 询问是否清理缓存
echo ""
echo "是否清理 Cargo 缓存并重新下载依赖？(y/n)"
read -p "选择: " choice2

if [[ $choice2 == "y" || $choice2 == "Y" ]]; then
    echo "清理缓存..."
    cargo clean
    
    echo "重新下载依赖..."
    cargo fetch
    
    echo "✓ 缓存已清理，依赖已重新下载"
fi

echo ""
echo "=== 配置完成 ==="
echo ""
echo "配置信息："
echo "  - crates.io 镜像: 清华源 (tuna)"
echo "  - 镜像地址: https://mirrors.tuna.tsinghua.edu.cn"
echo ""
echo "使用方法："
echo "  1. cargo build    # 编译项目"
echo "  2. cargo run      # 运行程序"
echo "  3. cargo test     # 运行测试"
echo ""
echo "如果下载速度仍然较慢，可以尝试："
echo "  1. 检查网络连接"
echo "  2. 使用其他镜像源（中科大、阿里云等）"
echo "  3. 配置代理（如果需要）"
echo ""
