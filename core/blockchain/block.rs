use crate::sha256_hash; // 从根模块导入
use crate::blockchain::transaction::Transaction; // 修正为具体路径
use serde::{Serialize, Deserialize};

/// 区块头结构体，包含区块的核心元数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    pub prev_hash: String,      // 前一区块的哈希值
    pub timestamp: i64,         // 区块创建时间戳（Unix时间）
    pub difficulty: u32,        // PoW挖矿难度
    pub nonce: u64,             // PoW随机数，用于挖矿
    pub issuance_type: String,  // 发行类型（如"创世发行"、"省储行创立发行"）
}

/// 区块结构体，包含区块头和交易列表
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,        // 区块头
    pub transactions: Vec<Transaction>, // 交易列表
    pub hash: String,               // 当前区块的哈希值
    pub is_offchain: bool,          // 是否为链下交易区块
}

impl Block {
    /// 创建一个新区块
    /// 参数：
    /// - prev_hash: 前一区块的哈希值
    /// - transactions: 当前区块包含的交易列表
    /// - issuance_type: 发行类型（记录区块的货币发行来源）
    /// - is_offchain: 是否为链下交易区块
    /// 返回：新创建的区块
    pub fn new(prev_hash: String, transactions: Vec<Transaction>, issuance_type: String, is_offchain: bool) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        let header = BlockHeader {
            prev_hash,
            timestamp,
            difficulty: 4,
            nonce: 0,
            issuance_type,
        };
        let mut block = Block {
            header,
            transactions,
            hash: String::new(),
            is_offchain,
        };
        block.hash = block.calculate_hash();
        block
    }

    /// 计算区块的哈希值
    /// 返回：区块的SHA256哈希值（16进制字符串）
    pub fn calculate_hash(&self) -> String {
        let data = serde_json::to_string(self).unwrap();
        sha256_hash(&data)
    }

    /// 更新nonce并重新计算哈希（用于PoW挖矿）
    /// 参数：
    /// - nonce: 新随机数
    pub fn update_nonce(&mut self, nonce: u64) {
        self.header.nonce = nonce;
        self.hash = self.calculate_hash();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let prev_hash = "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let transactions = vec![];
        let block = Block::new(prev_hash.clone(), transactions, "创世发行".to_string(), false);
        assert_eq!(block.header.prev_hash, prev_hash);
        assert_eq!(block.header.issuance_type, "创世发行");
        assert_eq!(block.is_offchain, false);
        assert!(!block.hash.is_empty());
    }
}