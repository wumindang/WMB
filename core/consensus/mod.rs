/// 共识模块，定义五民币系统的 PoW 和 PoS 共识机制
pub mod pow;
pub mod pos;

pub use pow::{FullNode, mine_block, validate_pow};
pub use pos::{ProvincialNode, ProvincialNodeManager, validate_transactions};