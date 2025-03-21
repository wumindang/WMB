// 文件路径: src/blockchain.rs
// 作用: 定义交易和区块结构，管理区块链状态

use serde::{Serialize, Deserialize}; // 序列化和反序列化支持
use sha3::{Digest, Keccak256};       // Keccak256 哈希算法
use chrono;                          // 时间处理库

// 交易结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,      // 发送者地址
    pub receiver: String,    // 接收者地址
    pub amount: f64,         // 交易金额（单位：元）
    pub fee: f64,            // 交易费用（单位：元）
    pub signature: Vec<u8>,  // 交易签名
}

// 区块结构
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u64,           // 区块高度
    pub previous_hash: String, // 前一区块哈希
    pub timestamp: i64,        // 时间戳
    pub transactions: Vec<Transaction>, // 区块包含的交易列表
    pub hash: String,          // 当前区块哈希
}

// 区块链管理
pub struct Blockchain {
    blocks: Vec<Block>, // 区块列表
}

impl Blockchain {
    // 创建新区块链
    pub fn new() -> Self {
        Blockchain { blocks: Vec::new() }
    }

    // 添加新区块
    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_hash = if self.blocks.is_empty() {
            "0".to_string() // 创世区块的前哈希为 "0"
        } else {
            self.blocks.last().unwrap().hash.clone() // 获取前一区块的哈希
        };
        let index = self.blocks.len() as u64; // 当前区块高度
        let timestamp = chrono::Utc::now().timestamp(); // 当前时间戳

        let mut block = Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            hash: String::new(), // 初始哈希为空
        };
        block.hash = block.calculate_hash(); // 计算区块哈希
        self.blocks.push(block); // 将区块添加到链中
    }

    // 获取最新区块
    pub fn get_latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}

impl Block {
    // 计算区块哈希
    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{:?}",
            self.index, self.previous_hash, self.timestamp, self.transactions
        ); // 拼接区块数据
        let mut hasher = Keccak256::new(); // 创建 Keccak256 哈希器
        hasher.update(input); // 输入数据
        hex::encode(hasher.finalize()) // 返回哈希的十六进制字符串
    }
}