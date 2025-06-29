// BasicBlock 类实现
//
// 这个模块定义了 VIL 的基本块类，包含指令序列

use crate::ir::function::Function;
use crate::ir::instruction::InstructionRef;
use crate::ir::types::Type;
use crate::ir::value::Value;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

// BasicBlock 引用
pub type BasicBlockRef = Rc<RefCell<BasicBlock>>;

/// 基本块类，包含指令序列
pub struct BasicBlock {
    // 继承自 Value
    value: Value,
    // BasicBlock 特有字段
    parent: Option<Rc<RefCell<Function>>>,
    instructions: Vec<InstructionRef>,
}

impl BasicBlock {
    /// 创建一个新的基本块
    pub fn new(name: String, parent: Option<Rc<RefCell<Function>>>) -> Self {
        // 基本块没有实际类型，使用 void 类型
        let void_type = Type::get_void_type();

        BasicBlock {
            value: Value::new(void_type, name),
            parent,
            instructions: Vec::new(),
        }
    }

    /// 获取基本块名称
    pub fn get_name(&self) -> &str {
        self.value.get_name()
    }

    /// 设置基本块名称
    pub fn set_name(&mut self, name: String) {
        self.value.set_name(name);
    }

    /// 获取所属函数
    pub fn get_parent(&self) -> Option<Rc<RefCell<Function>>> {
        self.parent.clone()
    }

    /// 设置所属函数
    pub fn set_parent(&mut self, parent: Option<Rc<RefCell<Function>>>) {
        self.parent = parent;
    }

    /// 获取指令列表
    pub fn get_instructions(&self) -> &[InstructionRef] {
        &self.instructions
    }

    /// 添加指令
    pub fn add_instruction(&mut self, instruction: InstructionRef, this_bb_ref: BasicBlockRef) {
        // 设置指令的父基本块
        instruction
            .borrow_mut()
            .set_parent_bb(Some(this_bb_ref.clone())); // Clone the Rc to pass it
        self.instructions.push(instruction);
    }

    /// 插入指令到指定位置
    pub fn insert_instruction(&mut self, index: usize, instruction: InstructionRef, this_bb_ref: BasicBlockRef) {
        assert!(index <= self.instructions.len());
        // 设置指令的父基本块
        instruction
            .borrow_mut()
            .set_parent_bb(Some(this_bb_ref.clone())); // Clone the Rc to pass it
        self.instructions.insert(index, instruction);
    }

    /// 移除指令
    pub fn remove_instruction(&mut self, instruction: &InstructionRef) -> bool {
        if let Some(pos) = self
            .instructions
            .iter()
            .position(|i| Rc::ptr_eq(i, instruction))
        {
            // 清除指令的父基本块
            self.instructions[pos].borrow_mut().set_parent_bb(None);
            self.instructions.remove(pos);
            true
        } else {
            false
        }
    }

    /// 清空所有指令
    pub fn clear_instructions(&mut self) {
        // 清除所有指令的父基本块
        for instruction in &self.instructions {
            instruction.borrow_mut().set_parent_bb(None);
        }
        self.instructions.clear();
    }

    /// 获取终结指令
    pub fn get_terminator(&self) -> Option<InstructionRef> {
        self.instructions.last().cloned()
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.get_name())?;
        for instruction in &self.instructions {
            writeln!(f, "  {}", instruction.borrow())?;
        }
        Ok(())
    }
}

impl fmt::Debug for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BasicBlock({}, {} instructions)",
            self.get_name(),
            self.instructions.len()
        )
    }
}

impl Clone for BasicBlock {
    fn clone(&self) -> Self {
        BasicBlock {
            value: Value::new(self.value.get_type(), self.value.get_name().to_string()),
            parent: self.parent.clone(),
            instructions: self.instructions.clone(),
        }
    }
}
