// IR 模块入口点
//
// 这个模块包含中间表示(IR)的所有核心数据结构和操作

// 子模块
pub mod basic_block;
pub mod function;
pub mod instruction;
pub mod module;
pub mod operand;
pub mod types;
pub mod value;

// 重新导出常用类型
pub use basic_block::{BasicBlock, BasicBlockRef};
pub use function::{Argument, ArgumentRef, Function, FunctionRef};
pub use instruction::{Instruction, InstructionModifier, InstructionRef, Opcode};
pub use module::{Module, ModuleRef};
pub use operand::{Operand, OperandRef};
pub use types::{Type, TypeKind, TypeRef};
pub use value::{Value, ValueRef};

// 内存空间枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemorySpace {
    Generic,   // 通用内存空间
    VSPM,      // 向量暂存器内存
    SRAM,      // 标量内存
    Parameter, // 参数内存空间
}

impl std::fmt::Display for MemorySpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemorySpace::Generic => write!(f, "generic"),
            MemorySpace::VSPM => write!(f, "vspm"),
            MemorySpace::SRAM => write!(f, "sram"),
            MemorySpace::Parameter => write!(f, "param"),
        }
    }
}
