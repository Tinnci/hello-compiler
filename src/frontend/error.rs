// 错误处理模块
//
// 这个模块定义了前端解析器的错误类型

use std::error::Error;
use std::fmt;
use std::io;

/// 解析位置，用于错误报告
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub filename: String,
    pub line: usize,
    pub column: usize,
}

impl SourceLocation {
    pub fn new(filename: &str, line: usize, column: usize) -> Self {
        SourceLocation {
            filename: filename.to_string(),
            line,
            column,
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.column)
    }
}

/// 解析错误类型
#[derive(Debug)]
pub enum ParseErrorKind {
    /// 词法错误
    Lexical(String),
    /// 语法错误
    Syntax(String),
    /// 语义错误
    Semantic(String),
    /// IO错误
    IO(io::Error),
}

/// 解析错误
#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    location: Option<SourceLocation>,
}

impl ParseError {
    /// 创建一个新的词法错误
    pub fn new_lexical_error(location: SourceLocation, message: &str) -> Self {
        ParseError {
            kind: ParseErrorKind::Lexical(message.to_string()),
            location: Some(location),
        }
    }

    /// 创建一个新的语法错误
    pub fn new_syntax_error(location: SourceLocation, message: &str) -> Self {
        ParseError {
            kind: ParseErrorKind::Syntax(message.to_string()),
            location: Some(location),
        }
    }

    /// 创建一个新的语义错误
    pub fn new_semantic_error(location: SourceLocation, message: &str) -> Self {
        ParseError {
            kind: ParseErrorKind::Semantic(message.to_string()),
            location: Some(location),
        }
    }

    /// 创建一个新的IO错误
    pub fn new_io_error(filename: &str, error: io::Error) -> Self {
        ParseError {
            kind: ParseErrorKind::IO(error),
            location: Some(SourceLocation::new(filename, 0, 0)),
        }
    }

    /// 获取错误位置
    pub fn location(&self) -> Option<&SourceLocation> {
        self.location.as_ref()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ParseErrorKind::Lexical(msg) => {
                if let Some(loc) = &self.location {
                    write!(f, "词法错误 at {}: {}", loc, msg)
                } else {
                    write!(f, "词法错误: {}", msg)
                }
            }
            ParseErrorKind::Syntax(msg) => {
                if let Some(loc) = &self.location {
                    write!(f, "语法错误 at {}: {}", loc, msg)
                } else {
                    write!(f, "语法错误: {}", msg)
                }
            }
            ParseErrorKind::Semantic(msg) => {
                if let Some(loc) = &self.location {
                    write!(f, "语义错误 at {}: {}", loc, msg)
                } else {
                    write!(f, "语义错误: {}", msg)
                }
            }
            ParseErrorKind::IO(e) => {
                if let Some(loc) = &self.location {
                    write!(f, "IO错误 at {}: {}", loc, e)
                } else {
                    write!(f, "IO错误: {}", e)
                }
            }
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            ParseErrorKind::IO(e) => Some(e),
            _ => None,
        }
    }
}

/// 解析结果类型
pub type ParseResult<T> = Result<T, ParseError>;
