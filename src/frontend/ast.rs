// AST 模块
//
// 这个模块定义了 VIL 的抽象语法树 (AST) 节点

use crate::frontend::error::SourceLocation;
use crate::ir::{BasicBlockRef, FunctionRef, InstructionRef, ModuleRef};

/// AST 节点
#[derive(Debug)]
pub enum ASTNode {
    Module(ModuleRef),
    Function(FunctionRef),
    BasicBlock(BasicBlockRef),
    Instruction(InstructionRef),
    // TODO: 添加更多 AST 节点类型
}

impl ASTNode {
    /// 获取节点的位置信息
    pub fn get_location(&self) -> SourceLocation {
        // 实际实现中，这里需要根据具体的 AST 节点类型返回其对应的 SourceLocation
        SourceLocation::new("ast.rs", 0, 0) // 占位符
    }
}
