// vemu-venus-compiler-rs 库入口点
//
// 这个文件是 vil 库的主入口点，导出所有公共模块和类型

// 重新导出子模块
pub mod backend;
pub mod frontend;
pub mod ir;
pub mod optimizer;

// 版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

/// 初始化库
///
/// 设置日志系统和其他全局状态
pub fn init() {
    env_logger::init();
    log::info!("Venus Intermediate Language (VIL) Compiler v{}", VERSION);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
