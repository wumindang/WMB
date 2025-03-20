// 文件路径: WMB/core/src/blockchain.rs

use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub fee: f64,
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub previous_hash: String,
    pub timestamp: u64,
    pub difficulty: u32,
    pub nonce: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub hash: String,
}

impl Block {
    pub fn new(previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let header = BlockHeader {
            previous_hash,
            timestamp,
            difficulty: 4,
            nonce: 0,
        };
        let mut block = Block {
            header,
            transactions,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let header_str = format!(
            "{}{}{}{}",
            self.header.previous_hash,
            self.header.timestamp,
            self.header.difficulty,
            self.header.nonce
        );
        hasher.update(header_str);
        for tx in &self.transactions {
            hasher.update(&tx.sender);
            hasher.update(&tx.receiver);
            hasher.update(tx.amount.to_string());
            hasher.update(tx.fee.to_string());
            hasher.update(&tx.signature);
        }
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let tx = Transaction {
            sender: "addr1".to_string(),
            receiver: "addr2".to_string(),
            amount: 10.00,
            fee: 0.50,
            signature: vec![0x01, 0x02, 0x03],
        };
        let block = Block::new("0000".to_string(), vec![tx]);
        assert!(!block.hash.is_empty());
        println!("区块哈希: {}", block.hash);
    }
}