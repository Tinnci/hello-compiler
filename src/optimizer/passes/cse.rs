use crate::ir::ModuleRef;
use crate::optimizer::pass_manager::Pass;
use std::collections::HashMap;

use crate::ir::instruction::Instruction;

fn has_side_effects(instr: &Instruction) -> bool {
    match instr.get_opcode().as_str() {
        "store" | "call" | "ret" | "br" | "condbr" => true,
        "load" => instr.has_attribute("volatile"),
        _ => false,
    }
}

/// 公共子表达式消除 Pass（简化占位实现）
pub struct CommonSubexpressionEliminationPass;

impl CommonSubexpressionEliminationPass {
    pub fn new() -> Self { Self }
}

impl Pass for CommonSubexpressionEliminationPass {
    fn name(&self) -> &'static str {
        "optimizer::CommonSubexpressionEliminationPass"
    }

    fn description(&self) -> &'static str {
        "公共子表达式消除 (Stub)"
    }

    fn dependencies(&self) -> Vec<&'static str> {
        vec!["optimizer::ConstantFoldingPass"]
    }

    fn run(&self, module: &ModuleRef) {
        for func in module.borrow().get_functions() {
            for bb in func.borrow().get_basic_blocks() {
                let mut available: HashMap<String, String> = HashMap::new(); // sig -> name

                // 收集需要删除的指令
                let mut to_delete = Vec::new();

                for instr in bb.borrow().get_instructions() {
                    let ib = instr.borrow();
                    if ib.has_result() && !has_side_effects(&ib) {
                        // 构造签名
                        let mut sig = String::from(ib.get_opcode().as_str());
                        sig.push('(');
                        for idx in 0..ib.get_operand_count() {
                            if idx > 0 { sig.push(','); }
                            let op = ib.get_operand(idx);
                            let op_val = op.borrow();
                            sig.push_str(op_val.get_name());
                        }
                        sig.push(')');

                        if let Some(existing) = available.get(&sig) {
                            if let Some(cur_name) = ib.get_name() {
                                // 替换所有引用
                                drop(ib);
                                Self::replace_uses(&func, cur_name.as_str(), existing.as_str());
                                to_delete.push(instr.clone());
                            }
                        } else if let Some(result_name) = ib.get_name() {
                            available.insert(sig, result_name.to_string());
                        }
                    }
                }

                for instr in to_delete {
                    bb.borrow_mut().remove_instruction(&instr);
                }
            }
        }
    }
}

impl CommonSubexpressionEliminationPass {
    fn replace_uses(
        func: &crate::ir::function::FunctionRef,
        old_name: &str,
        new_name: &str,
    ) {
        for bb in func.borrow().get_basic_blocks() {
            for instr in bb.borrow().get_instructions() {
                let mut ib = instr.borrow_mut();
                for idx in 0..ib.get_operand_count() {
                    let op = ib.get_operand(idx);
                    if op.borrow().get_name() == old_name {
                        // 创建新 ValueRef 引用
                        let ty = op.borrow().get_type();
                        let new_val = crate::ir::value::Value::new(ty, new_name.to_string());
                        ib.set_operand(idx, std::rc::Rc::new(std::cell::RefCell::new(new_val)));
                    }
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
    fn test_cse_stub() {
        let module = Module::new("test_module".to_string());
        CommonSubexpressionEliminationPass::new().run(&std::rc::Rc::new(std::cell::RefCell::new(module)));
    }
} 