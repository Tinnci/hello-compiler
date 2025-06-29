// Instruction 类实现
//
// 这个模块定义了 VIL 的指令类，包括各种指令类型

use crate::ir::MemorySpace;
use crate::ir::basic_block::BasicBlockRef;
use crate::ir::types::{Type, TypeRef};
use crate::ir::value::{Value, ValueRef};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

// Instruction 引用
pub type InstructionRef = Rc<RefCell<Instruction>>;

/// 指令操作码枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    // 算术指令
    Add,  // 加法
    Sub,  // 减法
    Mul,  // 乘法
    SAdd, // 向量与标量加法
    SMul, // 向量与标量乘法
    Sra,  // 算术右移
    Srl,  // 逻辑右移
    Sll,  // 左移

    // 逻辑指令
    And, // 按位与
    Or,  // 按位或
    Xor, // 按位异或
    Not, // 按位取反

    // 比较指令
    CmpEq, // 等于比较
    CmpNe, // 不等比较
    CmpGt, // 大于比较
    CmpGe, // 大于等于比较
    CmpLt, // 小于比较
    CmpLe, // 小于等于比较

    // 谓词操作指令
    PredAnd, // 谓词与
    PredOr,  // 谓词或
    PredNot, // 谓词取反

    // 内存操作指令
    Load,  // 加载
    Store, // 存储

    // 归约指令
    RedSum, // 求和归约
    RedMax, // 最大值归约
    RedMin, // 最小值归约

    // 特殊指令
    Range,     // 生成序列
    Broadcast, // 广播标量
    Shuffle,   // 向量洗牌
    Alloc,     // 分配内存
    Free,      // 释放内存

    // 控制流指令
    Br,     // 无条件跳转
    CondBr, // 条件跳转
    Ret,    // 函数返回

    // 其他
    Mov, // 移动/复制
    Phi, // Phi节点

    // --- Venus 扩展指令 (来自硬件 OP_TYPE 枚举) ---
    // 乘法扩展
    MulH,   // 高位乘法 (signed * signed)
    MulHU,  // 高位乘法 (unsigned * unsigned)
    MulHSU, // 高位乘法 (signed * unsigned)
    MulAdd, // 乘加
    MulSub, // 乘减
    AddMul, // 加后乘
    SubMul, // 减后乘
    CmxMul, // 复数乘 (Complex Multiply)

    // 除法/取余
    Div,  // 除法 (signed)
    DivU, // 除法 (unsigned)
    Rem,  // 取余 (signed)
    RemU, // 取余 (unsigned)

    // 饱和算术
    SAddSat,  // 饱和加法 (signed)
    SAddUSat, // 饱和加法 (unsigned)
    SSubSat,  // 饱和减法 (signed)
    SSubUSat, // 饱和减法 (unsigned)

    // 其他扩展
    RSub,         // 反向减法
    ShuffleClbmv, // 特殊洗牌指令
    SetCsr,       // 设置 CSR
    Yield,        // 让出执行权
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
            Opcode::MulH => "mulh",
            Opcode::MulHU => "mulhu",
            Opcode::MulHSU => "mulhsu",
            Opcode::MulAdd => "muladd",
            Opcode::MulSub => "mulsub",
            Opcode::AddMul => "addmul",
            Opcode::SubMul => "submul",
            Opcode::CmxMul => "cmxmul",
            Opcode::Div => "div",
            Opcode::DivU => "divu",
            Opcode::Rem => "rem",
            Opcode::RemU => "remu",
            Opcode::SAddSat => "saddsat",
            Opcode::SAddUSat => "saddusat",
            Opcode::SSubSat => "ssubsat",
            Opcode::SSubUSat => "ssubusat",
            Opcode::RSub => "rsub",
            Opcode::ShuffleClbmv => "shuffle_clbmv",
            Opcode::SetCsr => "setcsr",
            Opcode::Yield => "yield",
        };
        write!(f, "{}", name)
    }
}

/// 指令修饰符枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstructionModifier {
    None,      // 无修饰符
    Vector,    // 向量操作
    Scalar,    // 标量操作
    Predicate, // 谓词操作
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
#[derive(Debug, Clone)]
pub struct Instruction {
    opcode: Opcode,
    result: Option<ValueRef>, // 指令结果，如果指令产生一个值
    operands: Vec<ValueRef>,   // 操作数 (Changed from OperandRef to ValueRef)
    parent_bb: Option<BasicBlockRef>, // 所属的基本块
    attributes: Vec<String>, // 指令属性，如 "volatile" (Moved from Value)
    modifier: InstructionModifier, // Added back modifier
}

