//! Rust学习库
//!
//! 这个库包含了从基础到高级的Rust编程示例，
//! 特别针对有C++经验的开发者设计。

// 导出过程宏
pub use cli_macro::Parser;

// 导出 ParseError 供 proc macro 使用
pub use cli_parser::ParseError;

pub mod basics;
pub mod cpp_comparison;
pub mod concurrency;
pub mod advanced;
pub mod macro_demo;
pub mod error_handling;
pub mod unsafe_examples;
pub mod cli_parser;

#[cfg(feature = "async")]
pub mod async_programming;

#[cfg(feature = "async")]
pub mod async_call_sync;

#[cfg(feature = "web")]
pub mod web_server;

#[cfg(feature = "db")]
pub mod database;

// 重新导出常用类型
pub use basics::run_examples as basics_run_examples;
pub use cpp_comparison::run_examples as cpp_comparison_run_examples;
pub use concurrency::run_examples as concurrency_run_examples;
pub use advanced::run_examples as advanced_run_examples;
pub use macro_demo::run_examples as macro_demo_run_examples;
pub use error_handling::run_examples as error_handling_run_examples;
pub use unsafe_examples::run_examples as unsafe_examples_run_examples;
pub use cli_parser::run_examples as cli_parser_run_examples;

#[cfg(feature = "async")]
pub use async_programming::run_examples as async_programming_run_examples;

#[cfg(feature = "async")]
pub use async_call_sync::run_examples as async_call_sync_run_examples;

#[cfg(feature = "web")]
pub use web_server::run_examples as web_server_run_examples;

#[cfg(feature = "db")]
pub use database::run_examples as database_run_examples;pub mod memory_layout; pub use memory_layout::run_examples as memory_layout_run_examples;
pub mod advanced_types; pub use advanced_types::run_examples as advanced_types_run_examples;
pub mod smart_pointers; pub use smart_pointers::run_examples as smart_pointers_run_examples;
pub mod ffi_cxx; pub use ffi_cxx::run_examples as ffi_cxx_run_examples; pub mod observability; pub use observability::run_examples as observability_run_examples;
