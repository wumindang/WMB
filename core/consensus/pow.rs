use crate::blockchain::block::Block;

/// 全节点结构体
#[derive(Debug, Clone)]
pub struct FullNode {
    pub node_id: String,           // 全节点ID，如 "FULLNODE_001"
    pub public_key: secp256k1::PublicKey, // 节点公钥
}

/// 全节点执行 PoW 铸块
/// 参数：
/// - block: 待铸造的区块
/// - difficulty: 铸块难度（哈希前导零的个数）
/// - node: 执行铸块的全节点
/// - registered_nodes: 已注册的全节点ID列表
/// 返回：是否成功铸块
pub fn mine_block(block: &mut Block, difficulty: u32, node: &FullNode, registered_nodes: &[String]) -> bool {
    if !registered_nodes.contains(&node.node_id) {
        return false; // 未注册节点无法铸块
    }

    let target = "0".repeat(difficulty as usize);
    loop {
        let hash = block.calculate_hash();
        if hash.starts_with(&target) {
            block.hash = hash;
            return true;
        }
        block.header.nonce += 1;
    }
}

/// 验证 PoW 是否有效
/// 参数：
/// - block: 已铸造的区块
/// - difficulty: 预期难度
/// - node_id: 铸块节点ID
/// - registered_nodes: 已注册的全节点ID列表
/// 返回：是否有效
pub fn validate_pow(block: &Block, difficulty: u32, node_id: &str, registered_nodes: &[String]) -> bool {
    if !registered_nodes.contains(&node_id.to_string()) {
        return false;
    }
    let target = "0".repeat(difficulty as usize);
    block.hash.starts_with(&target) // 只验证前缀，不重新计算哈希
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_keypair;

    #[test]
    fn test_pow_mining_with_valid_node() {
        let (_sk, pk) = generate_keypair();
        let node = FullNode {
            node_id: "FULLNODE_001".to_string(),
            public_key: pk,
        };
        let registered_nodes = vec!["FULLNODE_001".to_string()];
        let prev_hash = "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let transactions = vec![];
        let mut block = Block::new(prev_hash, transactions, "全节点铸块".to_string(), false);
        let difficulty = 2;
        assert!(mine_block(&mut block, difficulty, &node, &registered_nodes));
        assert!(validate_pow(&block, difficulty, &node.node_id, &registered_nodes));
    }

    #[test]
    fn test_pow_mining_with_invalid_node() {
        let (_sk, pk) = generate_keypair();
        let node = FullNode {
            node_id: "FULLNODE_999".to_string(),
            public_key: pk,
        };
        let registered_nodes = vec!["FULLNODE_001".to_string()];
        let prev_hash = "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let transactions = vec![];
        let mut block = Block::new(prev_hash, transactions, "全节点铸块".to_string(), false);
        let difficulty = 2;
        assert!(!mine_block(&mut block, difficulty, &node, &registered_nodes));
    }
}