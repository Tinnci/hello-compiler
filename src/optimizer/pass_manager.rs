// PassManager skeleton
// 负责注册、拓扑排序并依次运行各个优化 Pass。
// 后续高级功能（依赖解析、重复执行等）将在该基础上迭代。

use crate::ir::ModuleRef;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

/// Pass 执行错误
#[derive(Debug)]
pub enum PassError {
    /// Pass 未注册
    NotRegistered(String),
    /// 依赖循环
    CircularDependency(Vec<String>),
    /// 缺少依赖
    MissingDependency { pass: String, dependency: String },
}

impl fmt::Display for PassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PassError::NotRegistered(name) => write!(f, "Pass '{}' 未注册", name),
            PassError::CircularDependency(cycle) => {
                write!(f, "检测到 Pass 依赖循环: {}", cycle.join(" -> "))
            }
            PassError::MissingDependency { pass, dependency } => {
                write!(f, "Pass '{}' 依赖未注册的 Pass '{}'", pass, dependency)
            }
        }
    }
}

impl std::error::Error for PassError {}

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

/// PassManager：负责注册、依赖解析、拓扑排序并依次运行各个优化 Pass
pub struct PassManager {
    registered: HashMap<String, Box<dyn Pass>>,
    pipeline: Vec<String>,
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
        self.registered.insert(name.to_string(), Box::new(pass));
    }

    /// 将 Pass 加入执行流水线
    pub fn add_to_pipeline(&mut self, pass_name: &'static str) {
        self.pipeline.push(pass_name.to_string());
    }

    /// 检查所有依赖是否已注册
    fn check_dependencies(&self) -> Result<(), PassError> {
        for (name, pass) in &self.registered {
            for dep in pass.dependencies() {
                if !self.registered.contains_key(dep) {
                    return Err(PassError::MissingDependency {
                        pass: name.clone(),
                        dependency: dep.to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    /// 对 pipeline 中的 Pass 进行拓扑排序，确保依赖先执行
    fn topological_sort(&self) -> Result<Vec<String>, PassError> {
        // 检查依赖是否都已注册
        self.check_dependencies()?;

        // 构建依赖图
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();

        // 初始化
        for name in &self.pipeline {
            if !self.registered.contains_key(name) {
                return Err(PassError::NotRegistered(name.clone()));
            }
            graph.insert(name.clone(), Vec::new());
            in_degree.insert(name.clone(), 0);
        }

        // 构建图和入度
        for name in &self.pipeline {
            let pass = self.registered.get(name).unwrap();
            for dep in pass.dependencies() {
                // 只考虑 pipeline 中的依赖
                let dep_str = dep.to_string();
                if self.pipeline.contains(&dep_str) {
                    graph.get_mut(&dep_str).unwrap().push(name.clone());
                    *in_degree.get_mut(name).unwrap() += 1;
                }
            }
        }

        // Kahn 算法进行拓扑排序
        let mut sorted = Vec::new();
        let mut queue = VecDeque::new();

        // 将所有入度为 0 的节点加入队列
        for name in &self.pipeline {
            if in_degree[name] == 0 {
                queue.push_back(name.clone());
            }
        }

        while let Some(name) = queue.pop_front() {
            sorted.push(name.clone());

            for next in &graph[&name] {
                let in_deg = in_degree.get_mut(next).unwrap();
                *in_deg -= 1;
                if *in_deg == 0 {
                    queue.push_back(next.clone());
                }
            }
        }

        // 检查是否有环
        if sorted.len() != self.pipeline.len() {
            // 找出循环依赖
            let mut cycle = Vec::new();
            let mut visited = HashSet::new();
            let mut stack = Vec::new();

            // 找出一个未处理的节点
            for name in &self.pipeline {
                if !sorted.contains(name) {
                    self.find_cycle(name.clone(), &graph, &mut visited, &mut stack, &mut cycle);
                    break;
                }
            }

            return Err(PassError::CircularDependency(cycle));
        }

        Ok(sorted)
    }

    /// DFS 查找依赖环
    #[allow(clippy::only_used_in_recursion)]
    fn find_cycle(
        self: &PassManager,
        node: String,
        graph: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        stack: &mut Vec<String>,
        cycle: &mut Vec<String>,
    ) -> bool {
        if !cycle.is_empty() {
            return true; // 已找到环
        }

        if stack.contains(&node) {
            // 找到环
            let start_idx = stack.iter().position(|x| x == &node).unwrap();
            cycle.extend(stack[start_idx..].iter().cloned());
            cycle.push(node.clone()); // 完整环
            return true;
        }

        if visited.contains(&node) {
            return false;
        }

        visited.insert(node.clone());
        stack.push(node.clone());

        for next in &graph[&node] {
            if self.find_cycle(next.clone(), graph, visited, stack, cycle) {
                return true;
            }
        }

        stack.pop();
        false
    }

    /// 运行 pipeline 上的 Pass，自动处理依赖关系
    pub fn run(&self, module: &ModuleRef) -> Result<(), PassError> {
        // 拓扑排序
        let sorted_pipeline = self.topological_sort()?;

        // 按顺序执行
        for name in &sorted_pipeline {
            if let Some(pass) = self.registered.get(name) {
                pass.run(module);
            }
        }

        Ok(())
    }

    /// 获取当前注册的所有 Pass 名称
    pub fn get_registered_passes(&self) -> Vec<String> {
        self.registered.keys().cloned().collect()
    }

    /// 获取当前 pipeline 中的 Pass 名称
    pub fn get_pipeline(&self) -> &[String] {
        &self.pipeline
    }
}

impl Default for PassManager {
    fn default() -> Self {
        Self::new()
    }
}
