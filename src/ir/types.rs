// 类型系统实现
//
// 这个模块定义了 VIL 的类型系统，包括基本类型、向量类型、谓词类型等

use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};
use crate::ir::MemorySpace;

// 类型引用，使用 Rc<RefCell<T>> 代替 C++ 中的 std::shared_ptr<T>
pub type TypeRef = Rc<RefCell<Type>>;

/// 类型种类枚举
#[derive(Debug, Clone)]
pub enum TypeKind {
    // 标量类型
    Int8,       // 8位有符号整数
    Uint8,      // 8位无符号整数
    Int16,      // 16位有符号整数
    Uint16,     // 16位无符号整数
    Int32,      // 32位有符号整数
    Uint32,     // 32位无符号整数
    Bit8,       // 8位位域
    Bit16,      // 16位位域
    Bit32,      // 32位位域
    
    // 复合类型
    Vector(TypeRef, u32),     // 向量类型(元素类型, 长度)
    Predicate(u32),           // 谓词类型(长度)
    Void,                     // 空类型
    Pointer(TypeRef, MemorySpace),  // 指针类型(指向类型, 内存空间)
    Function(TypeRef, Vec<TypeRef>),  // 函数类型(返回类型, 参数类型)
}

impl PartialEq for TypeKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeKind::Int8, TypeKind::Int8) => true,
            (TypeKind::Uint8, TypeKind::Uint8) => true,
            (TypeKind::Int16, TypeKind::Int16) => true,
            (TypeKind::Uint16, TypeKind::Uint16) => true,
            (TypeKind::Int32, TypeKind::Int32) => true,
            (TypeKind::Uint32, TypeKind::Uint32) => true,
            (TypeKind::Bit8, TypeKind::Bit8) => true,
            (TypeKind::Bit16, TypeKind::Bit16) => true,
            (TypeKind::Bit32, TypeKind::Bit32) => true,
            (TypeKind::Vector(elem_ty_self, len_self), TypeKind::Vector(elem_ty_other, len_other)) => {
                len_self == len_other && elem_ty_self.borrow().eq(&elem_ty_other.borrow())
            },
            (TypeKind::Predicate(len_self), TypeKind::Predicate(len_other)) => len_self == len_other,
            (TypeKind::Void, TypeKind::Void) => true,
            (TypeKind::Pointer(pointee_ty_self, space_self), TypeKind::Pointer(pointee_ty_other, space_other)) => {
                space_self == space_other && pointee_ty_self.borrow().eq(&pointee_ty_other.borrow())
            },
            (TypeKind::Function(ret_ty_self, param_tys_self), TypeKind::Function(ret_ty_other, param_tys_other)) => {
                ret_ty_self.borrow().eq(&ret_ty_other.borrow()) &&
                param_tys_self.len() == param_tys_other.len() &&
                param_tys_self.iter().zip(param_tys_other.iter()).all(|(s, o)| s.borrow().eq(&o.borrow()))
            },
            _ => false,
        }
    }
}

impl Eq for TypeKind {}

impl Hash for TypeKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            TypeKind::Int8 => "Int8".hash(state),
            TypeKind::Uint8 => "Uint8".hash(state),
            TypeKind::Int16 => "Int16".hash(state),
            TypeKind::Uint16 => "Uint16".hash(state),
            TypeKind::Int32 => "Int32".hash(state),
            TypeKind::Uint32 => "Uint32".hash(state),
            TypeKind::Bit8 => "Bit8".hash(state),
            TypeKind::Bit16 => "Bit16".hash(state),
            TypeKind::Bit32 => "Bit32".hash(state),
            TypeKind::Vector(elem_type, length) => {
                "Vector".hash(state);
                elem_type.borrow().hash(state);
                length.hash(state);
            },
            TypeKind::Predicate(length) => {
                "Predicate".hash(state);
                length.hash(state);
            },
            TypeKind::Void => "Void".hash(state),
            TypeKind::Pointer(pointee_type, space) => {
                "Pointer".hash(state);
                pointee_type.borrow().hash(state);
                space.hash(state);
            },
            TypeKind::Function(return_type, param_types) => {
                "Function".hash(state);
                return_type.borrow().hash(state);
                for param_type in param_types {
                    param_type.borrow().hash(state);
                }
            },
        }
    }
}

/// 类型结构体
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Type {
    kind: TypeKind,
}

impl Type {
    /// 创建一个新类型
    pub fn new(kind: TypeKind) -> Self {
        Type { kind }
    }
    
    /// 获取类型种类
    pub fn get_kind(&self) -> &TypeKind {
        &self.kind
    }
    
    /// 获取类型位宽
    pub fn get_bit_width(&self) -> u32 {
        match &self.kind {
            TypeKind::Int8 | TypeKind::Uint8 | TypeKind::Bit8 => 8,
            TypeKind::Int16 | TypeKind::Uint16 | TypeKind::Bit16 => 16,
            TypeKind::Int32 | TypeKind::Uint32 | TypeKind::Bit32 => 32,
            TypeKind::Vector(elem_type, length) => {
                elem_type.borrow().get_bit_width() * length
            },
            TypeKind::Predicate(length) => *length,
            TypeKind::Void => 0,
            TypeKind::Pointer(_, _) => 32, // 假设所有指针都是32位
            TypeKind::Function(_, _) => 0, // 函数类型没有位宽
        }
    }
    