impl Instruction {
    pub fn new(
        opcode: Opcode,
        result: Option<ValueRef>,
        operands: Vec<ValueRef>,
        modifier: InstructionModifier,
    ) -> Self {
        Self {
            opcode,
            result,
            operands,
            parent_bb: None,
            attributes: Vec::new(),
            modifier,
        }
    }

    pub fn get_opcode(&self) -> Opcode {
        self.opcode.clone()
    }

    pub fn get_type(&self) -> TypeRef {
        self.result
            .as_ref()
            .map(|v| v.borrow().get_type())
            .unwrap_or_else(|| {
                // Default to Int32 for instructions that don't produce a value
                Type::get_int_type(crate::ir::types::TypeKind::Int32)
            })
    }

    pub fn get_name(&self) -> Option<String> {
        self.result.as_ref().map(|v| v.borrow().get_name().to_string())
    }

    pub fn set_name(&mut self, name: String) {
        if let Some(res) = &self.result {
            res.borrow_mut().set_name(name);
        }
    }

    // Changed to return ValueRef directly
    pub fn get_operand(&self, index: usize) -> ValueRef {
        self.operands[index].clone()
    }

    // Changed to accept ValueRef directly
    pub fn set_operand(&mut self, index: usize, operand: ValueRef) {
        self.operands[index] = operand;
    }

