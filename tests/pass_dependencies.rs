use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use vil::ir::module::Module;
use vil::ir::ModuleRef;
use vil::optimizer::pass_manager::{Pass, PassManager, PassError};

// 用于记录 Pass 执行顺序
static EXECUTION_ORDER: AtomicUsize = AtomicUsize::new(0);
static PASS_A_ORDER: AtomicUsize = AtomicUsize::new(0);
static PASS_B_ORDER: AtomicUsize = AtomicUsize::new(0);
static PASS_C_ORDER: AtomicUsize = AtomicUsize::new(0);

// 重置计数器
fn reset_counters() {
    EXECUTION_ORDER.store(0, Ordering::SeqCst);
    PASS_A_ORDER.store(0, Ordering::SeqCst);
    PASS_B_ORDER.store(0, Ordering::SeqCst);
    PASS_C_ORDER.store(0, Ordering::SeqCst);
}

// Pass A: 不依赖其他 Pass
struct PassA;

impl Pass for PassA {
    fn name(&self) -> &'static str {
        "test::PassA"
    }

    fn run(&self, _module: &ModuleRef) {
        let order = EXECUTION_ORDER.fetch_add(1, Ordering::SeqCst);
        PASS_A_ORDER.store(order, Ordering::SeqCst);
    }
}

// Pass B: 依赖 Pass A
struct PassB;

impl Pass for PassB {
    fn name(&self) -> &'static str {
        "test::PassB"
    }

    fn dependencies(&self) -> Vec<&'static str> {
        vec!["test::PassA"]
    }

    fn run(&self, _module: &ModuleRef) {
        let order = EXECUTION_ORDER.fetch_add(1, Ordering::SeqCst);
        PASS_B_ORDER.store(order, Ordering::SeqCst);
    }
}

// Pass C: 依赖 Pass B
struct PassC;

impl Pass for PassC {
    fn name(&self) -> &'static str {
        "test::PassC"
    }

    fn dependencies(&self) -> Vec<&'static str> {
        vec!["test::PassB"]
    }

    fn run(&self, _module: &ModuleRef) {
        let order = EXECUTION_ORDER.fetch_add(1, Ordering::SeqCst);
        PASS_C_ORDER.store(order, Ordering::SeqCst);
    }
}

// 测试 Pass 依赖关系解析
#[test]
fn test_pass_dependencies() {
    // 重置计数器
    reset_counters();

    // 创建 PassManager
    let mut pm = PassManager::new();

    // 注册 Pass
    pm.register_pass(PassA);
    pm.register_pass(PassB);
    pm.register_pass(PassC);

    // 按逆序添加到 pipeline，依赖解析应该重排顺序
    pm.add_to_pipeline("test::PassC");
    pm.add_to_pipeline("test::PassB");
    pm.add_to_pipeline("test::PassA");

    // 创建测试模块
    let module = Rc::new(RefCell::new(Module::new("test_module".to_string())));

    // 运行 PassManager
    pm.run(&module).expect("PassManager 执行失败");

    // 验证执行顺序
    // Pass A 应该最先执行
    assert_eq!(PASS_A_ORDER.load(Ordering::SeqCst), 0);
    // Pass B 应该在 Pass A 之后执行
    assert_eq!(PASS_B_ORDER.load(Ordering::SeqCst), 1);
    // Pass C 应该在 Pass B 之后执行
    assert_eq!(PASS_C_ORDER.load(Ordering::SeqCst), 2);
}

// 测试循环依赖检测
#[test]
fn test_circular_dependency_detection() {
    // 创建带循环依赖的 Pass
    struct CircularPassA;
    impl Pass for CircularPassA {
        fn name(&self) -> &'static str {
            "test::CircularPassA"
        }
        fn dependencies(&self) -> Vec<&'static str> {
            vec!["test::CircularPassB"]
        }
        fn run(&self, _: &ModuleRef) {}
    }

    struct CircularPassB;
    impl Pass for CircularPassB {
        fn name(&self) -> &'static str {
            "test::CircularPassB"
        }
        fn dependencies(&self) -> Vec<&'static str> {
            vec!["test::CircularPassA"]
        }
        fn run(&self, _: &ModuleRef) {}
    }

    // 创建 PassManager
    let mut pm = PassManager::new();

    // 注册 Pass
    pm.register_pass(CircularPassA);
    pm.register_pass(CircularPassB);

    // 添加到 pipeline
    pm.add_to_pipeline("test::CircularPassA");
    pm.add_to_pipeline("test::CircularPassB");

    // 创建测试模块
    let module = Rc::new(RefCell::new(Module::new("test_module".to_string())));

    // 运行 PassManager，应该检测到循环依赖
    let result = pm.run(&module);
    assert!(result.is_err());
    
    if let Err(PassError::CircularDependency(cycle)) = result {
        // 验证循环依赖包含两个 Pass
        assert!(cycle.len() >= 2);
        assert!(cycle.len() <= 3);
    } else {
        panic!("预期 CircularDependency 错误");
    }
}

// 测试缺失依赖检测
#[test]
fn test_missing_dependency_detection() {
    // 创建依赖不存在 Pass 的 Pass
    struct MissingDepPass;
    impl Pass for MissingDepPass {
        fn name(&self) -> &'static str {
            "test::MissingDepPass"
        }
        fn dependencies(&self) -> Vec<&'static str> {
            vec!["test::NonExistentPass"]
        }
        fn run(&self, _: &ModuleRef) {}
    }

    // 创建 PassManager
    let mut pm = PassManager::new();

    // 注册 Pass
    pm.register_pass(MissingDepPass);

    // 添加到 pipeline
    pm.add_to_pipeline("test::MissingDepPass");

    // 创建测试模块
    let module = Rc::new(RefCell::new(Module::new("test_module".to_string())));

    // 运行 PassManager，应该检测到缺失依赖
    let result = pm.run(&module);
    assert!(result.is_err());
    
    if let Err(PassError::MissingDependency { pass, dependency }) = result {
        assert_eq!(pass, "test::MissingDepPass");
        assert_eq!(dependency, "test::NonExistentPass");
    } else {
        panic!("预期 MissingDependency 错误");
    }
} 