//! 命令行参数解析器 - 简化版 clap 风格
//!
//! 这个模块提供了错误类型和 Parser trait，
//! 实际解析逻辑由 cli_macro 过程宏生成。

/// 命令行参数解析错误
#[derive(Debug)]
pub enum ParseError {
    MissingArgument(String),
    InvalidFormat(String, String),
    UnknownArgument(String),
    ConflictError(String, String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::MissingArgument(name) => write!(f, "缺少必需参数: {}", name),
            ParseError::InvalidFormat(name, value) => {
                write!(f, "参数 {} 格式错误: {}", name, value)
            }
            ParseError::UnknownArgument(arg) => write!(f, "未知参数: {}", arg),
            ParseError::ConflictError(arg1, arg2) => {
                write!(f, "参数冲突: {} 和 {} 不能同时使用", arg1, arg2)
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// 命令行参数解析器特性
pub trait Parser: Sized {
    /// 从标准命令行参数解析 (跳过第一个参数，即可执行程序名)
    fn parse() -> Result<Self, ParseError> {
        Self::parse_from(std::env::args().skip(1))
    }

    /// 从任意字符串迭代器解析
    fn parse_from<I, T>(iter: I) -> Result<Self, ParseError>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>;

    /// 获取帮助信息
    fn help() -> String;
}

/// 运行示例
pub fn run_examples() {
    println!("=== 简化版 clap 风格命令行参数解析器 ===\n");

    println!("这个示例展示了类似clap风格的命令行参数解析器。\n");

    println!("主要特性:");
    println!("  - 使用 #[derive(Parser)] 派生解析器");
    println!("  - 使用 #[command(...)] 配置命令信息");
    println!("  - 使用 #[arg(...)] 配置参数");
    println!("  - 支持文档注释作为帮助文本");
    println!("  - 支持短参数 (-p) 和长参数 (--provider)");
    println!("  - 支持默认值 (default_value)");
    println!("  - 支持参数冲突检测 (conflicts_with)");
    println!("  - 支持 Option<T> 类型");
    println!("  - 自动生成帮助信息\n");

    println!("使用方法:");
    println!("  cargo run --bin cli -- rusty-claw --provider aliyun --model qwen-max --verbose");
    println!("  cargo run --bin cli -- --help\n");

    println!("示例代码:");
    println!("  #[derive(Parser)]");
    println!("  #[command(name = \"rusty-claw\", about = \"Rusty-Claw CLI agent\")]");
    println!("  struct CliArgs {{");
    println!("      /// LLM Provider (gemini, aliyun)");
    println!("      #[arg(long, default_value = \"gemini\")]");
    println!("      provider: String,");
    println!("      ");
    println!("      /// Model name (e.g. gemini-2.0-flash, qwen-max)");
    println!("      #[arg(long)]");
    println!("      model: Option<String>,");
    println!("      ");
    println!("      /// Enable verbose output");
    println!("      #[arg(short, long)]");
    println!("      verbose: bool,");
    println!("  }}");
}
