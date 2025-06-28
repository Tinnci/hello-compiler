// Instruction 类实现
//
// 这个模块定义了 VIL 的指令类，包括各种指令类型

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use crate::ir::value::{Value, ValueRef};
use crate::ir::types::{Type, TypeRef};
use crate::ir::operand::{Operand, OperandRef};
use crate::ir::basic_block::{BasicBlock, BasicBlockRef};
use crate::ir::MemorySpace;

// Instruction 引用
pub type InstructionRef = Rc<RefCell<Instruction>>;

/// 指令操作码枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    // 算术指令
    Add,        // 加法
    Sub,        // 减法
    Mul,        // 乘法
    SAdd,       // 向量与标量加法
    SMul,       // 向量与标量乘法
    Sra,        // 算术右移
    Srl,        // 逻辑右移
    Sll,        // 左移
    
    // 逻辑指令
    And,        // 按位与
    Or,         // 按位或
    Xor,        // 按位异或
    Not,        // 按位取反
    
    // 比较指令
    CmpEq,      // 等于比较
    CmpNe,      // 不等比较
    CmpGt,      // 大于比较
    CmpGe,      // 大于等于比较
    CmpLt,      // 小于比较
    CmpLe,      // 小于等于比较
    
    // 谓词操作指令
    PredAnd,    // 谓词与
    PredOr,     // 谓词或
    PredNot,    // 谓词取反
    
    // 内存操作指令
    Load,       // 加载
    Store,      // 存储
    
    // 归约指令
    RedSum,     // 求和归约
    RedMax,     // 最大值归约
    RedMin,     // 最小值归约
    
    // 特殊指令
    Range,      // 生成序列
    Broadcast,  // 广播标量
    Shuffle,    // 向量洗牌
    Alloc,      // 分配内存
    Free,       // 释放内存
    
    // 控制流指令
    Br,         // 无条件跳转
    CondBr,     // 条件跳转
    Ret,        // 函数返回
    
    // 其他
    Mov,        // 移动/复制
    Phi,        // Phi节点
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Opcode::Add => "add",
            Opcode::Sub => "sub",
            Opcode::Mul => "mul",
            Opcode::SAdd => "sadd",
            Opcode::SMul => "smul",
            Opcode::Sra => "sra",
            Opcode::Srl => "srl",
            Opcode::Sll => "sll",
            Opcode::And => "and",
            Opcode::Or => "or",
            Opcode::Xor => "xor",
            Opcode::Not => "not",
            Opcode::CmpEq => "cmpeq",
            Opcode::CmpNe => "cmpne",
            Opcode::CmpGt => "cmpgt",
            Opcode::CmpGe => "cmpge",
            Opcode::CmpLt => "cmplt",
            Opcode::CmpLe => "cmple",
            Opcode::PredAnd => "pand",
            Opcode::PredOr => "por",
            Opcode::PredNot => "pnot",
            Opcode::Load => "load",
            Opcode::Store => "store",
            Opcode::RedSum => "redsum",
            Opcode::RedMax => "redmax",
            Opcode::RedMin => "redmin",
            Opcode::Range => "range",
            Opcode::Broadcast => "broadcast",
            Opcode::Shuffle => "shuffle",
            Opcode::Alloc => "alloc",
            Opcode::Free => "free",
            Opcode::Br => "br",
            Opcode::CondBr => "condbr",
            Opcode::Ret => "ret",
            Opcode::Mov => "mov",
            Opcode::Phi => "phi",
        };
        write!(f, "{}", name)
    }
}

/// 指令修饰符枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstructionModifier {
    None,       // 无修饰符
    Vector,     // 向量操作
    Scalar,     // 标量操作
    Predicate,  // 谓词操作
}

impl fmt::Display for InstructionModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionModifier::None => write!(f, ""),
            InstructionModifier::Vector => write!(f, ".v"),
            InstructionModifier::Scalar => write!(f, ".s"),
            InstructionModifier::Predicate => write!(f, ".p"),
        }
    }
}

/// 指令基类
pub struct Instruction {
    // 继承自 Value
    value: Value,
    // Instruction 特有字段
    opcode: Opcode,
    modifier: InstructionModifier,
    parent: Option<BasicBlockRef>,
    operands: Vec<OperandRef>,
    predicate: Option<OperandRef>,
}

impl Instruction {
    /// 创建一个新的指令
    pub fn new(opcode: Opcode, type_: TypeRef, modifier: InstructionModifier) -> Self {
        Instruction {
            value: Value::new(type_, String::new()),
            opcode,
            modifier,
            parent: None,
            operands: Vec::new(),
            predicate: None,
        }
    }
    
    /// 获取操作码
    pub fn get_opcode(&self) -> Opcode {
        self.opcode
    }
    
    /// 获取修饰符
    pub fn get_modifier(&self) -> InstructionModifier {
        self.modifier
    }
    
