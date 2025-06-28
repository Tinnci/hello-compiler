// Module 类实现
//
// 这个模块定义了 VIL 的模块类，包含函数和全局内存空间

use crate::ir::MemorySpace;
use crate::ir::function::FunctionRef; // 导入 FunctionRef
use crate::ir::types::{Type, TypeRef};
use crate::ir::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

// Module 引用
pub type ModuleRef = Rc<RefCell<Module>>;

/// 全局内存空间定义
#[derive(Debug)]
pub struct GlobalMemorySpace {
    name: String,
    space: MemorySpace,
    element_type: TypeRef,
    length: u32,
}

impl GlobalMemorySpace {
    pub fn new(name: String, space: MemorySpace, element_type: TypeRef, length: u32) -> Self {
        GlobalMemorySpace {
            name,
            space,
            element_type,
            length,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_space(&self) -> MemorySpace {
        self.space
    }

    pub fn get_element_type(&self) -> TypeRef {
        self.element_type.clone()
    }

    pub fn get_length(&self) -> u32 {
        self.length
    }
}

impl fmt::Display for GlobalMemorySpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            ".memory {} [{}] <{} x {}>",
            self.name,
            self.space,
            self.element_type.borrow(),
            self.length
        )
    }
}

/// Module 类，VIL 的顶层编译单元
#[derive(Debug)]
pub struct Module {
    value: Value, // 模块名
    functions: HashMap<String, FunctionRef>,
    global_memory_spaces: HashMap<String, Rc<RefCell<GlobalMemorySpace>>>,
}

impl Module {
    /// 创建一个新的模块
    pub fn new(name: String) -> Self {
        let void_type = Type::get_void_type(); // 模块没有具体类型
        Module {
            value: Value::new(void_type, name),
            functions: HashMap::new(),
            global_memory_spaces: HashMap::new(),
        }
    }

    /// 获取模块名称
    pub fn get_name(&self) -> &str {
        self.value.get_name()
    }

    /// 添加函数
    pub fn add_function(&mut self, func: FunctionRef) {
        self.functions
            .insert(func.borrow().get_name().to_string(), func.clone());
    }

    /// 通过名称获取函数
    pub fn get_function(&self, name: &str) -> Option<FunctionRef> {
        self.functions.get(name).cloned()
    }

    /// 获取所有函数
    pub fn get_functions(&self) -> Vec<FunctionRef> {
        self.functions.values().cloned().collect()
    }

    /// 添加全局内存空间
    pub fn add_global_memory_space(&mut self, mem_space: Rc<RefCell<GlobalMemorySpace>>) {
        self.global_memory_spaces
            .insert(mem_space.borrow().get_name().to_string(), mem_space.clone());
    }

    /// 通过名称获取全局内存空间
    pub fn get_global_memory_space(&self, name: &str) -> Option<Rc<RefCell<GlobalMemorySpace>>> {
        self.global_memory_spaces.get(name).cloned()
    }

    /// 获取所有全局内存空间
    pub fn get_global_memory_spaces(&self) -> Vec<Rc<RefCell<GlobalMemorySpace>>> {
        self.global_memory_spaces.values().cloned().collect()
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, ".module {}", self.get_name())?;
        writeln!(f, "")?;

        for mem_space in self.get_global_memory_spaces() {
            writeln!(f, "{}", mem_space.borrow())?;
        }
        writeln!(f, "")?;

        for func in self.get_functions() {
            writeln!(f, "{}", func.borrow())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::function::Function;
    use crate::ir::types::{Type, TypeKind};

    #[test]
    fn test_module_creation() {
        let module = Module::new("my_module".to_string());
        assert_eq!(module.get_name(), "my_module");
    }

    #[test]
    fn test_add_function_to_module() {
        let mut module = Module::new("test_module".to_string());
        let func = Rc::new(RefCell::new(Function::new(
            "test_func".to_string(),
            Type::get_void_type(),
            Vec::new(),
        )));
        module.add_function(func.clone());
        assert!(module.get_function("test_func").is_some());
        assert_eq!(module.get_functions().len(), 1);
    }

    #[test]
    fn test_add_global_memory_space_to_module() {
        let mut module = Module::new("test_module".to_string());
        let mem_space = Rc::new(RefCell::new(GlobalMemorySpace::new(
            "vspm_buffer".to_string(),
            MemorySpace::VSPM,
            Type::get_int_type(TypeKind::Int16),
            1024,
        )));
        module.add_global_memory_space(mem_space.clone());
        assert!(module.get_global_memory_space("vspm_buffer").is_some());
        assert_eq!(module.get_global_memory_spaces().len(), 1);
    }
}
