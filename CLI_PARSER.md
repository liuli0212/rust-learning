# 简化版 clap 风格命令行参数解析器 - 过程宏示例

这个示例展示了如何使用Rust的过程宏（procedural macro）来创建一个类似 clap 风格的命令行参数解析器。

## 项目结构

```
rust-learning/
├── cli_macro/                    # 过程宏crate
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs               # 过程宏实现
├── src/
│   ├── cli_parser.rs           # 基础解析器和特性定义
│   ├── bin/
│   │   └── cli/
│   │       └── main.rs         # CLI示例程序
│   └── lib.rs                  # 导出过程宏
└── Cargo.toml
```

## 使用方法

### 1. 定义命令行参数结构体

```rust
use cli_macro::Parser;

#[derive(Debug, Parser)]
#[command(name = "myapp", about = "My application")]
struct Args {
    /// 输入文件路径
    #[arg(short, long)]
    input: String,

    /// 输出文件路径
    #[arg(short, long, default_value = "output.txt")]
    output: String,

    /// 显示详细信息
    #[arg(short, long)]
    verbose: bool,
}
```

### 2. 解析参数

```rust
fn main() {
    match Args::parse() {
        Ok(args) => {
            println!("输入文件: {}", args.input);
            println!("输出文件: {}", args.output);
            println!("详细模式: {}", args.verbose);
        }
        Err(e) => {
            println!("错误: {}", e);
            println!("{}", Args::help());
        }
    }
}
```

### 3. 运行示例

```bash
# 显示帮助
cargo run --bin cli -- help

# 文件处理工具
cargo run --bin cli -- file --input data.txt --output result.txt --verbose --count 3 --mode copy

# 网络工具
cargo run --bin cli -- net --host example.com --port 443 --secure --method POST --timeout 60
```

## 过程宏特性

### 支持的属性

- `#[command(name = "app", about = "description")]`: 配置命令信息
- `#[arg(short)]`: 短参数（使用字段名第一个字符）
- `#[arg(short = "c")]`: 自定义短参数
- `#[arg(long)]`: 长参数（使用字段名，下划线自动转换为连字符）
- `#[arg(long = "config")]`: 自定义长参数
- `#[arg(default_value = "值")]`: 默认值
- `#[arg(conflicts_with = "other")]`: 参数冲突检测
- `#[arg(flag)]`: 布尔标志（不需要值，bool类型自动识别）

### 自动生成的功能

1. **参数解析**: 从命令行参数自动解析到结构体字段
2. **类型转换**: 自动将字符串转换为对应类型（String, u32, bool, Option<T>等）
3. **帮助信息**: 自动生成格式化的帮助信息（支持文档注释）
4. **错误处理**: 提供清晰的错误信息
5. **冲突检测**: 检测参数冲突并报告错误

## 实现原理

### 1. 过程宏 (`cli_macro`)

- `#[derive(CliParser)]`: 为结构体生成解析代码
- `CliParser` trait: 定义解析接口
- 属性解析: 解析 `#[arg(...)]` 属性中的配置

### 2. 基础解析器 (`cli_parser`)

- `ArgConfig`: 参数配置结构
- `parse_args()`: 核心解析逻辑
- `CliParser` trait: 为派生宏提供接口

### 3. 生成的代码

过程宏会为每个字段生成：
- 参数配置代码
- 字段赋值代码
- 帮助信息条目

## 示例输出

```
=== 命令行参数解析器示例 ===

文件处理工具模式

解析到的参数:
  输入文件: data.txt
  输出文件: result.txt
  详细模式: true
  处理次数: 3
  处理模式: copy

开始处理文件...
  详细模式已启用
  从 data.txt 读取文件
  处理 3 次
  使用 copy 模式
  写入到 result.txt
文件处理完成！
```

## 学习要点

1. **过程宏基础**: 如何创建 `proc-macro` crate
2. **属性解析**: 使用 `syn` 库解析自定义属性
3. **代码生成**: 使用 `quote` 库生成Rust代码
4. **Trait设计**: 如何设计可扩展的接口
5. **错误处理**: 如何提供清晰的错误信息

## 扩展建议

- 支持更多类型（Vec, Option等）
- 添加参数验证
- 支持子命令
- 生成bash/zsh自动补全
- 支持配置文件