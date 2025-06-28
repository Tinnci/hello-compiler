// 词法分析器模块
//
// 这个模块实现了 VIL 的词法分析器，将源代码文本转换为词法单元序列

use crate::frontend::error::{ParseError, ParseResult, SourceLocation};
use crate::frontend::token::{Token, TokenKind};
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

/// 词法分析器
pub struct Lexer<'a> {
    #[allow(dead_code)] // 允许未使用的代码，因为它通过 chars 字段间接使用
    source: &'a str,
    filename: String,
    chars: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
    keywords: HashMap<String, TokenKind>,
}

impl<'a> Lexer<'a> {
    /// 创建一个新的词法分析器
    pub fn new(source: &'a str, filename: &str) -> Self {
        let mut keywords = HashMap::new();

        // 关键字
        keywords.insert(".module".to_string(), TokenKind::Module);
        keywords.insert(".function".to_string(), TokenKind::Function);
        keywords.insert(".memory".to_string(), TokenKind::Memory);
        keywords.insert(".param".to_string(), TokenKind::Param);
        keywords.insert(".entry".to_string(), TokenKind::Entry);
        keywords.insert(".result".to_string(), TokenKind::Result);
        keywords.insert(".type".to_string(), TokenKind::Type);

        // 操作码
        keywords.insert("add".to_string(), TokenKind::Add);
        keywords.insert("sub".to_string(), TokenKind::Sub);
        keywords.insert("mul".to_string(), TokenKind::Mul);
        keywords.insert("sadd".to_string(), TokenKind::SAdd);
        keywords.insert("smul".to_string(), TokenKind::SMul);
        keywords.insert("sra".to_string(), TokenKind::Sra);
        keywords.insert("srl".to_string(), TokenKind::Srl);
        keywords.insert("sll".to_string(), TokenKind::Sll);
        keywords.insert("and".to_string(), TokenKind::And);
        keywords.insert("or".to_string(), TokenKind::Or);
        keywords.insert("xor".to_string(), TokenKind::Xor);
        keywords.insert("not".to_string(), TokenKind::Not);
        keywords.insert("cmpeq".to_string(), TokenKind::CmpEq);
        keywords.insert("cmpne".to_string(), TokenKind::CmpNe);
        keywords.insert("cmpgt".to_string(), TokenKind::CmpGt);
        keywords.insert("cmpge".to_string(), TokenKind::CmpGe);
        keywords.insert("cmplt".to_string(), TokenKind::CmpLt);
        keywords.insert("cmple".to_string(), TokenKind::CmpLe);
        keywords.insert("pand".to_string(), TokenKind::PredAnd);
        keywords.insert("por".to_string(), TokenKind::PredOr);
        keywords.insert("pnot".to_string(), TokenKind::PredNot);
        keywords.insert("load".to_string(), TokenKind::Load);
        keywords.insert("store".to_string(), TokenKind::Store);
        keywords.insert("redsum".to_string(), TokenKind::RedSum);
        keywords.insert("redmax".to_string(), TokenKind::RedMax);
        keywords.insert("redmin".to_string(), TokenKind::RedMin);
        keywords.insert("range".to_string(), TokenKind::Range);
        keywords.insert("broadcast".to_string(), TokenKind::Broadcast);
        keywords.insert("shuffle".to_string(), TokenKind::Shuffle);
        keywords.insert("alloc".to_string(), TokenKind::Alloc);
        keywords.insert("free".to_string(), TokenKind::Free);
        keywords.insert("br".to_string(), TokenKind::Br);
        keywords.insert("condbr".to_string(), TokenKind::CondBr);
        keywords.insert("ret".to_string(), TokenKind::Ret);
        keywords.insert("mov".to_string(), TokenKind::Mov);
        keywords.insert("phi".to_string(), TokenKind::Phi);

        // 修饰符
        keywords.insert(".v".to_string(), TokenKind::Vector);
        keywords.insert(".s".to_string(), TokenKind::Scalar);
        keywords.insert(".p".to_string(), TokenKind::Predicate);

        // 内存空间
        keywords.insert("generic".to_string(), TokenKind::Generic);
        keywords.insert("vspm".to_string(), TokenKind::VSPM);
        keywords.insert("sram".to_string(), TokenKind::SRAM);
        keywords.insert("param".to_string(), TokenKind::Parameter);

        Lexer {
            source,
            filename: filename.to_string(),
            chars: source.chars().peekable(),
            line: 1,
            column: 1,
            keywords,
        }
    }

