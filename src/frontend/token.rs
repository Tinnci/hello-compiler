// 词法单元模块
//
// 这个模块定义了 VIL 的词法单元类型

use crate::frontend::error::SourceLocation;
use std::fmt;

/// 词法单元种类
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // 标点符号
    Dot,       // .
    Comma,     // ,
    Colon,     // :
    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
    LAngle,    // <
    RAngle,    // >
    Equal,     // =
    At,        // @

    // 关键字
    Module,   // .module
    Function, // .function
    Memory,   // .memory
    Param,    // .param
    Entry,    // .entry
    Result,   // .result

    // 操作码
    Add,       // add
    Sub,       // sub
    Mul,       // mul
    SAdd,      // sadd
    SMul,      // smul
    Sra,       // sra
    Srl,       // srl
    Sll,       // sll
    And,       // and
    Or,        // or
    Xor,       // xor
    Not,       // not
    CmpEq,     // cmpeq
    CmpNe,     // cmpne
    CmpGt,     // cmpgt
    CmpGe,     // cmpge
    CmpLt,     // cmplt
    CmpLe,     // cmple
    PredAnd,   // pand
    PredOr,    // por
    PredNot,   // pnot
    Load,      // load
    Store,     // store
    RedSum,    // redsum
    RedMax,    // redmax
    RedMin,    // redmin
    Range,     // range
    Broadcast, // broadcast
    Shuffle,   // shuffle
    Alloc,     // alloc
    Free,      // free
    Br,        // br
    CondBr,    // condbr
    Ret,       // ret
    Mov,       // mov
    Phi,       // phi

    // 修饰符
    Vector,    // .v
    Scalar,    // .s
    Predicate, // .p

    // 内存空间
    Generic,   // generic
    VSPM,      // vspm
    SRAM,      // sram
    Parameter, // param

    // 标识符和字面量
    Identifier(String),    // 标识符
    IntLiteral(i64),       // 整数字面量
    StringLiteral(String), // 字符串字面量

    // 特殊标记
    EOF,     // 文件结束
    Unknown, // 未知标记
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Dot => write!(f, "."),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::LAngle => write!(f, "<"),
            TokenKind::RAngle => write!(f, ">"),
            TokenKind::Equal => write!(f, "="),
            TokenKind::At => write!(f, "@"),

            TokenKind::Module => write!(f, ".module"),
            TokenKind::Function => write!(f, ".function"),
            TokenKind::Memory => write!(f, ".memory"),
            TokenKind::Param => write!(f, ".param"),
            TokenKind::Entry => write!(f, ".entry"),
            TokenKind::Result => write!(f, ".result"),

            TokenKind::Add => write!(f, "add"),
            TokenKind::Sub => write!(f, "sub"),
            TokenKind::Mul => write!(f, "mul"),
            TokenKind::SAdd => write!(f, "sadd"),
            TokenKind::SMul => write!(f, "smul"),
            TokenKind::Sra => write!(f, "sra"),
            TokenKind::Srl => write!(f, "srl"),
            TokenKind::Sll => write!(f, "sll"),
            TokenKind::And => write!(f, "and"),
            TokenKind::Or => write!(f, "or"),
            TokenKind::Xor => write!(f, "xor"),
            TokenKind::Not => write!(f, "not"),
            TokenKind::CmpEq => write!(f, "cmpeq"),
            TokenKind::CmpNe => write!(f, "cmpne"),
            TokenKind::CmpGt => write!(f, "cmpgt"),
            TokenKind::CmpGe => write!(f, "cmpge"),
            TokenKind::CmpLt => write!(f, "cmplt"),
            TokenKind::CmpLe => write!(f, "cmple"),
            TokenKind::PredAnd => write!(f, "pand"),
            TokenKind::PredOr => write!(f, "por"),
            TokenKind::PredNot => write!(f, "pnot"),
            TokenKind::Load => write!(f, "load"),
            TokenKind::Store => write!(f, "store"),
            TokenKind::RedSum => write!(f, "redsum"),
            TokenKind::RedMax => write!(f, "redmax"),
            TokenKind::RedMin => write!(f, "redmin"),
            TokenKind::Range => write!(f, "range"),
            TokenKind::Broadcast => write!(f, "broadcast"),
            TokenKind::Shuffle => write!(f, "shuffle"),
            TokenKind::Alloc => write!(f, "alloc"),
            TokenKind::Free => write!(f, "free"),
            TokenKind::Br => write!(f, "br"),
            TokenKind::CondBr => write!(f, "condbr"),
            TokenKind::Ret => write!(f, "ret"),
            TokenKind::Mov => write!(f, "mov"),
            TokenKind::Phi => write!(f, "phi"),

            TokenKind::Vector => write!(f, ".v"),
            TokenKind::Scalar => write!(f, ".s"),
            TokenKind::Predicate => write!(f, ".p"),

            TokenKind::Generic => write!(f, "generic"),
            TokenKind::VSPM => write!(f, "vspm"),
            TokenKind::SRAM => write!(f, "sram"),
            TokenKind::Parameter => write!(f, "param"),

            TokenKind::Identifier(s) => write!(f, "{}", s),
            TokenKind::IntLiteral(n) => write!(f, "{}", n),
            TokenKind::StringLiteral(s) => write!(f, "\"{}\"", s),

            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

/// 词法单元
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub location: SourceLocation,
}

impl Token {
    pub fn new(kind: TokenKind, location: SourceLocation) -> Self {
        Token { kind, location }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}", self.kind, self.location)
    }
}
