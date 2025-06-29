use crate::optimizer::pass_manager::Pass;

/// SSA 命名策略
#[derive(Clone, Copy)]
pub enum NamingStrategy {
    /// %0, %1, %2 ...
    Sequential,
    /// 根据类型前缀，例如 i32_0, i32_1 ...
    TypeBased,
    /// 每个基本块内部重新从 0 计数
    BlockBased,
}

/// SSA 重命名 Pass
pub struct SSARenumberPass {
    strategy: NamingStrategy,
}

impl SSARenumberPass {
    pub fn new() -> Self {
        SSARenumberPass {
            strategy: NamingStrategy::Sequential,
        }
    }

    /// 链式接口：设置命名策略
    pub fn with_strategy(mut self, strategy: NamingStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// 核心逻辑：遍历函数并为有返回值的指令重新命名
    fn process_function(&self, func: &crate::ir::function::FunctionRef) {
        use crate::ir::instruction::InstructionModifier;

        match self.strategy {
            NamingStrategy::Sequential | NamingStrategy::TypeBased => {
                let mut counter: usize = 0;
                for bb in func.borrow().get_basic_blocks() {
                    for instr in bb.borrow().get_instructions() {
                        if instr.borrow().has_result() {
                            let new_name = match self.strategy {
                                NamingStrategy::Sequential => format!("%{}", counter),
                                NamingStrategy::TypeBased => {
                                    let ty_str = instr.borrow().get_type().borrow().to_string();
                                    format!("{}_{}", ty_str, counter)
                                }
                                _ => unreachable!(),
                            };
                            counter += 1;
                            instr.borrow_mut().set_name(new_name);
                        }
                    }
                }
            }
            NamingStrategy::BlockBased => {
                for bb in func.borrow().get_basic_blocks() {
                    let mut counter: usize = 0;
                    for instr in bb.borrow().get_instructions() {
                        if instr.borrow().has_result() {
                            let new_name = format!("%{}_{}", bb.borrow().get_name(), counter);
                            counter += 1;
                            instr.borrow_mut().set_name(new_name);
                        }
                    }
                }
            }
        }
    }
}

// 类型别名兼容旧测试
pub type SSARenumber = SSARenumberPass;

impl Pass for SSARenumberPass {
    fn name(&self) -> &'static str {
        "optimizer::SSARenumberPass"
    }

    fn description(&self) -> &'static str {
        "为 SSA 指令重新分配唯一名称"
    }

    fn dependencies(&self) -> Vec<&'static str> {
        Vec::new()
    }

    fn run(&self, module: &crate::ir::ModuleRef) {
        for func in module.borrow().get_functions() {
            self.process_function(&func);
        }
    }
}

#[cfg(all(test, feature = "advanced_pass_tests"))]
mod tests {
    use super::*;
    use crate::ir::{module::Module, types::{Type, TypeKind}, instruction::{Instruction, Opcode, InstructionModifier}, basic_block::BasicBlock, function::Function};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_basic_sequential() {
        let mut module = Module::new("m".to_string());
        let int_ty = Type::get_int_type(TypeKind::Int32);
        let func = Rc::new(RefCell::new(Function::new("f".to_string(),  Type::get_void_type(), vec![])));
        let bb = Rc::new(RefCell::new(BasicBlock::new("entry".to_string(), Some(func.clone()))));
        let inst1 = Rc::new(RefCell::new(Instruction::new(Opcode::Add, Some(Rc::new(RefCell::new(crate::ir::value::Value::new(int_ty.clone(), "".to_string())))), vec![], InstructionModifier::None)));
        bb.borrow_mut().add_instruction(inst1.clone(), bb.clone());
        func.borrow_mut().add_basic_block(bb.clone());
        module.add_function(func.clone());

        SSARenumberPass::new().run(&Rc::new(RefCell::new(module)));
        assert_eq!(inst1.borrow().get_name(), Some("%0".to_string()));
    }
}
