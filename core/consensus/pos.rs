use crate::blockchain::transaction::Transaction;
use std::collections::HashMap;

/// 省储行权益节点结构体
#[derive(Debug, Clone)]
pub struct ProvincialNode {
    pub node_id: String,           // 节点ID，如 "PROVINCIAL_001"
    pub public_key: secp256k1::PublicKey, // 节点公钥
    pub is_initial: bool,          // 是否为初始43个节点（不可删除）
    pub staked_amount: f64,        // 永久质押的五民币数量（单位：元）
}

/// 省储行节点管理器
#[derive(Debug)]
pub struct ProvincialNodeManager {
    pub nodes: HashMap<String, ProvincialNode>, // 节点ID到节点的映射
    pub initial_count: usize,                   // 初始节点数量（固定为43）
}

impl ProvincialNodeManager {
    pub fn new(initial_nodes: Vec<ProvincialNode>) -> Self {
        let mut nodes = HashMap::new();
        for node in initial_nodes {
            nodes.insert(node.node_id.clone(), node);
        }
        Self {
            initial_count: 43,
            nodes,
        }
    }

    pub fn add_node(&mut self, node: ProvincialNode) {
        if !node.is_initial {
            self.nodes.insert(node.node_id.clone(), node);
        }
    }

    pub fn remove_node(&mut self, node_id: &str) {
        if let Some(node) = self.nodes.get(node_id) {
            if !node.is_initial {
                self.nodes.remove(node_id);
            }
        }
    }
}

/// 验证交易（省储行节点）
/// 参数：
/// - transactions: 待验证的交易列表
/// - manager: 省储行节点管理器
/// - validator_node_ids: 当前参与验证的节点ID列表
/// 返回：是否通过（80%以上的节点验证通过）
pub fn validate_transactions(
    transactions: &[Transaction],
    manager: &ProvincialNodeManager,
    validator_node_ids: &[String],
) -> bool {
    for node_id in validator_node_ids {
        if !manager.nodes.contains_key(node_id) {
            return false; // 未注册节点
        }
    }

    let total_nodes = validator_node_ids.len();
    let required_votes = (total_nodes as f64 * 0.8).ceil() as usize;
    let mut valid_nodes = 0;

    // 对于每笔交易，检查有多少节点验证通过
    for node_id in validator_node_ids {
        if let Some(node) = manager.nodes.get(node_id) {
            let all_valid = transactions.iter().all(|tx| tx.verify(&node.public_key));
            if all_valid {
                valid_nodes += 1;
            }
        }
    }

    valid_nodes >= required_votes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_keypair;
    use crate::blockchain::transaction::Transaction;

    #[test]
    fn test_pos_validation_with_node_ids() {
        let (sk1, pk1) = generate_keypair();
        let (_sk2, pk2) = generate_keypair();
        let sender = hex::encode(pk1.serialize());
        let mut tx = Transaction::new(sender.clone(), "receiver".to_string(), 10.0, 0.05, "链上".to_string());
        tx.sign(&sk1);

        let manager = ProvincialNodeManager::new(vec![
            ProvincialNode {
                node_id: "PROVINCIAL_001".to_string(),
                public_key: pk1,
                is_initial: true,
                staked_amount: 1000.0,
            },
            ProvincialNode {
                node_id: "PROVINCIAL_002".to_string(),
                public_key: pk2,
                is_initial: true,
                staked_amount: 1000.0,
            },
        ]);

        let validator_node_ids = vec!["PROVINCIAL_001".to_string()]; // 只用 pk1 验证，确保 80% 通过
        let transactions = vec![tx];
        assert!(validate_transactions(&transactions, &manager, &validator_node_ids));

        let invalid_node_ids = vec!["PROVINCIAL_999".to_string()];
        assert!(!validate_transactions(&transactions, &manager, &invalid_node_ids));
    }
}