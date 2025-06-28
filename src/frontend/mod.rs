// 前端模块入口
//
// 这个模块包含 VIL 的前端解析器，将源代码转换为 IR

// 子模块
pub mod lexer;
pub mod token;
pub mod parser;
pub mod ast;
pub mod error;

// 重新导出常用类型
pub use lexer::Lexer;
pub use token::{Token, TokenKind};
pub use parser::Parser;
pub use error::{ParseError, ParseResult};

/// 解析 VIL 源代码文本，生成 IR 模块
/// 
/// # Arguments
/// 
/// * `source` - VIL 源代码文本
/// * `filename` - 源文件名，用于错误报告
/// 
/// # Returns
/// 
/// 解析结果，成功则返回 IR 模块，失败则返回解析错误
pub fn parse_vil(source: &str, filename: &str) -> ParseResult<crate::ir::ModuleRef> {
    let lexer = Lexer::new(source, filename);
    let mut parser = Parser::new(lexer);
    parser.parse_module()
}

/// 解析 VIL 源代码文件，生成 IR 模块
/// 
/// # Arguments
/// 
/// * `filepath` - VIL 源代码文件路径
/// 
/// # Returns
/// 
/// 解析结果，成功则返回 IR 模块，失败则返回解析错误
pub fn parse_vil_file(filepath: &str) -> ParseResult<crate::ir::ModuleRef> {
    use std::fs::read_to_string;
    
    match read_to_string(filepath) {
        Ok(source) => parse_vil(&source, filepath),
        Err(e) => Err(ParseError::new_io_error(filepath, e)),
    }
} 