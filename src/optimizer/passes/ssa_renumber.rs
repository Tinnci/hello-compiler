use crate::ir::ModuleRef;
use crate::ir::function::FunctionRef;
use crate::ir::basic_block::BasicBlockRef;
use crate::ir::instruction::InstructionRef;
use crate::optimizer::pass_manager::Pass;

/// SSA Renumber Pass —— 为 IR 中的每个指令重新分配 SSA 名称。
///
/// 该 Pass 遍历整个模块，为每个函数中的指令按顺序重新分配名称，
/// 确保 SSA 形式中的值命名唯一且符合标准格式。
pub struct SSARenumber;

impl SSARenumber {
    /// 处理单个函数
    fn process_function(&self, function: &FunctionRef) {
        let mut counter = 0;
        let function_borrowed = function.borrow();
        
        // 获取所有基本块
        let basic_blocks: Vec<BasicBlockRef> = function_borrowed.get_basic_blocks().to_vec();
        
        // 处理每个基本块
        for bb in basic_blocks {
            self.process_basic_block(&bb, &mut counter);
        }
    }
    
    /// 处理单个基本块
    fn process_basic_block(&self, bb: &BasicBlockRef, counter: &mut usize) {
        let bb_borrowed = bb.borrow();
        
        // 获取基本块中的所有指令
        let instructions = bb_borrowed.get_instructions();
        
        // 处理每条指令
        for instr in instructions {
            self.process_instruction(instr, counter);
        }
    }
    
    /// 处理单条指令
    fn process_instruction(&self, instruction: &InstructionRef, counter: &mut usize) {
        let mut instr_borrowed = instruction.borrow_mut();
        
        // 只为有结果的指令重命名（有些指令如 store、br 等没有结果值）
        if instr_borrowed.has_result() {
            // 生成新的 SSA 名称
            let new_name = format!("%{}", counter);
            *counter += 1;
            
            // 设置新名称
            instr_borrowed.set_name(new_name);
        }
    }
}

impl Pass for SSARenumber {
    fn name(&self) -> &'static str {
        "optimizer::SSARenumber"
    }

    fn run(&self, module: &ModuleRef) {
        let module_borrowed = module.borrow();
        
        // 获取模块中的所有函数
        let functions = module_borrowed.get_functions();
        
        // 处理每个函数
        for function in functions {
            self.process_function(&function);
        }
    }
}
