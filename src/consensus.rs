// 文件路径: src/consensus.rs
// 作用: 定义共识算法（简单 PoW）

use crate::blockchain::Block; // 引入区块结构
use sha3::{Digest, Keccak256}; // Keccak256 哈希算法

// 共识管理
pub struct Consensus;

impl Consensus {
    // 创建新共识实例
    pub fn new() -> Self {
        Consensus
    }

    // 工作量证明（PoW）
    pub fn proof_of_work(&self, block: &mut Block, difficulty: u32) {
        let target = "0".repeat(difficulty as usize); // 难度目标（前导零）
        let mut nonce = 0; // 初始 nonce 值

        loop {
            let input = format!(
                "{}{}{}{:?}{}",
                block.index, block.previous_hash, block.timestamp, block.transactions, nonce
            ); // 拼接区块数据和 nonce
            let mut hasher = Keccak256::new(); // 创建哈希器
            hasher.update(&input); // 输入数据
            let hash = hex::encode(hasher.finalize()); // 计算哈希

            if hash.starts_with(&target) { // 检查是否满足难度
                block.hash = hash; // 更新区块哈希
                break;
            }
            nonce += 1; // 增加 nonce 重试
        }
    }
}