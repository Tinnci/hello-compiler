use std::sync::atomic::{AtomicUsize, Ordering};

use std::cell::RefCell;
use std::rc::Rc;
use vil::ir::ModuleRef;
use vil::ir::module::Module;
use vil::optimizer::pass_manager::{Pass, PassManager};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

struct DummyPass;

impl Pass for DummyPass {
    fn name(&self) -> &'static str {
        "test::DummyPass"
    }

    fn run(&self, _module: &ModuleRef) {
        COUNTER.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn test_pass_manager_exec_order() {
    // 创建 PassManager 并注册 DummyPass
    let mut pm = PassManager::new();
    pm.register_pass(DummyPass);
    pm.add_to_pipeline("test::DummyPass");

    // 构造一个空的 IR 模块
    let module = Rc::new(RefCell::new(Module::new("dummy".to_string())));

    // 运行 PassManager
    pm.run(&module).expect("PassManager 执行失败");

    // 断言 DummyPass 的 run 被调用一次
    assert_eq!(COUNTER.load(Ordering::SeqCst), 1);
}
