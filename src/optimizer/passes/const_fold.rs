use crate::ir::{FunctionRef, Instruction, InstructionRef, ModuleRef, Value};
use crate::optimizer::pass_manager::Pass;

/// 常量折叠 Pass
///
/// 在编译时计算常量表达式，减少运行时计算。
/// 例如，将 `add i32 5, 3` 替换为 `i32 8`。
pub struct ConstantFoldingPass;

impl ConstantFoldingPass {
    /// 创建新的常量折叠 Pass
    pub fn new() -> Self {
        Self
    }

    /// 处理单个函数
    fn process_function(&self, function: &FunctionRef) {
        let mut changed = true;
        
        // 重复处理直到没有更多的折叠机会
        while changed {
            changed = false;
            
            // 遍历所有基本块和指令
            for bb in function.get_basic_blocks() {
                let bb_ref = bb.borrow();
                let instructions = bb_ref.get_instructions().to_vec();
                
                // 遍历指令
                for instr in &instructions {
                    // 尝试折叠这条指令
                    if self.try_fold_instruction(instr) {
                        changed = true;
                    }
                }
            }
        }
    }

    /// 尝试折叠单条指令
    fn try_fold_instruction(&self, instr: &InstructionRef) -> bool {
        let instr_ref = instr.borrow();
        
        // 只处理二元运算指令
        match instr_ref.get_opcode().as_str() {
            "add" | "sub" | "mul" | "udiv" | "sdiv" | "urem" | "srem" |
            "shl" | "lshr" | "ashr" | "and" | "or" | "xor" => {
                // 确保有两个操作数
                if instr_ref.get_operand_count() != 2 {
                    return false;
                }
                
                // 获取操作数
                let lhs = instr_ref.get_operand(0);
                let rhs = instr_ref.get_operand(1);
                
                // 检查两个操作数是否都是常量
                if let (Value::Constant(lhs_val), Value::Constant(rhs_val)) = (&lhs, &rhs) {
                    // 根据操作码计算结果
                    if let Some(result) = self.compute_constant_expression(
                        instr_ref.get_opcode().as_str(),
                        lhs_val,
                        rhs_val,
                    ) {
                        // 替换指令为常量
                        drop(instr_ref); // 释放借用
                        let mut instr_mut = instr.borrow_mut();
                        instr_mut.replace_with_constant(result);
                        return true;
                    }
                }
            }
            // 处理一元操作
            "fneg" => {
                if instr_ref.get_operand_count() != 1 {
                    return false;
                }
                
                let operand = instr_ref.get_operand(0);
                
                if let Value::Constant(val) = &operand {
                    if let Some(result) = self.compute_unary_expression(
                        instr_ref.get_opcode().as_str(),
                        val,
                    ) {
                        // 替换指令为常量
                        drop(instr_ref); // 释放借用
                        let mut instr_mut = instr.borrow_mut();
                        instr_mut.replace_with_constant(result);
                        return true;
                    }
                }
            }
            // 其他指令类型
            _ => {}
        }
        
        false
    }

    /// 计算二元常量表达式
    fn compute_constant_expression(&self, opcode: &str, lhs: &str, rhs: &str) -> Option<String> {
        // 尝试将操作数解析为整数
        let lhs_int = lhs.parse::<i64>().ok()?;
        let rhs_int = rhs.parse::<i64>().ok()?;
        
        // 根据操作码计算结果
        let result = match opcode {
            "add" => lhs_int.checked_add(rhs_int)?,
            "sub" => lhs_int.checked_sub(rhs_int)?,
            "mul" => lhs_int.checked_mul(rhs_int)?,
            "sdiv" => {
                if rhs_int == 0 {
                    return None; // 避免除零错误
                }
                lhs_int.checked_div(rhs_int)?
            }
            "udiv" => {
                if rhs_int == 0 {
                    return None; // 避免除零错误
                }
                (lhs_int as u64).checked_div(rhs_int as u64).map(|r| r as i64)?
            }
            "srem" => {
                if rhs_int == 0 {
                    return None; // 避免除零错误
                }
                lhs_int.checked_rem(rhs_int)?
            }
            "urem" => {
                if rhs_int == 0 {
                    return None; // 避免除零错误
                }
                (lhs_int as u64).checked_rem(rhs_int as u64).map(|r| r as i64)?
            }
            "shl" => {
                if rhs_int < 0 || rhs_int >= 64 {
                    return None; // 无效的移位量
                }
                lhs_int.checked_shl(rhs_int as u32)?
            }
            "lshr" => {
                if rhs_int < 0 || rhs_int >= 64 {
                    return None; // 无效的移位量
                }
                (lhs_int as u64).checked_shr(rhs_int as u32).map(|r| r as i64)?
            }
            "ashr" => {
                if rhs_int < 0 || rhs_int >= 64 {
                    return None; // 无效的移位量
                }
                lhs_int.checked_shr(rhs_int as u32)?
            }
            "and" => lhs_int & rhs_int,
            "or" => lhs_int | rhs_int,
            "xor" => lhs_int ^ rhs_int,
            _ => return None,
        };
        
        // 将结果转换为字符串
        Some(result.to_string())
    }

    /// 计算一元常量表达式
    fn compute_unary_expression(&self, opcode: &str, val: &str) -> Option<String> {
        match opcode {
            "fneg" => {
                // 尝试解析为浮点数
                if let Ok(f) = val.parse::<f64>() {
                    return Some((-f).to_string());
                }
            }
            _ => {}
        }
        
        None
    }
}

impl Pass for ConstantFoldingPass {
    fn name(&self) -> &'static str {
        "optimizer::ConstantFoldingPass"
    }

    fn description(&self) -> &'static str {
        "在编译时计算常量表达式，减少运行时计算"
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
    fn test_constant_folding() {
        // 创建测试模块
        let module = Module::new("test_module");
        
        // 运行常量折叠 Pass
        let folder = ConstantFoldingPass::new();
        folder.run(&module);
        
        // 由于我们没有实际的指令，这里只是验证 Pass 能够运行
        // 实际的功能测试在集成测试中进行
    }
} 