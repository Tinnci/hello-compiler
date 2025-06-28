// Function 类实现
//
// 这个模块定义了 VIL 的函数类，包含参数和基本块

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;
use crate::ir::value::Value;
use crate::ir::types::{Type, TypeRef, TypeKind};
use crate::ir::basic_block::BasicBlockRef; // 导入 BasicBlockRef

// Function 引用
pub type FunctionRef = Rc<RefCell<Function>>;
// Weak Function 引用 (用于避免循环引用)
pub type WeakFunctionRef = Weak<RefCell<Function>>;

// Argument 引用
pub type ArgumentRef = Rc<RefCell<Argument>>;

/// 函数参数类
#[derive(Debug)]
pub struct Argument {
    value: Value,
    parent: Option<WeakFunctionRef>, // 所属函数 (弱引用)
    arg_idx: usize,             // 参数索引
}

impl Argument {
    /// 创建一个新的函数参数
    pub fn new(type_: TypeRef, name: String, parent: Option<WeakFunctionRef>, arg_idx: usize) -> Self {
        Argument {
            value: Value::new(type_, name),
            parent,
            arg_idx,
        }
    }

    /// 获取参数名称
    pub fn get_name(&self) -> &str {
        self.value.get_name()
    }

    /// 获取参数类型
    pub fn get_type(&self) -> TypeRef {
        self.value.get_type()
    }

    /// 获取参数索引
    pub fn get_arg_idx(&self) -> usize {
        self.arg_idx
    }

    /// 获取所属函数 (尝试升级为强引用)
    pub fn get_parent(&self) -> Option<FunctionRef> {
        self.parent.as_ref().and_then(|weak_ref| weak_ref.upgrade())
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.get_name(), self.get_type().borrow())
    }
}

/// 函数类，包含基本块和参数
#[derive(Debug)]
pub struct Function {
    value: Value, // 函数名和函数类型 (TypeKind::Function)
    arguments: Vec<ArgumentRef>,
    basic_blocks: Vec<BasicBlockRef>,
}

impl Function {
    /// 创建一个新的函数
    pub fn new(name: String, return_type: TypeRef, param_types: Vec<TypeRef>) -> Self {
        let function_type = Type::get_function_type(return_type, param_types);
        Function {
            value: Value::new(function_type, name),
            arguments: Vec::new(),
            basic_blocks: Vec::new(),
        }
    }

    /// 获取函数名称
    pub fn get_name(&self) -> &str {
        self.value.get_name()
    }

    /// 获取函数类型
    pub fn get_type(&self) -> TypeRef {
        self.value.get_type()
    }

    /// 获取返回类型
    pub fn get_return_type(&self) -> TypeRef {
        if let TypeKind::Function(ret_type, _) = self.value.get_type().borrow().get_kind() {
            ret_type.clone()
        } else {
            // This should not happen for a Function type
            panic!("Expected Function type");
        }
    }

    /// 获取参数类型列表
    pub fn get_param_types(&self) -> Vec<TypeRef> {
        if let TypeKind::Function(_, param_types) = self.value.get_type().borrow().get_kind() {
            param_types.clone()
        } else {
            // This should not happen for a Function type
            panic!("Expected Function type");
        }
    }

    /// 获取入口基本块
    pub fn get_entry_block(&self) -> Option<BasicBlockRef> {
        self.basic_blocks.first().cloned()
    }

    /// 添加基本块
    /// 注意：此方法不负责设置基本块的父函数，调用者应在添加后手动设置，
    /// 以避免在此处产生循环引用或不必要的克隆。
    pub fn add_basic_block(&mut self, bb: BasicBlockRef) {
        self.basic_blocks.push(bb);
    }

    /// 获取参数列表
    pub fn get_arguments(&self) -> &[ArgumentRef] {
        &self.arguments
    }
    
    /// 添加参数
    pub fn add_argument(&mut self, arg: ArgumentRef) {
        self.arguments.push(arg);
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, ".function {}(", self.get_name())?;
        for (i, arg) in self.arguments.iter().enumerate() {
            if i > 0 {
                writeln!(f, ", ")?;
            }
            write!(f, "    .param {}", arg.borrow())?;
        }
        writeln!(f, ") {{")?;
        
        for bb in &self.basic_blocks {
            writeln!(f, "{}", bb.borrow())?;
        }
        writeln!(f, "}}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::{Type, TypeKind};
    
    #[test]
    fn test_function_creation() {
        let ret_type = Type::get_void_type();
        let param_type1 = Type::get_int_type(TypeKind::Int32);
        let param_type2 = Type::get_vector_type(Type::get_int_type(TypeKind::Int16), 4);
        let param_types = vec![param_type1.clone(), param_type2.clone()];
        
        let func = Function::new("my_func".to_string(), ret_type.clone(), param_types.clone());
        
        assert_eq!(func.get_name(), "my_func");
        assert_eq!(func.get_return_type().borrow().to_string(), "void");
        assert_eq!(func.get_param_types().len(), 2);
        assert_eq!(func.get_param_types()[0].borrow().to_string(), "i32");
        assert_eq!(func.get_param_types()[1].borrow().to_string(), "<i16 x 4>");
    }

    #[test]
    fn test_argument_creation() {
        let int_type = Type::get_int_type(TypeKind::Int32);
        let arg = Argument::new(int_type.clone(), "arg0".to_string(), None, 0);
        assert_eq!(arg.get_name(), "arg0");
        assert_eq!(arg.get_type().borrow().to_string(), "i32");
        assert_eq!(arg.get_arg_idx(), 0);
    }
} 