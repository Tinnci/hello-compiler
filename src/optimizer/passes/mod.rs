pub mod ssa_renumber;
pub mod dce;
pub mod const_fold;
pub mod cse;

// 重新导出已实现的 Pass
pub use ssa_renumber::SSARenumberPass;
pub use dce::DeadCodeEliminationPass;
pub use const_fold::ConstantFoldingPass;
pub use cse::CommonSubexpressionEliminationPass;
