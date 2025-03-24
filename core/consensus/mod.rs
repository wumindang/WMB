pub mod pow;
pub mod pos;

pub use pow::{FullNode, mine_block};
pub use pos::{ProvincialNodeManager, validate_transactions};
// 中文注释：验证区块的函数，用于共识机制
pub fn validate_block(_block: &super::blockchain::Block) -> bool {
    // 中文注释：示例实现，始终返回 true，后续可添加具体共识逻辑
    true
}