// 优化器模块入口
//
// 该目录下包含 PassManager 及各类优化 Pass 的实现。

pub mod pass_manager;

// 运行优化器主入口；目前仅构造 PassManager 并执行 pipeline。

// 引入子模块及占位 Pass
pub mod passes;
use passes::ssa_renumber::SSARenumberPass;

// 重新导出 pass_manager 中的 Pass trait
pub use pass_manager::Pass;

pub fn run_optimizer(module: &crate::ir::ModuleRef) {
    let mut pm = pass_manager::PassManager::new();

    // 注册占位 Pass，并加入 pipeline
    pm.register_pass(passes::ssa_renumber::SSARenumberPass::new());
    pm.add_to_pipeline("optimizer::SSARenumberPass");

    // 运行优化器，处理可能的错误
    pm.run(module).expect("优化过程中出错");
}
