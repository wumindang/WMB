// 文件路径: WMB/core/src/consensus.rs

use crate::blockchain::{Block, Transaction};
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct ConsensusConfig {
    pub difficulty: u32,
    pub stake_threshold: f64,
}

pub struct PowConsensus {
    pub config: ConsensusConfig,
}

impl PowConsensus {
    pub fn new(difficulty: u32) -> Self {
        PowConsensus {
            config: ConsensusConfig {
                difficulty,
                stake_threshold: 0.0,
            },
        }
    }

    pub fn mine_block(&self, mut block: Block) -> Block {
        let target = "0".repeat(self.config.difficulty as usize);
        loop {
            let hash = block.calculate_hash();
            if hash.starts_with(&target) {
                block.hash = hash;
                return block;
            }
            block.header.nonce += 1;
        }
    }
}

pub struct PosConsensus {
    pub config: ConsensusConfig,
    pub stake: f64,
}

impl PosConsensus {
    pub fn new(stake: f64, stake_threshold: f64) -> Self {
        PosConsensus {
            config: ConsensusConfig {
                difficulty: 0,
                stake_threshold,
            },
            stake,
        }
    }

    pub fn validate_transaction(&self, tx: &Transaction) -> bool {
        if self.stake < self.config.stake_threshold {
            return false;
        }
        let _tx_hash = Self::calculate_tx_hash(tx);
        tx.signature.len() > 0
    }

    fn calculate_tx_hash(tx: &Transaction) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&tx.sender);
        hasher.update(&tx.receiver);
        hasher.update(tx.amount.to_string());
        hasher.update(tx.fee.to_string());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

pub struct HybridConsensus {
    pow: PowConsensus,
    pos: PosConsensus,
}

impl HybridConsensus {
    pub fn new(difficulty: u32, stake: f64, stake_threshold: f64) -> Self {
        HybridConsensus {
            pow: PowConsensus::new(difficulty),
            pos: PosConsensus::new(stake, stake_threshold),
        }
    }

    pub fn process_block(&self, previous_hash: String, transactions: Vec<Transaction>) -> Block {
        let mut valid_txs = Vec::new();
        for tx in transactions {
            if self.pos.validate_transaction(&tx) {
                valid_txs.push(tx);
            }
        }
        let block = Block::new(previous_hash, valid_txs);
        self.pow.mine_block(block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hybrid_consensus() {
        let consensus = HybridConsensus::new(4, 100.00, 50.00);
        let tx = Transaction {
            sender: "addr1".to_string(),
            receiver: "addr2".to_string(),
            amount: 10.00,
            fee: 0.50,
            signature: vec![0x01, 0x02, 0x03],
        };
        let block = consensus.process_block("0000".to_string(), vec![tx]);
        assert!(block.hash.starts_with("0000"));
        println!("区块哈希: {}", block.hash);
    }
}