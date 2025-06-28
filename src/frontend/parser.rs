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

        self.advance()?; // Load the first token (should be .module)
        self.consume_expected_token(TokenKind::Module, "期望关键字 '.module'")?;
        // After this, `self.current_token` holds the module name.

        // 期望模块名称标识符
        let (module_name, _) = self.expect_identifier("期望模块名称")?;
        // After this, `self.current_token` holds the first top-level declaration (or EOF).

        let module_ref: ModuleRef = Rc::new(RefCell::new(Module::new(module_name)));

        loop {
            let current_kind_clone = self.peek_token_kind().cloned();
            let current_loc = self.current_location();

            match current_kind_clone {
                Some(TokenKind::Memory) => {
                    self.consume_expected_token(TokenKind::Memory, "期望关键字 '.memory'")?; // Consumes and advances
                    let mem_space = self.parse_global_memory_space()?; // parse_global_memory_space will assume current_token is the memory name, and consume/advance from there.
                    module_ref.borrow_mut().add_global_memory_space(Rc::new(RefCell::new(mem_space)));
                },
                Some(TokenKind::Function) => {
                    self.consume_expected_token(TokenKind::Function, "期望关键字 '.function'")?; // Consumes and advances
                    let func = self.parse_function()?; // parse_function will assume current_token is the function name, and consume/advance from there.
                    module_ref.borrow_mut().add_function(func);
                },
                Some(TokenKind::EOF) => break, // 文件结束
                None => break, // 文件结束
                _ => {
                    return Err(ParseError::new_syntax_error(
                        current_loc,
                        "模块级声明格式不正确，期望 .memory 或 .function",
                    ));
                }
            }
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

    /// 消费当前 token，如果它的种类匹配 `expected_kind`。
    /// 如果匹配成功，则将内部的 `current_token` 更新为下一个 token。
    /// 返回被消费的 token，或在不匹配时返回错误。
    fn consume_expected_token(&mut self, expected_kind: TokenKind, message: &str) -> ParseResult<Token> {
        let current_loc = self.current_location();
        let token_to_check = self.current_token.take();

        if let Some(token) = token_to_check {
            if token.kind == expected_kind {
                self.advance()?; // Advance to the next token AFTER successful consumption
                Ok(token)
            } else {
                // If not matched, put the token back so error reporting can point to it
                self.current_token = Some(token);
                Err(ParseError::new_syntax_error(
                    current_loc,
                    message,
                ))
            }
        } else {
            Err(ParseError::new_syntax_error(
                current_loc,
                "意外的文件结束，期望一个 token",
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

    /// 期望并消费一个标识符 token，返回其字符串值。
    /// 如果当前 token 不是标识符，则返回错误。
    fn expect_identifier(&mut self, message: &str) -> ParseResult<(String, SourceLocation)> {
        let current_loc = self.current_location();
        let token_to_check = self.current_token.take();

        if let Some(token) = token_to_check {
            if let TokenKind::Identifier(name) = token.kind {
                self.advance()?; // 成功消费并推进
                Ok((name, token.location))
            } else {
                self.current_token = Some(token); // 放回 token 以供错误报告
                Err(ParseError::new_syntax_error(current_loc, message))
            }
        } else {
            Err(ParseError::new_syntax_error(current_loc, "意外的文件结束，期望标识符"))
        }
    }

    /// 期望并消费一个整数常量 token，返回其数值。
    /// 如果当前 token 不是整数常量，则返回错误。
    fn expect_int_literal(&mut self, message: &str) -> ParseResult<(i64, SourceLocation)> {
        let current_loc = self.current_location();
        let token_to_check = self.current_token.take();

        if let Some(token) = token_to_check {
            if let TokenKind::IntLiteral(value) = token.kind {
                self.advance()?; // 成功消费并推进
                Ok((value, token.location))
            } else {
                self.current_token = Some(token); // 放回 token 以供错误报告
                Err(ParseError::new_syntax_error(current_loc, message))
            }
        } else {
            Err(ParseError::new_syntax_error(current_loc, "意外的文件结束，期望整数常量"))
        }
    }

    /// 期望并消费一个内存空间标识符（generic/vspm/sram/param 或普通 Identifier），返回其字符串值。
    /// 这用于解析诸如 "[vspm]" 或指针类型中的内存空间后缀。
    fn expect_memory_space_identifier(&mut self, message: &str) -> ParseResult<(String, SourceLocation)> {
        let current_loc = self.current_location();
        let token_to_check = self.current_token.take();

        if let Some(token) = token_to_check {
            match &token.kind {
                TokenKind::Identifier(s) => {
                    self.advance()?;
                    Ok((s.clone(), token.location))
                }
                TokenKind::Generic => {
                    self.advance()?;
                    Ok(("generic".to_string(), token.location))
                }
                TokenKind::VSPM => {
                    self.advance()?;
                    Ok(("vspm".to_string(), token.location))
                }
                TokenKind::SRAM => {
                    self.advance()?;
                    Ok(("sram".to_string(), token.location))
                }
                TokenKind::Parameter => {
                    self.advance()?;
                    Ok(("param".to_string(), token.location))
                }
                _ => {
                    // 非法 token，放回并报错
                    self.current_token = Some(token);
                    Err(ParseError::new_syntax_error(current_loc, message))
                }
            }
        } else {
            Err(ParseError::new_syntax_error(current_loc, "意外的文件结束，期望内存空间"))
        }
    }

    /// 解析基本类型 (i8, u8, i16, u16, i32, u32, b8, b16, b32, void)
    fn parse_base_type(&mut self) -> ParseResult<crate::ir::TypeRef> {
        // `current_token` should hold the base type token when this function is called.
        let token = self.current_token.as_ref().unwrap(); // Use as_ref() to avoid taking ownership prematurely
        let kind = &token.kind;
        let _location = token.location.clone(); // 已标记为未使用

        let result_type = match kind {
            TokenKind::Identifier(s) => match s.as_str() {
                "i8" => Ok(crate::ir::Type::get_int_type(crate::ir::TypeKind::Int8)),
                "u8" => Ok(crate::ir::Type::get_int_type(crate::ir::TypeKind::Uint8)),
                "i16" => Ok(crate::ir::Type::get_int_type(crate::ir::TypeKind::Int16)),
                "u16" => Ok(crate::ir::Type::get_int_type(crate::ir::TypeKind::Uint16)),
                "i32" => Ok(crate::ir::Type::get_int_type(crate::ir::TypeKind::Int32)),
                "u32" => Ok(crate::ir::Type::get_int_type(crate::ir::TypeKind::Uint32)),
                "b8" => Ok(crate::ir::Type::get_bit_type(crate::ir::TypeKind::Bit8)),
                "b16" => Ok(crate::ir::Type::get_bit_type(crate::ir::TypeKind::Bit16)),
                "b32" => Ok(crate::ir::Type::get_bit_type(crate::ir::TypeKind::Bit32)),
                "void" => Ok(crate::ir::Type::get_void_type()),
                _ => Err(ParseError::new_syntax_error(
                    _location,
                    &format!("未知基本类型: '{}'", s),
                )),
            },
            _ => Err(ParseError::new_syntax_error(
                _location,
                "期望基本类型标识符",
            )),
        }?;
        
        // After successfully determining the type, consume the token and advance.
        self.current_token.take(); // Consume the token
        self.advance()?; // Advance to the next token

        Ok(result_type)
    }

    /// 解析 VIL 类型，例如 `<i32 x 4>`, `<pred 32>`, `i16* vspm`, `void`, `i32`
    fn parse_type(&mut self) -> ParseResult<crate::ir::TypeRef> {
        // `current_token` should hold the type's first token when this function is called.
        if self.peek_token_kind() == Some(&TokenKind::LAngle) {
            self.consume_expected_token(TokenKind::LAngle, "期望 '<' 开始类型声明")?;
            // `current_token` now holds the inner token (pred or base type).

            let inner_token_kind = self.peek_token_kind().cloned().unwrap_or(TokenKind::Unknown);
            let _inner_location = self.current_location(); // 已标记为未使用

            let parsed_type = match inner_token_kind {
                TokenKind::Identifier(s) if s == "pred" => {
                    let (pred_keyword, pred_location) = self.expect_identifier("期望 'pred' 关键字")?;
                    if pred_keyword != "pred" { return Err(ParseError::new_syntax_error(pred_location, "期望 'pred' 关键字")); }
                    let (length_val, _) = self.expect_int_literal("期望谓词长度")?;
                    let length = length_val as u32;
                    crate::ir::Type::get_predicate_type(length)
                },
                _ => {
                    // `current_token` holds the element type. parse_base_type will consume it and advance.
                    let element_type = self.parse_base_type()?; // parse_base_type consumes its token and advances.
                    // `current_token` now holds 'x'.
                    let (x_keyword, x_location) = self.expect_identifier("期望 'x'")?;
                    if x_keyword != "x" { return Err(ParseError::new_syntax_error(x_location, "期望 'x'")); }
                    let (length_val, _) = self.expect_int_literal("期望向量长度")?;
                    let length = length_val as u32;
                    crate::ir::Type::get_vector_type(element_type, length)
                }
            };
            
            // `current_token` now holds `>`.
            self.consume_expected_token(TokenKind::RAngle, "期望 '>' 闭合类型声明")?;
            // `current_token` now holds the token *after* `>` (could be `*` or something else).

            // Check for pointer type
            if self.peek_token_kind() == Some(&TokenKind::Star) {
                self.consume_expected_token(TokenKind::Star, "期望 '*'")?;
                // `current_token` now holds the memory space identifier.
                let (mem_space_name, mem_space_location) = self.expect_memory_space_identifier("期望内存空间")?;
                let mem_space = parse_memory_space_from_ident(&mem_space_name, mem_space_location)?;
                // `current_token` now holds the token *after* the memory space.
                Ok(crate::ir::Type::get_pointer_type(parsed_type, mem_space))
            } else {
                Ok(parsed_type)
            }

        } else {
            // `current_token` holds the base type.
            let base_type = self.parse_base_type()?; // parse_base_type consumes and advances.
            // `current_token` now holds the token *after* the base type.

            if self.peek_token_kind() == Some(&TokenKind::Star) {
                self.consume_expected_token(TokenKind::Star, "期望 '*'")?;
                // `current_token` now holds the memory space identifier.
                let (mem_space_name, mem_space_location) = self.expect_memory_space_identifier("期望内存空间")?;
                let mem_space = parse_memory_space_from_ident(&mem_space_name, mem_space_location)?;
                // `current_token` now holds the token *after* the memory space.
                Ok(crate::ir::Type::get_pointer_type(base_type, mem_space))
            } else {
                Ok(base_type)
            }
        }
    }

    /// 解析函数参数: `.param %name <type>` 或 `.result %name <type>`
    fn parse_argument(&mut self, is_result_param: bool) -> ParseResult<crate::ir::function::ArgumentRef> {
        let _is_result_param = is_result_param; // 已标记为未使用
        // `current_token` should hold the argument name when this function is called.
        let (name, name_location) = self.expect_identifier("期望参数名称 (例如: %in1)")?;
        if !name.starts_with("%") {
            return Err(ParseError::new_syntax_error(
                name_location,
                "参数名称应以 '%' 开头",
            ));
        }
        // `current_token` now holds the argument type.
        let arg_type = self.parse_type()?; // parse_type consumes its tokens and advances `current_token`.

        Ok(Rc::new(RefCell::new(crate::ir::function::Argument::new(
            arg_type, name, None, 0,
        ))))
    }

    /// 解析全局内存空间声明: `.memory <name> [memory_space] <element_type x length>`
    fn parse_global_memory_space(&mut self) -> ParseResult<crate::ir::module::GlobalMemorySpace> {
        let _start_location = self.current_location(); // 已标记为未使用
        // `current_token` should hold the memory name when this function is called.
        println!("parse_global_memory_space: ENTRY - current_token: {:?}", self.current_token);
        let (name, _) = self.expect_identifier("期望内存空间名称")?;
        // `current_token` now holds `[`.
        self.consume_expected_token(TokenKind::LBracket, "期望 '[' 开始内存空间指定")?;
        // `current_token` now holds space identifier
        let (space_name, space_location) = self.expect_memory_space_identifier("期望内存空间类型 (e.g., vspm, sram)")?;
        println!("parse_global_memory_space: AFTER SPACE_NAME - current_token: {:?}", self.current_token);
        let space = parse_memory_space_from_ident(&space_name, space_location)?;
        // `current_token` now holds `]`
        self.consume_expected_token(TokenKind::RBracket, "期望 ']' 闭合内存空间指定")?;
        println!("parse_global_memory_space: AFTER RBRACKET - current_token: {:?}", self.current_token);
        // 解析元素类型
        let elem_type_token = self.parse_type()?; // parse_type 将消费其相关 token。

        // 根据是否为向量类型决定长度来源：
        let length: u32 = {
            use crate::ir::types::TypeKind;
            let elem_kind = elem_type_token.borrow().get_kind().clone();
            match elem_kind {
                TypeKind::Vector(_, vec_len) => vec_len, // 从向量类型本身提取长度
                _ => {
                    // 非向量类型，需要显式的整数长度
                    if matches!(self.peek_token_kind(), Some(&TokenKind::IntLiteral(_))) {
                        let (length_val, _) = self.expect_int_literal("期望内存空间长度")?;
                        length_val as u32
                    } else {
                        return Err(ParseError::new_syntax_error(
                            self.current_location(),
                            "期望内存空间长度",
                        ));
                    }
                }
            }
        };
        // 处理完长度后，current_token 指向长度之后（若有显式长度）的 token。
        // `current_token` now holds the token *after* the length.

        Ok(crate::ir::module::GlobalMemorySpace::new(name, space, elem_type_token, length))
    }

    /// 解析函数声明: `.function <name>(<params>) { <body> }`
    fn parse_function(&mut self) -> ParseResult<crate::ir::FunctionRef> {
        let _start_location = self.current_location(); // 已标记为未使用
        // `current_token` should hold the function name when this function is called.
        let (name, _) = self.expect_identifier("期望函数名称")?;
        // `current_token` now holds `(`.
        self.consume_expected_token(TokenKind::LParen, "期望 '(' 开始参数列表")?;

        let mut arguments = Vec::new();
        let mut param_types = Vec::new();

        // 解析参数列表：连续的 .param/.result 项，逗号分隔
        while matches!(self.peek_token_kind(), Some(&TokenKind::Param)) || matches!(self.peek_token_kind(), Some(&TokenKind::Result)) {
            let is_result_param = matches!(self.peek_token_kind(), Some(&TokenKind::Result));
            if is_result_param {
                self.consume_expected_token(TokenKind::Result, "期望关键字 '.result'")?;
            } else {
                self.consume_expected_token(TokenKind::Param, "期望关键字 '.param'")?;
            }

            // 现在 current_token 应为参数名称
            let arg_ref = self.parse_argument(is_result_param)?;
            param_types.push(arg_ref.borrow().get_type());
            arguments.push(arg_ref);

            // 如果后面还有逗号，则消费
            if self.peek_token_kind() == Some(&TokenKind::Comma) {
                self.consume_expected_token(TokenKind::Comma, "期望 ',' 分隔参数或 ')' 结束")?;
            }
        }

        // 参数解析完毕，期望 ')'
        self.consume_expected_token(TokenKind::RParen, "期望 ')' 闭合参数列表")?;

        // 解析完参数列表后，期望出现函数体的大括号起始 '{'
        self.consume_expected_token(TokenKind::LBrace, "期望 '{' 开始函数体")?;

        // 跳过函数体：使用大括号深度计数，直到匹配完毕。
        let mut brace_depth = 1;
        while brace_depth > 0 {
            let kind_opt = self.peek_token_kind().cloned();
            match kind_opt {
                Some(TokenKind::LBrace) => {
                    self.advance()?; // consume '{'
                    brace_depth += 1;
                }
                Some(TokenKind::RBrace) => {
                    self.advance()?; // consume '}'
                    brace_depth -= 1;
                }
                Some(TokenKind::EOF) | None => {
                    return Err(ParseError::new_syntax_error(self.current_location(), "函数体未正确闭合"));
                }
                _ => {
                    self.advance()?; // consume other tokens
                }
            }
        }
        // 结束循环时已消费配对的 '}'，current_token 指向 '}' 之后的 token。

        // 构造函数 IR 对象
        let return_type = crate::ir::Type::get_void_type();
        let function_ref = Rc::new(RefCell::new(crate::ir::Function::new(
            name,
            return_type,
            param_types,
        )));

        for arg in &arguments {
            arg.borrow_mut().set_parent(Some(Rc::downgrade(&function_ref)));
            function_ref.borrow_mut().add_argument(arg.clone());
        }

        Ok(function_ref)
    }
}

/// 解析内存空间标识符到 MemorySpace 枚举
fn parse_memory_space_from_ident(ident: &str, location: SourceLocation) -> ParseResult<crate::ir::MemorySpace> {
    match ident {
        "generic" => Ok(crate::ir::MemorySpace::Generic),
        "vspm" => Ok(crate::ir::MemorySpace::VSPM),
        "sram" => Ok(crate::ir::MemorySpace::SRAM),
        "param" => Ok(crate::ir::MemorySpace::Parameter),
        _ => Err(ParseError::new_syntax_error(
            location,
            &format!("未知内存空间: '{}'", ident),
        )),
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

    #[test]
    fn test_parse_module_with_memory() {
        let source = r#".module my_module
.memory vspm_buffer [vspm] <i16 x 1024>
.memory sram_buffer [sram] i32 512
            "#;
        let lexer = Lexer::new(source, "test.vil");
        let mut parser = Parser::new(lexer);
        let module = parser.parse_module().expect("应成功解析模块");

        assert_eq!(module.borrow().get_name(), "my_module");
        assert_eq!(module.borrow().get_global_memory_spaces().len(), 2);

        let mem1 = module.borrow().get_global_memory_space("vspm_buffer").unwrap();
        assert_eq!(mem1.borrow().get_name(), "vspm_buffer");
        assert_eq!(mem1.borrow().get_space(), crate::ir::MemorySpace::VSPM);
        assert_eq!(mem1.borrow().get_element_type().borrow().to_string(), "<i16 x 1024>");
        assert_eq!(mem1.borrow().get_length(), 1024);

        let mem2 = module.borrow().get_global_memory_space("sram_buffer").unwrap();
        assert_eq!(mem2.borrow().get_name(), "sram_buffer");
        assert_eq!(mem2.borrow().get_space(), crate::ir::MemorySpace::SRAM);
        assert_eq!(mem2.borrow().get_element_type().borrow().to_string(), "i32");
        assert_eq!(mem2.borrow().get_length(), 512);
    }

    #[test]
    fn test_parse_module_with_function() {
        let source = r#".module my_module
.function my_func(.param %in1 i32, .param %in2 <i16 x 4>, .result %out i32* sram) {
    // function body
}
            "#;
        let lexer = Lexer::new(source, "test.vil");
        let mut parser = Parser::new(lexer);
        let module = parser.parse_module().expect("应成功解析模块");

        assert_eq!(module.borrow().get_name(), "my_module");
        assert_eq!(module.borrow().get_functions().len(), 1);

        let func = module.borrow().get_function("my_func").unwrap();
        assert_eq!(func.borrow().get_name(), "my_func");
        assert_eq!(func.borrow().get_arguments().len(), 3);

        let func_borrowed = func.borrow();
        let arg1 = func_borrowed.get_arguments()[0].borrow();
        assert_eq!(arg1.get_name(), "%in1");
        assert_eq!(arg1.get_type().borrow().to_string(), "i32");

        let arg2 = func_borrowed.get_arguments()[1].borrow();
        assert_eq!(arg2.get_name(), "%in2");
        assert_eq!(arg2.get_type().borrow().to_string(), "<i16 x 4>");

        let arg3 = func_borrowed.get_arguments()[2].borrow();
        assert_eq!(arg3.get_name(), "%out");
        assert_eq!(arg3.get_type().borrow().to_string(), "i32* sram");
    }
} 