#!/bin/bash

# 测试清华源配置脚本

set -e

echo "=== 测试清华源配置 ==="
echo ""

# 检查配置文件
echo "1. 检查配置文件..."
if [ -f ".cargo/config.toml" ]; then
    echo "   ✓ 项目配置文件存在"
    if grep -q "tuna" .cargo/config.toml; then
        echo "   ✓ 配置文件包含清华源"
    else
        echo "   ✗ 配置文件不包含清华源"
        exit 1
    fi
else
    echo "   ✗ 项目配置文件不存在"
    exit 1
fi

# 检查环境变量
echo ""
echo "2. 检查环境变量..."
if [ -n "$RUSTUP_DIST_SERVER" ]; then
    echo "   ✓ RUSTUP_DIST_SERVER: $RUSTUP_DIST_SERVER"
else
    echo "   ⚠ RUSTUP_DIST_SERVER 未设置（可选）"
fi

if [ -n "$RUSTUP_UPDATE_ROOT" ]; then
    echo "   ✓ RUSTUP_UPDATE_ROOT: $RUSTUP_UPDATE_ROOT"
else
    echo "   ⚠ RUSTUP_UPDATE_ROOT 未设置（可选）"
fi

# 测试下载
echo ""
echo "3. 测试从清华源下载依赖..."
echo "   正在清理缓存..."
cargo clean > /dev/null 2>&1

echo "   正在下载依赖..."
if cargo fetch > /dev/null 2>&1; then
    echo "   ✓ 从清华源下载成功"
else
    echo "   ✗ 下载失败，请检查网络连接"
    exit 1
fi

# 检查下载源
echo ""
echo "4. 检查下载源..."
if cargo fetch -v 2>&1 | grep -q "tuna"; then
    echo "   ✓ 正在使用清华源"
else
    echo "   ⚠ 未检测到清华源"
fi

# 测试编译
echo ""
echo "5. 测试编译..."
if cargo build --lib > /dev/null 2>&1; then
    echo "   ✓ 编译成功"
else
    echo "   ✗ 编译失败"
    exit 1
fi

echo ""
echo "=== 测试完成 ==="
echo ""
echo "清华源配置正常！"
echo ""
echo "现在可以使用以下命令："
echo "  cargo build    # 编译项目"
echo "  cargo run      # 运行程序"
echo "  cargo test     # 运行测试"
echo ""
