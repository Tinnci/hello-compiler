// 优化器模块入口
//
// 该目录下包含 PassManager 及各类优化 Pass 的实现。

pub mod pass_manager;

// 运行优化器主入口；目前仅构造 PassManager 并执行 pipeline。
use crate::optimizer::pass_manager::PassManager;

// 引入子模块及占位 Pass
pub mod passes;
use passes::ssa_renumber::SSARenumber;

pub fn run_optimizer(module: &crate::ir::ModuleRef) {
    let mut pm = PassManager::new();

    // 注册占位 Pass，并加入 pipeline
    pm.register_pass(SSARenumber);
    pm.add_to_pipeline("optimizer::SSARenumber");

    pm.run(module);
}
