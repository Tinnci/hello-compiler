// Value 类实现
//
// 这个模块定义了 VIL 的 Value 类，是所有 IR 元素的基类

use crate::ir::types::TypeRef;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::hash::{Hash, Hasher};

// Value 引用，使用 Rc<RefCell<T>> 代替 C++ 中的 std::shared_ptr<T>
pub type ValueRef = Rc<RefCell<Value>>;

/// IR 中的值。可以是指令结果、函数参数或常量。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
    type_: TypeRef,
    name: String,
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_.borrow().hash(state);
        self.name.hash(state);
    }
}

impl Value {
    /// 创建一个新的 Value
    pub fn new(type_: TypeRef, name: String) -> Self {
        Self {
            type_,
            name,
        }
    }

    /// 获取值的类型
    pub fn get_type(&self) -> TypeRef {
        self.type_.clone()
    }

    /// 设置值的类型
    pub fn set_type(&mut self, type_: TypeRef) {
        self.type_ = type_;
    }

    /// 获取值的名称
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// 设置值的名称
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// 检查此值是否为常量 (通过名称是否能解析为数字判断)
    pub fn is_constant(&self) -> bool {
        self.name.parse::<i64>().is_ok() || self.name.parse::<f64>().is_ok()
    }

    /// 判断该值是否为对其他指令结果的引用（简单地认为以 '%' 开头且非常量）
    pub fn is_reference(&self) -> bool {
        !self.is_constant() && self.name.starts_with('%')
    }

    /// 如果是整型常量，返回其 i64 值
    pub fn as_i64(&self) -> Option<i64> {
        self.name.parse::<i64>().ok()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.name.is_empty() {
            write!(f, "<unnamed:{}>", self.type_.borrow())
        } else {
            write!(f, "{}:{}", self.name, self.type_.borrow())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::types::{Type, TypeKind};

    #[test]
    fn test_value_basics() {
        let int_type = Type::get_int_type(TypeKind::Int32);
        let value = Value::new(int_type.clone(), "test_value".to_string());

        assert_eq!(value.get_name(), "test_value");
        assert_eq!(value.to_string(), "test_value:i32");
        assert!(!value.is_constant());

        // 测试修改名称
        let mut value2 = Value::new(int_type, String::new());
        assert_eq!(value2.to_string(), "<unnamed:i32>");

        value2.set_name("renamed".to_string());
        assert_eq!(value2.get_name(), "renamed");
        assert_eq!(value2.to_string(), "renamed:i32");
    }
}
