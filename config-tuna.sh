#!/bin/bash

# 快速配置清华源脚本

set -e

echo "=== 配置清华源 ==="

# 创建 .cargo 目录
mkdir -p .cargo

# 配置 crates.io 镜像
cat > .cargo/config.toml << 'EOF'
# Cargo 配置文件 - 清华源
# 用于加速 Rust 依赖下载

[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

[registries.tuna]
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"

[build]
# jobs = 0  # 使用默认值

[term]
color = "always"
EOF

echo "✓ 已配置清华源到 .cargo/config.toml"

# 配置 Rustup 镜像
echo "export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup" >> ~/.bashrc
echo "export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup" >> ~/.bashrc

if [ -f ~/.zshrc ]; then
    echo "export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup" >> ~/.zshrc
    echo "export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup" >> ~/.zshrc
fi

echo "✓ 已配置 Rustup 镜像到 ~/.bashrc"

echo ""
echo "=== 配置完成 ==="
echo ""
echo "请运行以下命令使配置生效："
echo "  source ~/.bashrc"
echo ""
echo "或者重新打开终端。"
echo ""
echo "现在可以使用以下命令："
echo "  cargo build    # 编译项目"
echo "  cargo run      # 运行程序"
echo "  cargo test     # 运行测试"
echo ""
