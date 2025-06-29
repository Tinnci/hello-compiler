use crate::ir::{ModuleRef, Instruction};
use crate::optimizer::pass_manager::Pass;

use std::collections::{HashSet, VecDeque};

fn has_side_effects(instr: &Instruction) -> bool {
    match instr.get_opcode().as_str() {
        "store" | "call" | "ret" | "br" | "condbr" => true,
        "load" => instr.has_attribute("volatile"),
        _ => false,
    }
}

/// 死代码消除 Pass（简化占位实现）
pub struct DeadCodeEliminationPass;

impl DeadCodeEliminationPass {
    pub fn new() -> Self {
        Self
    }
}

impl Pass for DeadCodeEliminationPass {
    fn name(&self) -> &'static str {
        "optimizer::DeadCodeEliminationPass"
    }

    fn description(&self) -> &'static str {
        "删除不会影响程序结果的无用指令"
    }

    fn dependencies(&self) -> Vec<&'static str> {
        Vec::new()
    }

    fn run(&self, module: &ModuleRef) {
        for func in module.borrow().get_functions() {
            // 第一遍：收集定义 map
            let mut def_map = std::collections::HashMap::new(); // name -> instr ptr
            for bb in func.borrow().get_basic_blocks() {
                for instr in bb.borrow().get_instructions() {
                    if let Some(name) = instr.borrow().get_name() {
                        def_map.insert(name.clone(), instr.clone());
                    }
                }
            }

            // 活跃集合
            let mut live: HashSet<*const std::cell::RefCell<crate::ir::instruction::Instruction>> = HashSet::new();
            let mut work: VecDeque<crate::ir::instruction::InstructionRef> = VecDeque::new();

            // 具有副作用的指令先入队
            for bb in func.borrow().get_basic_blocks() {
                for instr in bb.borrow().get_instructions() {
                    if has_side_effects(&instr.borrow()) {
                        let ptr = std::rc::Rc::as_ptr(instr);
                        live.insert(ptr);
                        work.push_back(instr.clone());
                    }
                }
            }

            // 向后追踪依赖
            while let Some(instr) = work.pop_front() {
                let instr_borrow = instr.borrow();
                for idx in 0..instr_borrow.get_operand_count() {
                    let op = instr_borrow.get_operand(idx);
                    let op_val = op.borrow();
                    if op_val.is_reference() {
                        let name = op_val.get_name();
                        if let Some(def_instr) = def_map.get(name) {
                            let ptr = std::rc::Rc::as_ptr(def_instr);
                            if live.insert(ptr) {
                                work.push_back(def_instr.clone());
                            }
                        }
                    }
                }
            }

            // 第二遍：删除 dead 指令
            for bb in func.borrow().get_basic_blocks() {
                // 收集要删除的指令
                let to_remove: Vec<_> = bb
                    .borrow()
                    .get_instructions()
                    .iter()
                    .filter(|instr| {
                        if !instr.borrow().has_result() { return false; }
                        let ptr = std::rc::Rc::as_ptr(*instr);
                        !live.contains(&ptr)
                    })
                    .cloned()
                    .collect();

                for instr in to_remove {
                    bb.borrow_mut().remove_instruction(&instr);
                }
            }
        }
    }
}

#[cfg(all(test, feature = "advanced_pass_tests"))]
mod tests {
    use super::*;
    use crate::ir::Module;

    #[test]
    fn test_dce_stub() {
        let module = Module::new("test_module".to_string());
        DeadCodeEliminationPass::new().run(&std::rc::Rc::new(std::cell::RefCell::new(module)));
    }
} 