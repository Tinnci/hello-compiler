// PassManager skeleton
// 负责注册、拓扑排序并依次运行各个优化 Pass。
// 后续高级功能（依赖解析、重复执行等）将在该基础上迭代。

use crate::ir::ModuleRef;
use std::collections::HashMap;

/// 所有优化 Pass 需实现的统一接口
pub trait Pass {
    /// Pass 唯一名称（建议使用 "namespace::PassName" 格式）
    fn name(&self) -> &'static str;

    /// 指定依赖的其它 Pass 名称（可为空）
    fn dependencies(&self) -> Vec<&'static str> {
        Vec::new()
    }

    /// 运行 Pass
    fn run(&self, module: &ModuleRef);
}

/// 简易 PassManager：按注册顺序执行，不做复杂依赖解析
pub struct PassManager {
    registered: HashMap<&'static str, Box<dyn Pass>>,
    pipeline: Vec<&'static str>,
}

impl PassManager {
    /// 创建空的 PassManager
    pub fn new() -> Self {
        Self {
            registered: HashMap::new(),
            pipeline: Vec::new(),
        }
    }

    /// 注册一个 Pass
    pub fn register_pass<P: Pass + 'static>(&mut self, pass: P) {
        let name = pass.name();
        self.registered.insert(name, Box::new(pass));
    }

    /// 将 Pass 加入执行流水线
    pub fn add_to_pipeline(&mut self, pass_name: &'static str) {
        self.pipeline.push(pass_name);
    }

    /// 运行 pipeline 上的 Pass
    pub fn run(&self, module: &ModuleRef) {
        for &name in &self.pipeline {
            match self.registered.get(name) {
                Some(pass) => pass.run(module),
                None => eprintln!("[PassManager] 未找到 Pass '{}', 跳过", name),
            }
        }
    }
}

impl Default for PassManager {
    fn default() -> Self {
        Self::new()
    }
}
