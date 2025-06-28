// Parser 模块
//
// 这个模块实现了 VIL 的语法分析器，将词法单元序列转换为抽象语法树 (AST)

use crate::frontend::lexer::Lexer;
use crate::frontend::error::{ParseResult, ParseError, SourceLocation};
use crate::frontend::token::{Token, TokenKind};
use crate::ir::{Module, ModuleRef};
use std::rc::Rc;
use std::cell::RefCell;

/// 语法分析器
pub struct Parser<'a> {
    #[allow(dead_code)] // 允许未使用的字段，因为解析器仍在开发中
    lexer: Lexer<'a>,
    #[allow(dead_code)] // 允许未使用的字段，因为解析器仍在开发中
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    /// 创建一个新的语法分析器
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer,
            current_token: None, // 初始为空，会在 advance() 中填充
        }
    }

    /// 解析模块
    pub fn parse_module(&mut self) -> ParseResult<ModuleRef> {
        // 解析入口: `.module <identifier>`

        // 首次读取 token
        self.advance()?;

        // 期望看到 '.'
        if self.peek_token_kind() != Some(&TokenKind::Dot) {
            return Err(ParseError::new_syntax_error(
                self.current_location(),
                "期望 '.' 开始模块声明",
            ));
        }

        // 消费 '.'
        self.advance()?;

        // 期望关键字 "module" (目前实现中作为 Identifier)
        if let Some(TokenKind::Identifier(ident)) = self.peek_token_kind() {
            if ident != "module" {
                return Err(ParseError::new_syntax_error(
                    self.current_location(),
                    "期望关键字 'module'",
                ));
            }
        } else {
            return Err(ParseError::new_syntax_error(
                self.current_location(),
                "期望关键字 'module'",
            ));
        }

        // 消费 "module"
        self.advance()?;

        // 期望模块名称标识符
        let module_name = if let Some(TokenKind::Identifier(name)) = self.peek_token_kind() {
            name.clone()
        } else {
            return Err(ParseError::new_syntax_error(
                self.current_location(),
                "缺少模块名称",
            ));
        };

        // 消费模块名称
        self.advance()?;

        // 创建 Module 实例
        let module_ref: ModuleRef = Rc::new(RefCell::new(Module::new(module_name)));

        // 当前版本: 跳过剩余 token 直到 EOF（后续将解析 .memory / .function 等声明）
        loop {
            self.advance()?;
            if self.peek_token_kind() == Some(&TokenKind::EOF) {
                break;
            }
            // 暂时忽略其他 token
        }

        Ok(module_ref)
    }

    // 占位符方法，用于后续开发
    #[allow(dead_code)] // 允许未使用的代码，因为解析器仍在开发中
    fn advance(&mut self) -> ParseResult<()> {
        self.current_token = Some(self.lexer.next_token()?);
        Ok(())
    }

    #[allow(dead_code)] // 允许未使用的代码，因为解析器仍在开发中
    fn peek_token_kind(&self) -> Option<&TokenKind> {
        self.current_token.as_ref().map(|t| &t.kind)
    }

    #[allow(dead_code)] // 允许未使用的代码，因为解析器仍在开发中
    fn expect_token(&mut self, kind: TokenKind, message: &str) -> ParseResult<Token> {
        self.advance()?;
        if self.peek_token_kind() == Some(&kind) {
            Ok(self.current_token.take().unwrap())
        } else {
            Err(ParseError::new_syntax_error(
                self.current_token.as_ref().map_or_else(|| SourceLocation::new("parser.rs", 0, 0), |t| t.location.clone()),
                message,
            ))
        }
    }

    // 获取当前 token 的 SourceLocation（若无当前 token，则构造占位 loc）
    fn current_location(&self) -> SourceLocation {
        self.current_token
            .as_ref()
            .map(|t| t.location.clone())
            .unwrap_or_else(|| SourceLocation::new("parser.rs", 0, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::Lexer;

    #[test]
    fn test_parse_simple_module() {
        let source = ".module test";
        let lexer = Lexer::new(source, "test.vil");
        let mut parser = Parser::new(lexer);
        let module = parser.parse_module().expect("应成功解析模块");
        assert_eq!(module.borrow().get_name(), "test");
    }
} 