    /// 获取所属的基本块
    pub fn get_parent(&self) -> Option<BasicBlockRef> {
        self.parent.clone()
    }
    
    /// 设置所属的基本块
    pub fn set_parent(&mut self, parent: Option<BasicBlockRef>) {
        self.parent = parent;
    }
    
    /// 获取操作数数量
    pub fn get_num_operands(&self) -> usize {
        self.operands.len()
    }
    
    /// 获取指定索引的操作数
    pub fn get_operand(&self, index: usize) -> Option<OperandRef> {
        if index < self.operands.len() {
            Some(self.operands[index].clone())
        } else {
            None
        }
    }
    
    /// 设置指定索引的操作数
    pub fn set_operand(&mut self, index: usize, operand: OperandRef) {
        if index < self.operands.len() {
            self.operands[index] = operand;
        } else if index == self.operands.len() {
            self.operands.push(operand);
        } else {
            panic!("操作数索引超出范围");
        }
    }
    
    /// 添加操作数
    pub fn add_operand(&mut self, operand: OperandRef) {
        self.operands.push(operand);
    }
    
    /// 获取所有操作数
    pub fn get_operands(&self) -> &[OperandRef] {
        &self.operands
    }
    
    /// 获取谓词操作数（条件执行）
    pub fn get_predicate(&self) -> Option<OperandRef> {
        self.predicate.clone()
    }
    
    /// 设置谓词操作数（条件执行）
    pub fn set_predicate(&mut self, predicate: Option<OperandRef>) {
        self.predicate = predicate;
    }
    
    /// 是否为条件执行
    pub fn is_predicated(&self) -> bool {
        self.predicate.is_some()
    }
    
    /// 判断指令是否产生结果（即是否具有返回值）
    pub fn has_result(&self) -> bool {
        match self.opcode {
            Opcode::Store | Opcode::Free | Opcode::Br | Opcode::CondBr => false,
            Opcode::Ret => false, // 无返回值的返回指令
            _ => true,
        }
    }
    
    /// 获取指令的名称
    pub fn get_name(&self) -> &str {
        self.value.get_name()
    }
    
    /// 设置指令的名称
    pub fn set_name(&mut self, name: String) {
        self.value.set_name(name);
    }
    
    /// 获取指令的类型
    pub fn get_type(&self) -> TypeRef {
        self.value.get_type()
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 输出结果赋值部分
        if self.has_result() {
            let name = if self.get_name().is_empty() {
                "%_".to_string()
            } else {
                format!("%{}", self.get_name())
            };
            write!(f, "{} = ", name)?;
        }
        
        // 输出操作码和修饰符
        write!(f, "{}{}", self.opcode, self.modifier)?;
        
        // 输出谓词（如果有）
        if let Some(pred) = &self.predicate {
            write!(f, " [{}]", pred.borrow())?;
        }
        
        // 输出操作数
        if !self.operands.is_empty() {
            write!(f, " ")?;
            for (i, op) in self.operands.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", op.borrow())?;
            }
        }
        
        Ok(())
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instruction({}{}, {} operands)", self.opcode, self.modifier, self.operands.len())
    }
}
// 以下是各种具体的指令类型实现
// 在 Rust 中，我们可以使用 enum 来表示不同类型的指令，而不是像 C++ 那样使用继承

/// 指令类型枚举
#[derive(Debug)]
pub enum InstructionKind {
    Binary(BinaryInstruction),
    Memory(MemoryInstruction),
    Reduction(ReductionInstruction),
    ControlFlow(ControlFlowInstruction),
    Special(SpecialInstruction),
    Move(MoveInstruction),
}

/// 二元运算指令
#[derive(Debug)]
pub struct BinaryInstruction {
    instruction: Instruction,
}

impl BinaryInstruction {
    /// 创建一个新的二元运算指令
    pub fn new(opcode: Opcode, type_: TypeRef, lhs: OperandRef, rhs: OperandRef, modifier: InstructionModifier) -> Self {
        let mut instruction = Instruction::new(opcode, type_, modifier);
        instruction.add_operand(lhs);
        instruction.add_operand(rhs);
        BinaryInstruction { instruction }
    }
    
    /// 获取左操作数
    pub fn get_lhs(&self) -> OperandRef {
        self.instruction.get_operand(0).unwrap()
    }
    
    /// 获取右操作数
    pub fn get_rhs(&self) -> OperandRef {
        self.instruction.get_operand(1).unwrap()
    }
    
    /// 设置左操作数
    pub fn set_lhs(&mut self, lhs: OperandRef) {
        self.instruction.set_operand(0, lhs);
    }
    
    /// 设置右操作数
    pub fn set_rhs(&mut self, rhs: OperandRef) {
        self.instruction.set_operand(1, rhs);
    }
}

/// 内存操作指令
#[derive(Debug)]
pub struct MemoryInstruction {
    instruction: Instruction,
    space: MemorySpace,
}

