/// 五民币核心库，定义区块链基础功能
pub mod blockchain;
pub mod crypto;
pub mod consensus;
pub mod network;
pub mod nodes;

pub use crypto::hash::sha256_hash;
pub use crypto::signature::{generate_keypair, sign, verify};
pub use consensus::pow::{FullNode, mine_block};
pub use consensus::pos::{ProvincialNodeManager, validate_transactions};
pub use network::Network;
pub use nodes::{SuperNodeInfo, NationalAuthorityInfo};  // 暂时只导出超级节点