    // Renamed from get_num_operands
    pub fn get_operand_count(&self) -> usize {
        self.operands.len()
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    pub fn get_result(&self) -> Option<ValueRef> {
        self.result.clone()
    }

    // Modified to accept Option<BasicBlockRef>
    pub fn set_parent_bb(&mut self, bb: Option<BasicBlockRef>) {
        self.parent_bb = bb;
    }

    pub fn get_parent_bb(&self) -> Option<BasicBlockRef> {
        self.parent_bb.clone()
    }

    // New: Add an attribute to the instruction
    pub fn add_attribute(&mut self, attr: String) {
        self.attributes.push(attr);
    }

    // New: Check if the instruction has a specific attribute
    pub fn has_attribute(&self, attr: &str) -> bool {
        self.attributes.contains(&attr.to_string())
    }

    /// 替换当前指令为一个常量值
    /// 这将把指令的结果值名称设置为常量字符串，并清空操作数和操作码，使其成为一个"常量"指令。
    pub fn replace_with_constant(&mut self, constant_name: String) {
        if let Some(result_val_ref) = &self.result {
            let mut result_val = result_val_ref.borrow_mut();
            result_val.set_name(constant_name);
            // 清空操作数和操作码，表示这是一个常数指令
            self.opcode = Opcode::Mov; // 使用 Mov 指令来表示一个常量的直接移动
            self.operands.clear();
        }
    }

    pub fn get_operands(&self) -> &Vec<ValueRef> {
        &self.operands
    }

    // Add back get_modifier (it was removed in previous iteration but existed in original code)
    pub fn get_modifier(&self) -> InstructionModifier {
        self.modifier
    }

    // Add back set_modifier (it was removed in previous iteration but existed in original code)
    pub fn set_modifier(&mut self, modifier: InstructionModifier) {
        self.modifier = modifier;
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 输出结果赋值部分
        if self.has_result() {
            let name_str = self.get_name().unwrap_or_default(); // Use unwrap_or_default() as get_name returns Option<String>
            let formatted_name = if name_str.is_empty() {
                "%_".to_string()
            } else {
                format!("%{}", name_str)
            };
            write!(f, "{} = ", formatted_name)?;
        }

        // 输出操作码和修饰符
        write!(f, "{}{}", self.opcode, self.modifier)?;

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
    pub fn new(
        opcode: Opcode,
        type_: TypeRef,
        lhs: ValueRef,
        rhs: ValueRef,
        modifier: InstructionModifier,
    ) -> Self {
        let instruction = Instruction::new(
            opcode,
            Some(Rc::new(RefCell::new(Value::new(type_, "".to_string())))),
            vec![lhs, rhs],
            modifier,
        );
        BinaryInstruction { instruction }
    }

    /// 获取左操作数
    pub fn get_lhs(&self) -> ValueRef {
        self.instruction.get_operand(0).clone()
    }

    /// 获取右操作数
    pub fn get_rhs(&self) -> ValueRef {
        self.instruction.get_operand(1).clone()
    }

    /// 设置左操作数
    pub fn set_lhs(&mut self, lhs: ValueRef) {
        self.instruction.set_operand(0, lhs);
    }

    /// 设置右操作数
    pub fn set_rhs(&mut self, rhs: ValueRef) {
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
    pub fn new(
        opcode: Opcode,
        result_type: TypeRef, // This is the type of the loaded value, not the result of Instruction::new
        space: MemorySpace,
        modifier: InstructionModifier,
    ) -> Self {
        let result_val = if opcode == Opcode::Load {
            Some(Rc::new(RefCell::new(Value::new(result_type, "".to_string()))))
        } else {
            None // For Store, it doesn't produce a value
        };
        MemoryInstruction {
            instruction: Instruction::new(opcode, result_val, Vec::new(), modifier),
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
    pub fn new(
        type_: TypeRef,
        address: ValueRef,
        space: MemorySpace,
        modifier: InstructionModifier,
    ) -> Self {
        // Load指令产生一个值，所以MemoryInstruction需要一个结果类型
        let mut memory_instruction = MemoryInstruction::new(Opcode::Load, type_, space, modifier);
        memory_instruction.instruction.set_operand(0, address);
        LoadInstruction { memory_instruction }
    }

    /// 获取地址操作数
    pub fn get_address(&self) -> ValueRef {
        self.memory_instruction.instruction.get_operand(0).clone()
    }

    /// 设置地址操作数
    pub fn set_address(&mut self, address: ValueRef) {
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
    pub fn new(
        value: ValueRef,
        address: ValueRef,
        space: MemorySpace,
        modifier: InstructionModifier,
    ) -> Self {
        let void_type = Type::get_void_type(); // Store指令没有返回值
        let mut memory_instruction = // changed to mut
            MemoryInstruction::new(Opcode::Store, void_type, space, modifier); // Here void_type is passed as result_type for MemoryInstruction::new
        memory_instruction.instruction.set_operand(0, value);
        memory_instruction.instruction.set_operand(1, address);
        StoreInstruction { memory_instruction }
    }

    /// 获取值操作数
    pub fn get_value(&self) -> ValueRef {
        self.memory_instruction.instruction.get_operand(0).clone()
    }

    /// 获取地址操作数
    pub fn get_address(&self) -> ValueRef {
        self.memory_instruction.instruction.get_operand(1).clone()
    }

    /// 设置值操作数
    pub fn set_value(&mut self, value: ValueRef) {
        self.memory_instruction.instruction.set_operand(0, value);
    }

    /// 设置地址操作数
    pub fn set_address(&mut self, address: ValueRef) {
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
    pub fn new(
        opcode: Opcode,
        result_type: TypeRef,
        vector: ValueRef,
        modifier: InstructionModifier,
    ) -> Self {
        let instruction = Instruction::new(
            opcode,
            Some(Rc::new(RefCell::new(Value::new(result_type, "".to_string())))),
            vec![vector],
            modifier,
        );
        ReductionInstruction { instruction }
    }

    /// 获取向量操作数
    pub fn get_vector(&self) -> ValueRef {
        self.instruction.get_operand(0).clone()
    }

    /// 设置向量操作数
    pub fn set_vector(&mut self, vector: ValueRef) {
        self.instruction.set_operand(0, vector);
    }
}

/// 控制流指令
#[derive(Debug)]
#[allow(dead_code)] // 允许未使用的代码，因为 instruction 字段通过方法访问
pub struct ControlFlowInstruction {
    instruction: Instruction,
}

impl ControlFlowInstruction {
    /// 创建一个新的控制流指令
    pub fn new(opcode: Opcode, _type_: TypeRef) -> Self { // type_ is unused, mark it with _
        ControlFlowInstruction {
            // 控制流指令通常不产生值
            instruction: Instruction::new(opcode, None, Vec::new(), InstructionModifier::None),
        }
    }

    /// 是否为终结指令
    pub fn is_terminator(&self) -> bool {
        true
    }
}

/// 特殊指令
#[derive(Debug)]
#[allow(dead_code)] // 允许未使用的代码，因为 instruction 字段通过方法访问
pub struct SpecialInstruction {
    instruction: Instruction,
}

impl SpecialInstruction {
    /// 创建一个新的特殊指令
    pub fn new(opcode: Opcode, type_: TypeRef, modifier: InstructionModifier) -> Self {
        SpecialInstruction {
            instruction: Instruction::new(
                opcode,
                Some(Rc::new(RefCell::new(Value::new(type_, "".to_string())))),
                Vec::new(),
                modifier,
            ),
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
    pub fn new(type_: TypeRef, source: ValueRef) -> Self {
        let instruction = Instruction::new(
            Opcode::Mov,
            Some(Rc::new(RefCell::new(Value::new(type_, "".to_string())))),
            vec![source],
            InstructionModifier::None,
        );
        MoveInstruction { instruction }
    }

    /// 获取源操作数
    pub fn get_source(&self) -> ValueRef {
        self.instruction.get_operand(0).clone()
    }

    /// 设置源操作数
    pub fn set_source(&mut self, source: ValueRef) {
        self.instruction.set_operand(0, source);
    }
}