impl MemoryInstruction {
    /// 创建一个新的内存操作指令
    pub fn new(opcode: Opcode, type_: TypeRef, space: MemorySpace, modifier: InstructionModifier) -> Self {
        MemoryInstruction {
            instruction: Instruction::new(opcode, type_, modifier),
            space,
        }
    }
    
    /// 获取内存空间
    pub fn get_memory_space(&self) -> MemorySpace {
        self.space
    }
}

/// 加载指令
#[derive(Debug)]
pub struct LoadInstruction {
    memory_instruction: MemoryInstruction,
}

impl LoadInstruction {
    /// 创建一个新的加载指令
    pub fn new(type_: TypeRef, address: OperandRef, space: MemorySpace, modifier: InstructionModifier) -> Self {
        let mut memory_instruction = MemoryInstruction::new(Opcode::Load, type_, space, modifier);
        memory_instruction.instruction.add_operand(address);
        LoadInstruction { memory_instruction }
    }
    
    /// 获取地址操作数
    pub fn get_address(&self) -> OperandRef {
        self.memory_instruction.instruction.get_operand(0).unwrap()
    }
    
    /// 设置地址操作数
    pub fn set_address(&mut self, address: OperandRef) {
        self.memory_instruction.instruction.set_operand(0, address);
    }
}

/// 存储指令
#[derive(Debug)]
pub struct StoreInstruction {
    memory_instruction: MemoryInstruction,
}

impl StoreInstruction {
    /// 创建一个新的存储指令
    pub fn new(value: OperandRef, address: OperandRef, space: MemorySpace, modifier: InstructionModifier) -> Self {
        // 存储指令没有返回值，使用 void 类型
        let void_type = Type::get_void_type();
        let mut memory_instruction = MemoryInstruction::new(Opcode::Store, void_type, space, modifier);
        memory_instruction.instruction.add_operand(value);
        memory_instruction.instruction.add_operand(address);
        StoreInstruction { memory_instruction }
    }
    
    /// 获取值操作数
    pub fn get_value(&self) -> OperandRef {
        self.memory_instruction.instruction.get_operand(0).unwrap()
    }
    
    /// 获取地址操作数
    pub fn get_address(&self) -> OperandRef {
        self.memory_instruction.instruction.get_operand(1).unwrap()
    }
    
    /// 设置值操作数
    pub fn set_value(&mut self, value: OperandRef) {
        self.memory_instruction.instruction.set_operand(0, value);
    }
    
    /// 设置地址操作数
    pub fn set_address(&mut self, address: OperandRef) {
        self.memory_instruction.instruction.set_operand(1, address);
    }
}

/// 归约指令
#[derive(Debug)]
pub struct ReductionInstruction {
    instruction: Instruction,
}

impl ReductionInstruction {
    /// 创建一个新的归约指令
    pub fn new(opcode: Opcode, result_type: TypeRef, vector: OperandRef, modifier: InstructionModifier) -> Self {
        let mut instruction = Instruction::new(opcode, result_type, modifier);
        instruction.add_operand(vector);
        ReductionInstruction { instruction }
    }
    
    /// 获取向量操作数
    pub fn get_vector(&self) -> OperandRef {
        self.instruction.get_operand(0).unwrap()
    }
    
    /// 设置向量操作数
    pub fn set_vector(&mut self, vector: OperandRef) {
        self.instruction.set_operand(0, vector);
    }
}

/// 控制流指令
#[derive(Debug)]
pub struct ControlFlowInstruction {
    instruction: Instruction,
}

impl ControlFlowInstruction {
    /// 创建一个新的控制流指令
    pub fn new(opcode: Opcode, type_: TypeRef) -> Self {
        ControlFlowInstruction {
            instruction: Instruction::new(opcode, type_, InstructionModifier::None),
        }
    }
    
    /// 是否为终结指令
    pub fn is_terminator(&self) -> bool {
        true
    }
}

/// 特殊指令
#[derive(Debug)]
pub struct SpecialInstruction {
    instruction: Instruction,
}

impl SpecialInstruction {
    /// 创建一个新的特殊指令
    pub fn new(opcode: Opcode, type_: TypeRef, modifier: InstructionModifier) -> Self {
        SpecialInstruction {
            instruction: Instruction::new(opcode, type_, modifier),
        }
    }
}

/// 移动/复制指令
#[derive(Debug)]
pub struct MoveInstruction {
    instruction: Instruction,
}

impl MoveInstruction {
    /// 创建一个新的移动/复制指令
    pub fn new(type_: TypeRef, source: OperandRef) -> Self {
        let mut instruction = Instruction::new(Opcode::Mov, type_, InstructionModifier::None);
        instruction.add_operand(source);
        MoveInstruction { instruction }
    }
    
    /// 获取源操作数
    pub fn get_source(&self) -> OperandRef {
        self.instruction.get_operand(0).unwrap()
    }
    
    /// 设置源操作数
    pub fn set_source(&mut self, source: OperandRef) {
        self.instruction.set_operand(0, source);
    }
} 
