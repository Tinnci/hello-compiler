// PassManager skeleton
// 负责注册、拓扑排序并依次运行各个优化 Pass。
// 后续高级功能（依赖解析、重复执行等）将在该基础上迭代。

use crate::ir::ModuleRef;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::time::{Duration, Instant};

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

/// Pass 执行统计信息
#[derive(Debug, Clone)]
pub struct PassStatistics {
    /// Pass 名称
    pub name: String,
    /// 执行时间
    pub duration: Duration,
    /// 是否被跳过
    pub skipped: bool,
    /// 跳过原因（如果被跳过）
    pub skip_reason: Option<String>,
}

impl fmt::Display for PassStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.skipped {
            write!(
                f,
                "{}: 已跳过 ({})",
                self.name,
                self.skip_reason.as_deref().unwrap_or("未知原因")
            )
        } else {
            write!(
                f,
                "{}: 执行时间 {:.2}ms",
                self.name,
                self.duration.as_secs_f64() * 1000.0
            )
        }
    }
}

/// Pass 分组
#[derive(Debug)]
pub struct PassGroup {
    /// 分组名称
    name: String,
    /// 分组描述
    description: String,
    /// 分组中的 Pass 名称
    passes: Vec<String>,
}

impl PassGroup {
    /// 创建新的 Pass 分组
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            passes: Vec::new(),
        }
    }

    /// 添加 Pass 到分组
    pub fn add_pass(&mut self, pass_name: &str) {
        self.passes.push(pass_name.to_string());
    }

    /// 获取分组名称
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// 获取分组描述
    pub fn get_description(&self) -> &str {
        &self.description
    }

    /// 获取分组中的 Pass 名称
    pub fn get_passes(&self) -> &[String] {
        &self.passes
    }
}

/// 所有优化 Pass 需实现的统一接口
pub trait Pass {
    /// Pass 唯一名称（建议使用 "namespace::PassName" 格式）
    fn name(&self) -> &'static str;

    /// 指定依赖的其它 Pass 名称（可为空）
    fn dependencies(&self) -> Vec<&'static str> {
        Vec::new()
    }

    /// 检查是否应该运行此 Pass
    /// 
    /// 默认总是返回 true。子类可以重写此方法以实现条件执行。
    /// 例如，可以基于命令行参数、模块特性或其他条件决定是否执行。
    fn should_run(&self, _module: &ModuleRef) -> bool {
        true
    }

    /// 获取 Pass 描述
    fn description(&self) -> &'static str {
        "No description provided"
    }

    /// 运行 Pass
    fn run(&self, module: &ModuleRef);
}

/// PassManager：负责注册、依赖解析、拓扑排序并依次运行各个优化 Pass
pub struct PassManager {
    registered: HashMap<String, Box<dyn Pass>>,
    pipeline: Vec<String>,
    groups: HashMap<String, PassGroup>,
    /// 是否收集执行统计信息
    collect_stats: bool,
    /// 最近一次执行的统计信息
    last_run_stats: Vec<PassStatistics>,
    /// 是否启用详细日志
    verbose: bool,
}

impl PassManager {
    /// 创建空的 PassManager
    pub fn new() -> Self {
        Self {
            registered: HashMap::new(),
            pipeline: Vec::new(),
            groups: HashMap::new(),
            collect_stats: false,
            last_run_stats: Vec::new(),
            verbose: false,
        }
    }

    /// 启用统计信息收集
    pub fn enable_statistics(&mut self) -> &mut Self {
        self.collect_stats = true;
        self
    }

    /// 启用详细日志
    pub fn enable_verbose(&mut self) -> &mut Self {
        self.verbose = true;
        self
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

    /// 创建新的 Pass 分组
    pub fn create_group(&mut self, name: &str, description: &str) -> &mut Self {
        let group = PassGroup::new(name, description);
        self.groups.insert(name.to_string(), group);
        self
    }

    /// 向分组添加 Pass
    pub fn add_pass_to_group(&mut self, group_name: &str, pass_name: &str) -> Result<&mut Self, PassError> {
        if !self.groups.contains_key(group_name) {
            return Err(PassError::NotRegistered(format!("分组 '{}'", group_name)));
        }
        
        if let Some(group) = self.groups.get_mut(group_name) {
            group.add_pass(pass_name);
        }
        
        Ok(self)
    }

    /// 将整个分组添加到执行流水线
    pub fn add_group_to_pipeline(&mut self, group_name: &str) -> Result<&mut Self, PassError> {
        if let Some(group) = self.groups.get(group_name) {
            for pass in group.get_passes() {
                self.pipeline.push(pass.clone());
            }
            Ok(self)
        } else {
            Err(PassError::NotRegistered(format!("分组 '{}'", group_name)))
        }
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
    pub fn run(&mut self, module: &ModuleRef) -> Result<(), PassError> {
        // 拓扑排序
        let sorted_pipeline = self.topological_sort()?;

        // 清空上次运行的统计信息
        if self.collect_stats {
            self.last_run_stats.clear();
        }

        // 按顺序执行
        for name in &sorted_pipeline {
            if let Some(pass) = self.registered.get(name) {
                // 检查是否应该运行此 Pass
                let should_run = pass.should_run(module);
                
                if self.verbose {
                    if should_run {
                        println!("正在运行 Pass: {} ({})", pass.name(), pass.description());
                    } else {
                        println!("跳过 Pass: {} ({})", pass.name(), pass.description());
                    }
                }
                
                // 收集统计信息
                if self.collect_stats {
                    if should_run {
                        let start = Instant::now();
                        pass.run(module);
                        let duration = start.elapsed();
                        
                        let stats = PassStatistics {
                            name: name.clone(),
                            duration,
                            skipped: false,
                            skip_reason: None,
                        };
                        
                        self.last_run_stats.push(stats);
                        
                        if self.verbose {
                            println!("  完成: {:.2}ms", duration.as_secs_f64() * 1000.0);
                        }
                    } else {
                        let stats = PassStatistics {
                            name: name.clone(),
                            duration: Duration::from_secs(0),
                            skipped: true,
                            skip_reason: Some("条件不满足".to_string()),
                        };
                        
                        self.last_run_stats.push(stats);
                    }
                } else if should_run {
                    pass.run(module);
                }
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

    /// 获取最近一次运行的统计信息
    pub fn get_statistics(&self) -> &[PassStatistics] {
        &self.last_run_stats
    }

    /// 打印最近一次运行的统计信息
    pub fn print_statistics(&self) {
        if self.last_run_stats.is_empty() {
            println!("没有可用的统计信息。请先运行 PassManager 并启用统计功能。");
            return;
        }
        
        println!("Pass 执行统计:");
        println!("----------------------------------------");
        
        let mut total_time = Duration::from_secs(0);
        let mut executed_count = 0;
        let mut skipped_count = 0;
        
        for stats in &self.last_run_stats {
            println!("  {}", stats);
            
            if stats.skipped {
                skipped_count += 1;
            } else {
                executed_count += 1;
                total_time += stats.duration;
            }
        }
        
        println!("----------------------------------------");
        println!(
            "总计: 执行 {} 个 Pass, 跳过 {} 个, 总时间: {:.2}ms",
            executed_count,
            skipped_count,
            total_time.as_secs_f64() * 1000.0
        );
    }

    /// 清除 pipeline
    pub fn clear_pipeline(&mut self) -> &mut Self {
        self.pipeline.clear();
        self
    }
}

impl Default for PassManager {
    fn default() -> Self {
        Self::new()
    }
}
