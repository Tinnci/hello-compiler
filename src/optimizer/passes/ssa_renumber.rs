use crate::ir::ModuleRef;
use crate::optimizer::pass_manager::Pass;

/// SSA Renumber Pass —— 仅占位，后续将实现真正的 SSA 重编号算法。
pub struct SSARenumber;

impl Pass for SSARenumber {
    fn name(&self) -> &'static str {
        "optimizer::SSARenumber"
    }

    fn run(&self, _module: &ModuleRef) {
        // TODO: 实现 SSA 重编号逻辑
        // 目前仅做占位，不修改 IR
    }
}
