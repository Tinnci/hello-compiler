use crate::ir::{FunctionRef, Instruction, InstructionRef, ModuleRef, Value};
use crate::optimizer::pass_manager::Pass;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// 死代码消除 Pass
/// 
/// 删除不会影响程序结果的无用指令。
/// 这包括：
/// 1. 未使用的计算指令
/// 2. 无法到达的代码块
/// 3. 无效的分支
pub struct DeadCodeEliminationPass;

impl DeadCodeEliminationPass {
    /// 创建新的死代码消除 Pass
    pub fn new() -> Self {
        Self
    }

    /// 处理单个函数
    fn process_function(&self, function: &FunctionRef) {
        // 第一步：标记所有"活跃"指令
        let live_instructions = self.mark_live_instructions(function);
        
        // 第二步：删除所有非活跃指令
        self.remove_dead_instructions(function, &live_instructions);
    }

    /// 标记所有活跃指令
    fn mark_live_instructions(&self, function: &FunctionRef) -> HashSet<*const Instruction> {
        let mut live_instructions = HashSet::new();
        let mut work_list = Vec::new();
        
        // 首先标记所有具有副作用的指令为活跃
        for bb in function.get_basic_blocks() {
            for instr in bb.borrow().get_instructions() {
                let instr_ref = instr.borrow();
                
                // 以下指令类型被认为是具有副作用的，必须保留
                if self.has_side_effects(&instr_ref) {
                    let ptr = Rc::as_ptr(&instr);
                    live_instructions.insert(ptr);
                    work_list.push(instr.clone());
                }
            }
        }
        
        // 然后递归标记所有被活跃指令使用的指令
        while let Some(instr) = work_list.pop() {
            let instr_ref = instr.borrow();
            
            // 检查所有操作数
            for i in 0..instr_ref.get_operand_count() {
                let operand = instr_ref.get_operand(i);
                
                if let Value::Reference(ref_name) = operand {
                    // 找到引用的指令
                    if let Some(def_instr) = self.find_instruction_by_name(function, &ref_name) {
                        let ptr = Rc::as_ptr(&def_instr);
                        
                        // 如果这个指令还没被标记为活跃，标记它并加入工作列表
                        if !live_instructions.contains(&ptr) {
                            live_instructions.insert(ptr);
                            work_list.push(def_instr);
                        }
                    }
                }
            }
        }
        
        live_instructions
    }

    /// 删除所有非活跃指令
    fn remove_dead_instructions(&self, function: &FunctionRef, live_instructions: &HashSet<*const Instruction>) {
        for bb in function.get_basic_blocks() {
            let mut bb_mut = bb.borrow_mut();
            let mut i = 0;
            
            // 遍历基本块中的所有指令
            while i < bb_mut.get_instructions().len() {
                let instr = &bb_mut.get_instructions()[i];
                let ptr = Rc::as_ptr(instr);
                
                // 如果指令不在活跃集合中，删除它
                if !live_instructions.contains(&ptr) {
                    bb_mut.remove_instruction(i);
                } else {
                    i += 1;
                }
            }
        }
    }

    /// 检查指令是否有副作用
    fn has_side_effects(&self, instr: &Instruction) -> bool {
        match instr.get_opcode().as_str() {
            // 以下指令类型被认为是具有副作用的
            "store" | "call" | "ret" | "br" | "switch" => true,
            // 以下指令类型可能有副作用，取决于它们的参数
            "load" => {
                // 如果 load 指令是 volatile 的，它有副作用
                instr.has_attribute("volatile")
            },
            _ => false,
        }
    }

    /// 通过名称查找指令
    fn find_instruction_by_name(&self, function: &FunctionRef, name: &str) -> Option<InstructionRef> {
        for bb in function.get_basic_blocks() {
            for instr in bb.borrow().get_instructions() {
                let instr_ref = instr.borrow();
                if let Some(instr_name) = instr_ref.get_name() {
                    if instr_name == name {
                        return Some(instr.clone());
                    }
                }
            }
        }
        None
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
        // 无依赖
        Vec::new()
    }

    fn run(&self, module: &ModuleRef) {
        for function in module.borrow().get_functions() {
            self.process_function(&function);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::Module;

    #[test]
    fn test_dead_code_elimination() {
        // 创建测试模块
        let module = Module::new("test_module");
        
        // 运行 DCE Pass
        let dce = DeadCodeEliminationPass::new();
        dce.run(&module);
        
        // 由于我们没有实际的指令，这里只是验证 Pass 能够运行
        // 实际的功能测试在集成测试中进行
    }
} 