    /// 检查是否为标量类型
    pub fn is_scalar(&self) -> bool {
        matches!(self.kind, 
            TypeKind::Int8 | TypeKind::Uint8 | 
            TypeKind::Int16 | TypeKind::Uint16 | 
            TypeKind::Int32 | TypeKind::Uint32 |
            TypeKind::Bit8 | TypeKind::Bit16 | TypeKind::Bit32
        )
    }
    
    /// 检查是否为向量类型
    pub fn is_vector(&self) -> bool {
        matches!(self.kind, TypeKind::Vector(_, _))
    }
    
    /// 检查是否为谓词类型
    pub fn is_predicate(&self) -> bool {
        matches!(self.kind, TypeKind::Predicate(_))
    }
    
    /// 检查是否为位域类型
    pub fn is_bit_type(&self) -> bool {
        matches!(self.kind, 
            TypeKind::Bit8 | TypeKind::Bit16 | TypeKind::Bit32
        )
    }
    
    /// 创建整数类型
    pub fn get_int_type(kind: TypeKind) -> TypeRef {
        assert!(matches!(kind, 
            TypeKind::Int8 | TypeKind::Uint8 | 
            TypeKind::Int16 | TypeKind::Uint16 | 
            TypeKind::Int32 | TypeKind::Uint32
        ));
        Rc::new(RefCell::new(Type::new(kind)))
    }
    
    /// 创建位域类型
    pub fn get_bit_type(kind: TypeKind) -> TypeRef {
        assert!(matches!(kind, 
            TypeKind::Bit8 | TypeKind::Bit16 | TypeKind::Bit32
        ));
        Rc::new(RefCell::new(Type::new(kind)))
    }
    
    /// 创建向量类型
    pub fn get_vector_type(element_type: TypeRef, length: u32) -> TypeRef {
        Rc::new(RefCell::new(Type::new(TypeKind::Vector(element_type, length))))
    }
    
    /// 创建谓词类型
    pub fn get_predicate_type(length: u32) -> TypeRef {
        Rc::new(RefCell::new(Type::new(TypeKind::Predicate(length))))
    }
    
    /// 创建指针类型
    pub fn get_pointer_type(pointee_type: TypeRef, space: MemorySpace) -> TypeRef {
        Rc::new(RefCell::new(Type::new(TypeKind::Pointer(pointee_type, space))))
    }
    
    /// 创建函数类型
    pub fn get_function_type(return_type: TypeRef, param_types: Vec<TypeRef>) -> TypeRef {
        Rc::new(RefCell::new(Type::new(TypeKind::Function(return_type, param_types))))
    }
    
    /// 创建空类型
    pub fn get_void_type() -> TypeRef {
        Rc::new(RefCell::new(Type::new(TypeKind::Void)))
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TypeKind::Int8 => write!(f, "i8"),
            TypeKind::Uint8 => write!(f, "u8"),
            TypeKind::Int16 => write!(f, "i16"),
            TypeKind::Uint16 => write!(f, "u16"),
            TypeKind::Int32 => write!(f, "i32"),
            TypeKind::Uint32 => write!(f, "u32"),
            TypeKind::Bit8 => write!(f, "b8"),
            TypeKind::Bit16 => write!(f, "b16"),
            TypeKind::Bit32 => write!(f, "b32"),
            TypeKind::Vector(elem_type, length) => {
                write!(f, "<{} x {}>", elem_type.borrow(), length)
            },
            TypeKind::Predicate(length) => {
                write!(f, "<pred {}>", length)
            },
            TypeKind::Void => write!(f, "void"),
            TypeKind::Pointer(pointee_type, space) => {
                write!(f, "{}* {}", pointee_type.borrow(), space)
            },
            TypeKind::Function(return_type, param_types) => {
                write!(f, "{} (", return_type.borrow())?;
                for (i, param_type) in param_types.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param_type.borrow())?;
                }
                write!(f, ")")
            },
        }
    }
}

/// 类型工具
pub struct TypeUtils;

impl TypeUtils {
    /// 解析类型字符串
    pub fn parse_type(type_str: &str) -> Result<TypeRef, String> {
        // 简单实现，实际应该使用解析器
        match type_str {
            "i8" => Ok(Type::get_int_type(TypeKind::Int8)),
            "u8" => Ok(Type::get_int_type(TypeKind::Uint8)),
            "i16" => Ok(Type::get_int_type(TypeKind::Int16)),
            "u16" => Ok(Type::get_int_type(TypeKind::Uint16)),
            "i32" => Ok(Type::get_int_type(TypeKind::Int32)),
            "u32" => Ok(Type::get_int_type(TypeKind::Uint32)),
            "b8" => Ok(Type::get_bit_type(TypeKind::Bit8)),
            "b16" => Ok(Type::get_bit_type(TypeKind::Bit16)),
            "b32" => Ok(Type::get_bit_type(TypeKind::Bit32)),
            "void" => Ok(Type::get_void_type()),
            _ => Err(format!("无法解析类型: {}", type_str)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_int_types() {
        let i8_type = Type::get_int_type(TypeKind::Int8);
        assert_eq!(i8_type.borrow().get_bit_width(), 8);
        assert!(i8_type.borrow().is_scalar());
        assert!(!i8_type.borrow().is_vector());
        assert_eq!(i8_type.borrow().to_string(), "i8");
    }
    
    #[test]
    fn test_vector_type() {
        let elem_type = Type::get_int_type(TypeKind::Int32);
        let vec_type = Type::get_vector_type(elem_type, 4);
        assert_eq!(vec_type.borrow().get_bit_width(), 128); // 4 * 32
        assert!(vec_type.borrow().is_vector());
        assert_eq!(vec_type.borrow().to_string(), "<i32 x 4>");
    }
} 