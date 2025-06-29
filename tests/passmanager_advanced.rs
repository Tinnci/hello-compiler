use vil::ir::Module;
use vil::optimizer::pass_manager::PassManager;
use vil::optimizer::passes::{
    SSARenumberPass, DeadCodeEliminationPass, ConstantFoldingPass, CommonSubexpressionEliminationPass
};
use std::rc::Rc;
use std::cell::RefCell;

type ModuleRef = Rc<RefCell<vil::ir::Module>>;

fn new_test_module() -> ModuleRef {
    Rc::new(RefCell::new(Module::new("test_module".to_string())))
}

#[test]
fn test_pass_manager_statistics() {
    // 创建测试模块
    let module = new_test_module();
    
    // 创建 PassManager 并启用统计
    let mut pm = PassManager::new();
    pm.enable_statistics();
    
    // 注册 Pass
    pm.register_pass(SSARenumberPass::new());
    pm.register_pass(DeadCodeEliminationPass::new());
    
    // 添加到执行流水线
    pm.add_to_pipeline("optimizer::SSARenumberPass");
    pm.add_to_pipeline("optimizer::DeadCodeEliminationPass");
    
    // 运行优化
    pm.run(&module).expect("优化过程中出错");
    
    // 验证统计信息
    let stats = pm.get_statistics();
    assert_eq!(stats.len(), 2);
    assert_eq!(stats[0].name, "optimizer::SSARenumberPass");
    assert_eq!(stats[1].name, "optimizer::DeadCodeEliminationPass");
}

#[test]
fn test_pass_groups() {
    // 创建测试模块
    let module = new_test_module();
    
    // 创建 PassManager
    let mut pm = PassManager::new();
    pm.enable_statistics();
    
    // 注册 Pass
    pm.register_pass(SSARenumberPass::new());
    pm.register_pass(DeadCodeEliminationPass::new());
    pm.register_pass(ConstantFoldingPass::new());
    pm.register_pass(CommonSubexpressionEliminationPass::new());
    
    // 创建分组
    pm.create_group("基础优化", "基本的代码清理优化");
    pm.create_group("高级优化", "更复杂的优化");
    
    // 添加 Pass 到分组
    pm.add_pass_to_group("基础优化", "optimizer::SSARenumberPass").unwrap();
    pm.add_pass_to_group("基础优化", "optimizer::DeadCodeEliminationPass").unwrap();
    pm.add_pass_to_group("高级优化", "optimizer::ConstantFoldingPass").unwrap();
    pm.add_pass_to_group("高级优化", "optimizer::CommonSubexpressionEliminationPass").unwrap();
    
    // 将分组添加到执行流水线
    pm.add_group_to_pipeline("基础优化").unwrap();
    
    // 运行优化
    pm.run(&module).expect("优化过程中出错");
    
    // 验证执行的 Pass
    let stats = pm.get_statistics();
    assert_eq!(stats.len(), 2);  // 只有基础优化组的两个 Pass 被执行
}

#[test]
fn test_conditional_execution() {
    // 创建测试模块
    let module = new_test_module();
    
    // 创建两个条件 Pass，一个会运行，一个不会
    struct ConditionalPassAlwaysRun;
    impl vil::optimizer::Pass for ConditionalPassAlwaysRun {
        fn name(&self) -> &'static str { "test::ConditionalPassAlwaysRun" }
        fn description(&self) -> &'static str { "测试条件执行的 Pass (总是运行)" }
        fn should_run(&self, _module: &vil::ir::ModuleRef) -> bool { true }
        fn run(&self, _module: &vil::ir::ModuleRef) { /* 空实现 */ }
    }

    struct ConditionalPassNeverRun;
    impl vil::optimizer::Pass for ConditionalPassNeverRun {
        fn name(&self) -> &'static str { "test::ConditionalPassNeverRun" }
        fn description(&self) -> &'static str { "测试条件执行的 Pass (永不运行)" }
        fn should_run(&self, _module: &vil::ir::ModuleRef) -> bool { false }
        fn run(&self, _module: &vil::ir::ModuleRef) { /* 空实现 */ }
    }

    let mut pm = PassManager::new();
    pm.enable_statistics();

    // 注册两个条件 Pass
    pm.register_pass(ConditionalPassAlwaysRun);
    pm.register_pass(ConditionalPassNeverRun);

    // 将两个 Pass 都添加到执行流水线
    pm.add_to_pipeline("test::ConditionalPassAlwaysRun");
    pm.add_to_pipeline("test::ConditionalPassNeverRun");

    // 运行优化
    pm.run(&module).expect("优化过程中出错");

    // 验证统计信息
    let stats = pm.get_statistics();
    assert_eq!(stats.len(), 2);
    assert_eq!(stats[0].name, "test::ConditionalPassAlwaysRun");
    assert!(!stats[0].skipped);
    assert_eq!(stats[1].name, "test::ConditionalPassNeverRun");
    assert!(stats[1].skipped);
}

#[test]
fn test_ssa_renumber_strategies() {
    // 创建测试模块
    let module = new_test_module();
    
    // 测试不同的命名策略
    let sequential = SSARenumberPass::new()
        .with_strategy(vil::optimizer::passes::ssa_renumber::NamingStrategy::Sequential);
    let type_based = SSARenumberPass::new()
        .with_strategy(vil::optimizer::passes::ssa_renumber::NamingStrategy::TypeBased);
    let block_based = SSARenumberPass::new()
        .with_strategy(vil::optimizer::passes::ssa_renumber::NamingStrategy::BlockBased);
    
    // 创建 PassManager
    let mut pm = PassManager::new();
    
    // 注册不同策略的 Pass
    pm.register_pass(sequential);
    pm.register_pass(type_based);
    pm.register_pass(block_based);
    
    // 添加到执行流水线
    pm.add_to_pipeline("optimizer::SSARenumberPass");
    
    // 运行优化
    pm.run(&module).expect("优化过程中出错");
} 