    /// 获取当前位置
    fn current_location(&self) -> SourceLocation {
        SourceLocation::new(&self.filename, self.line, self.column)
    }

    /// 读取下一个字符
    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some(c) = c {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        c
    }

    /// 查看下一个字符，但不消费
    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    /// 跳过空白字符
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    /// 跳过注释
    fn skip_comment(&mut self) {
        // 跳过 "//" 开头的行注释
        if let Some(&'/') = self.peek_char() {
            self.next_char(); // 消费第一个 '/'
            if let Some(&'/') = self.peek_char() {
                self.next_char(); // 消费第二个 '/'
                // 跳过直到行尾
                while let Some(&c) = self.peek_char() {
                    if c == '\n' {
                        break;
                    }
                    self.next_char();
                }
            }
        }
    }

    /// 读取标识符或关键字
    fn read_identifier(&mut self, first_char: char) -> TokenKind {
        let mut identifier = String::new();
        identifier.push(first_char);

        // 读取剩余的标识符字符
        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' || c == '.' {
                identifier.push(c);
                self.next_char();
            } else {
                break;
            }
        }

        // 检查是否是关键字
        self.keywords
            .get(&identifier)
            .cloned()
            .unwrap_or_else(|| TokenKind::Identifier(identifier))
    }

    /// 读取数字
    fn read_number(&mut self, first_char: char) -> TokenKind {
        let mut number = String::new();
        number.push(first_char);

        // 读取剩余的数字
        while let Some(&c) = self.peek_char() {
            if c.is_digit(10) {
                number.push(c);
                self.next_char();
            } else {
                break;
            }
        }

        // 解析为整数
        match number.parse::<i64>() {
            Ok(n) => TokenKind::IntLiteral(n),
            Err(_) => TokenKind::Unknown, // 解析失败
        }
    }

    /// 读取字符串字面量
    fn read_string(&mut self) -> ParseResult<TokenKind> {
        let mut string = String::new();
        let start_location = self.current_location();

        // 跳过开头的引号
        self.next_char();

        // 读取字符串内容
        while let Some(&c) = self.peek_char() {
            if c == '"' {
                self.next_char(); // 消费结束引号
                return Ok(TokenKind::StringLiteral(string));
            } else if c == '\\' {
                // 处理转义字符
                self.next_char(); // 消费反斜杠
                if let Some(&c) = self.peek_char() {
                    match c {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        _ => {
                            return Err(ParseError::new_lexical_error(
                                self.current_location(),
                                &format!("无效的转义字符: \\{}", c),
                            ));
                        }
                    }
                    self.next_char(); // 消费转义字符
                } else {
                    return Err(ParseError::new_lexical_error(
                        self.current_location(),
                        "字符串中的反斜杠后没有字符",
                    ));
                }
            } else {
                string.push(c);
                self.next_char();
            }
        }

        // 如果到达这里，说明字符串没有结束引号
        Err(ParseError::new_lexical_error(
            start_location,
            "未闭合的字符串字面量",
        ))
    }

    /// 获取下一个词法单元
    pub fn next_token(&mut self) -> ParseResult<Token> {
        self.skip_whitespace();

        // 检查是否到达文件末尾
        if let Some(&c) = self.peek_char() {
            let location = self.current_location();
            let kind = match c {
                // 标点符号
                '.' => {
                    let start_location = self.current_location();
                    let mut identifier = String::new();
                    identifier.push('.');
                    self.next_char(); // Consume the dot

                    // Read subsequent alphanumeric or underscore characters
                    while let Some(&c) = self.peek_char() {
                        if c.is_alphanumeric() || c == '_' {
                            identifier.push(c);
                            self.next_char();
                        } else {
                            break;
                        }
                    }

                    // Check if it's a keyword (like ".module", ".function")
                    if let Some(kind) = self.keywords.get(&identifier).cloned() {
                        kind
                    } else if identifier == "." { // It was just a dot
                        TokenKind::Dot
                    } else { // It started with . but not a recognized keyword or just a dot
                        // This could be an error or a different kind of identifier.
                        // For now, treat it as an Unknown token or a regular identifier.
                        // Depending on the VIL spec, this might need more specific error handling.
                        // Assuming it's an error for now if it doesn't match a keyword.
                        return Err(ParseError::new_lexical_error(
                            start_location,
                            &format!("未知的点前缀标识符: '{}'", identifier),
                        ));
                    }
                }
                ',' => {
                    self.next_char();
                    TokenKind::Comma
                }
                ':' => {
                    self.next_char();
                    TokenKind::Colon
                }
                ';' => {
                    self.next_char();
                    TokenKind::Semicolon
                }
                '(' => {
                    self.next_char();
                    TokenKind::LParen
                }
                ')' => {
                    self.next_char();
                    TokenKind::RParen
                }
                '{' => {
                    self.next_char();
                    TokenKind::LBrace
                }
                '}' => {
                    self.next_char();
                    TokenKind::RBrace
                }
                '[' => {
                    self.next_char();
                    TokenKind::LBracket
                }
                ']' => {
                    self.next_char();
                    TokenKind::RBracket
                }
                '<' => {
                    self.next_char();
                    TokenKind::LAngle
                }
                '>' => {
                    self.next_char();
                    TokenKind::RAngle
                }
                '=' => {
                    self.next_char();
                    TokenKind::Equal
                },
                '@' => {
                    self.next_char();
                    TokenKind::At
                },
                '*' => {
                    self.next_char();
                    TokenKind::Star
                },

                // 注释
                '/' => {
                    if let Some(&'/') = self.chars.peek() {
                        // 跳过注释
                        self.skip_comment();
                        return self.next_token();
                    } else {
                        self.next_char();
                        TokenKind::Unknown
                    }
                },

                // 字符串字面量
                '"' => return self.read_string().map(|kind| Token::new(kind, location)),

                // 数字
                c if c.is_digit(10) => {
                    self.next_char();
                    self.read_number(c)
                },

                // 标识符或关键字
                c if c.is_alphabetic() || c == '_' || c == '%' => {
                    self.next_char();
                    self.read_identifier(c)
                },

                // 未知字符
                _ => {
                    self.next_char();
                    TokenKind::Unknown
                }
            };

            Ok(Token::new(kind, location))
        } else {
            // 文件末尾
            Ok(Token::new(TokenKind::EOF, self.current_location()))
        }
    }

    /// 获取所有词法单元
    pub fn tokenize(&mut self) -> ParseResult<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let is_eof = token.kind == TokenKind::EOF;
            tokens.push(token);

            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_basic() {
        let source = ".module test\n.function main() {\n    ret;\n}";
        let mut lexer = Lexer::new(source, "test.vil");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0].kind, TokenKind::Module);
        assert_eq!(tokens[1].kind, TokenKind::Identifier("test".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Function);
        assert_eq!(tokens[3].kind, TokenKind::Identifier("main".to_string()));
        assert_eq!(tokens[4].kind, TokenKind::LParen);
        assert_eq!(tokens[5].kind, TokenKind::RParen);
        assert_eq!(tokens[6].kind, TokenKind::LBrace);
        assert_eq!(tokens[7].kind, TokenKind::Ret);
        assert_eq!(tokens[8].kind, TokenKind::Semicolon);
        assert_eq!(tokens[9].kind, TokenKind::RBrace);
        assert_eq!(tokens[10].kind, TokenKind::EOF);
        // ... 更多断言
    }

    #[test]
    fn test_lexer_keywords() {
        let source = "add sub mul";
        let mut lexer = Lexer::new(source, "test.vil");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4); // Add, Sub, Mul, EOF
        assert_eq!(tokens[0].kind, TokenKind::Add);
        assert_eq!(tokens[1].kind, TokenKind::Sub);
        assert_eq!(tokens[2].kind, TokenKind::Mul);
    }

    #[test]
    fn test_lexer_numbers() {
        let source = "123 456 789";
        let mut lexer = Lexer::new(source, "test.vil");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4); // 123, 456, 789, EOF
        if let TokenKind::IntLiteral(n) = tokens[0].kind {
            assert_eq!(n, 123);
        } else {
            panic!("Expected IntLiteral");
        }

        if let TokenKind::IntLiteral(n) = tokens[1].kind {
            assert_eq!(n, 456);
        } else {
            panic!("Expected IntLiteral");
        }

        if let TokenKind::IntLiteral(n) = tokens[2].kind {
            assert_eq!(n, 789);
        } else {
            panic!("Expected IntLiteral");
        }
    }
}
