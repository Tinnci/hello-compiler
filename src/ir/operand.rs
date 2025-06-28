// Operand 类实现
//
// 这个模块定义了 VIL 的操作数类，表示指令的输入

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use crate::ir::types::TypeRef;
use crate::ir::value::ValueRef;
use crate::ir::basic_block::BasicBlock;

// Operand 引用
pub type OperandRef = Rc<RefCell<Operand>>;

/// 操作数种类
#[derive(Debug, Clone)]
pub enum OperandKind {
    Value(ValueRef),            // 值操作数
    Immediate(i64, TypeRef),    // 立即数操作数
    BasicBlock(Rc<RefCell<BasicBlock>>),  // 基本块操作数
}

impl PartialEq for OperandKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OperandKind::Value(s), OperandKind::Value(o)) => s.borrow().eq(&o.borrow()),
            (OperandKind::Immediate(s_val, s_type), OperandKind::Immediate(o_val, o_type)) => {
                s_val == o_val && s_type.borrow().eq(&o_type.borrow())
            },
            (OperandKind::BasicBlock(s), OperandKind::BasicBlock(o)) => Rc::ptr_eq(s, o),
            _ => false,
        }
    }
}

/// 操作数类，表示指令的输入
#[derive(Debug, Clone, PartialEq)]
pub struct Operand {
    kind: OperandKind,
}

impl Operand {
    /// 创建值操作数
    pub fn create_value(value: ValueRef) -> OperandRef {
        Rc::new(RefCell::new(Operand {
            kind: OperandKind::Value(value),
        }))
    }
    
    /// 创建立即数操作数
    pub fn create_immediate(value: i64, type_: TypeRef) -> OperandRef {
        Rc::new(RefCell::new(Operand {
            kind: OperandKind::Immediate(value, type_),
        }))
    }
    
    /// 创建基本块操作数
    pub fn create_basic_block(bb: Rc<RefCell<BasicBlock>>) -> OperandRef {
        Rc::new(RefCell::new(Operand {
            kind: OperandKind::BasicBlock(bb),
        }))
    }
    
    /// 获取操作数种类
    pub fn get_kind(&self) -> &OperandKind {
        &self.kind
    }
    
    /// 判断是否为值操作数
    pub fn is_value(&self) -> bool {
        matches!(self.kind, OperandKind::Value(_))
    }
    
    /// 判断是否为立即数操作数
    pub fn is_immediate(&self) -> bool {
        matches!(self.kind, OperandKind::Immediate(_, _))
    }
    
    /// 判断是否为基本块操作数
    pub fn is_basic_block(&self) -> bool {
        matches!(self.kind, OperandKind::BasicBlock(_))
    }
    
    /// 获取值操作数
    pub fn get_value(&self) -> Option<ValueRef> {
        match &self.kind {
            OperandKind::Value(value) => Some(value.clone()),
            _ => None,
        }
    }
    
    /// 获取立即数操作数
    pub fn get_immediate(&self) -> Option<i64> {
        match &self.kind {
            OperandKind::Immediate(value, _) => Some(*value),
            _ => None,
        }
    }
    
    /// 获取基本块操作数
    pub fn get_basic_block(&self) -> Option<Rc<RefCell<BasicBlock>>> {
        match &self.kind {
            OperandKind::BasicBlock(bb) => Some(bb.clone()),
            _ => None,
        }
    }
    
    /// 获取操作数类型
    pub fn get_type(&self) -> Option<TypeRef> {
        match &self.kind {
            OperandKind::Value(value) => Some(value.borrow().get_type()),
            OperandKind::Immediate(_, type_) => Some(type_.clone()),
            OperandKind::BasicBlock(_) => None, // 基本块没有类型
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            OperandKind::Value(value) => write!(f, "{}", value.borrow()),
            OperandKind::Immediate(value, type_) => write!(f, "{} {}", value, type_.borrow()),
            OperandKind::BasicBlock(bb) => write!(f, "label {}", bb.borrow().get_name()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::{Type, TypeKind};
    use crate::ir::value::Value;
    use crate::ir::basic_block::BasicBlock;
    
    #[test]
    fn test_value_operand() {
        let int_type = Type::get_int_type(TypeKind::Int32);
        let value = Rc::new(RefCell::new(Value::new(int_type, "test_value".to_string())));
        let operand = Operand::create_value(value);
        
        assert!(operand.borrow().is_value());
        assert!(!operand.borrow().is_immediate());
        assert!(!operand.borrow().is_basic_block());
        
        let retrieved_value = operand.borrow().get_value().unwrap();
        assert_eq!(retrieved_value.borrow().get_name(), "test_value");
    }
    
    #[test]
    fn test_immediate_operand() {
        let int_type = Type::get_int_type(TypeKind::Int32);
        let operand = Operand::create_immediate(42, int_type);
        
        assert!(!operand.borrow().is_value());
        assert!(operand.borrow().is_immediate());
        assert!(!operand.borrow().is_basic_block());
        
        assert_eq!(operand.borrow().get_immediate().unwrap(), 42);
    }
} 