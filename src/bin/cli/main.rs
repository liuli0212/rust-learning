//! 简化版 clap 风格命令行参数解析器示例
//!
//! 这个示例展示了如何使用类似clap风格的过程宏来创建命令行参数解析器。

use rust_learning::cli_parser::Parser;
use cli_macro::Parser;

/// Rusty-Claw CLI agent 配置
#[derive(Debug, Parser)]
#[command(name = "rusty-claw", about = "Rusty-Claw CLI agent")]
struct CliArgs {
    /// LLM Provider (gemini, aliyun)
    #[arg(long, default_value = "gemini")]
    provider: String,

    /// Model name (e.g. gemini-2.0-flash, qwen-max)
    #[arg(long)]
    model: Option<String>,

    /// Log level (e.g. trace, debug, info, warn, error)
    #[arg(long)]
    log_level: Option<String>,

    /// Log directory for file logging
    #[arg(long)]
    log_dir: Option<String>,

    /// Log file name for file logging (daily rotation)
    #[arg(long)]
    log_file: Option<String>,

    /// Disable file logging (stdout only)
    #[arg(long)]
    no_file_log: bool,

    /// Force enable performance report output
    #[arg(long)]
    timing_report: bool,

    /// Disable performance report output
    #[arg(long, conflicts_with = "timing_report")]
    no_timing_report: bool,

    /// Enable prompt caching (if supported by the provider)
    #[arg(long)]
    cache: bool,
}

/// 文件处理工具配置
#[derive(Debug, Parser)]
#[command(name = "file-tool", about = "文件处理工具")]
struct FileArgs {
    /// Input file path
    #[arg(short, long)]
    input: String,

    /// Output file path
    #[arg(short, long, default_value = "output.txt")]
    output: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Number of processing iterations
    #[arg(short, long, default_value = "1")]
    count: u32,

    /// Processing mode (copy/move)
    #[arg(long, default_value = "copy")]
    mode: String,
}

fn main() {
    println!("=== 简化版 clap 风格命令行参数解析器示例 ===\n");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "rusty-claw" => {
            println!("Rusty-Claw CLI 配置模式\n");
            match CliArgs::parse_from(&args[2..]) {
                Ok(cli_args) => {
                    println!("解析到的参数:");
                    println!("  Provider: {}", cli_args.provider);
                    println!("  Model: {:?}", cli_args.model);
                    println!("  Log Level: {:?}", cli_args.log_level);
                    println!("  Log Dir: {:?}", cli_args.log_dir);
                    println!("  Log File: {:?}", cli_args.log_file);
                    println!("  No File Log: {}", cli_args.no_file_log);
                    println!("  Timing Report: {}", cli_args.timing_report);
                    println!("  No Timing Report: {}", cli_args.no_timing_report);
                    println!("  Cache: {}", cli_args.cache);

                    println!("\n应用配置...");
                    println!("  使用 {} 提供者", cli_args.provider);
                    if let Some(model) = cli_args.model {
                        println!("  模型: {}", model);
                    }
                    if cli_args.timing_report {
                        println!("  性能报告已启用");
                    }
                    if cli_args.no_timing_report {
                        println!("  性能报告已禁用");
                    }
                    if cli_args.cache {
                        println!("  提示缓存已启用");
                    }
                    println!("配置完成！");
                }
                Err(e) => {
                    println!("错误: {}", e);
                    println!("\n{}", CliArgs::help());
                }
            }
        }
        "file-tool" => {
            println!("文件处理工具模式\n");
            match FileArgs::parse_from(&args[2..]) {
                Ok(file_args) => {
                    println!("解析到的参数:");
                    println!("  输入文件: {}", file_args.input);
                    println!("  输出文件: {}", file_args.output);
                    println!("  详细模式: {}", file_args.verbose);
                    println!("  处理次数: {}", file_args.count);
                    println!("  处理模式: {}", file_args.mode);

                    println!("\n开始处理文件...");
                    if file_args.verbose {
                        println!("  详细模式已启用");
                    }
                    println!("  从 {} 读取文件", file_args.input);
                    println!("  处理 {} 次", file_args.count);
                    println!("  使用 {} 模式", file_args.mode);
                    println!("  写入到 {}", file_args.output);
                    println!("文件处理完成！");
                }
                Err(e) => {
                    println!("错误: {}", e);
                    println!("\n{}", FileArgs::help());
                }
            }
        }
        "help" => {
            print_help();
        }
        _ => {
            println!("未知命令: {}", command);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("用法: cargo run --bin cli -- <命令> [参数]");
    println!("\n可用命令:");
    println!("  rusty-claw    Rusty-Claw CLI agent 配置");
    println!("  file-tool     文件处理工具");
    println!("  help          显示帮助信息");
    println!("\n示例:");
    println!("  cargo run --bin cli -- rusty-claw --provider aliyun --model qwen-max --timing-report --cache");
    println!("  cargo run --bin cli -- file-tool --input data.txt --output result.txt --verbose --count 3");
    println!("  cargo run --bin cli -- help");
}

fn print_help() {
    println!("=== 简化版 clap 风格命令行参数解析器帮助 ===\n");

    println!("Rusty-Claw CLI 配置帮助:");
    println!("{}", CliArgs::help());

    println!("\n{}", "=".repeat(60));

    println!("文件处理工具帮助:");
    println!("{}", FileArgs::help());

    println!("\n{}", "=".repeat(60));

    println!("过程宏特性:");
    println!("  - 使用 #[derive(Parser)] 派生解析器");
    println!("  - 使用 #[command(...)] 配置命令信息");
    println!("  - 使用 #[arg(...)] 配置参数");
    println!("  - 支持文档注释作为帮助文本");
    println!("  - 支持短参数 (-p) 和长参数 (--provider)");
    println!("  - 支持默认值 (default_value)");
    println!("  - 支持参数冲突检测 (conflicts_with)");
    println!("  - 支持 Option<T> 类型");
    println!("  - 自动生成帮助信息");
}