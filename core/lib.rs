// 中文注释：声明核心层的子模块
pub mod blockchain;  // 区块链数据结构模块
pub mod consensus;   // 共识机制模块
pub mod nodes;       // 节点管理模块
pub mod wallet;      // 钱包模块

// 中文注释：公开导出 Block 和 Wallet，方便外部使用
pub use blockchain::Block;
pub use wallet::Wallet;

// 中文注释：定义区块链链结构，用于存储区块
pub struct Chain {
    blocks: Vec<Block>,  // 中文注释：区块数组
}

impl Chain {
    // 中文注释：创建新的区块链实例
    pub fn new() -> Self {
        Chain { blocks: Vec::new() }  // 中文注释：初始化空区块数组
    }

    // 中文注释：向区块链中添加新区块
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);  // 中文注释：将区块追加到数组
    }
}

// 中文注释：测试模块，用于验证 core crate 功能
#[cfg(test)]
mod tests {
    use super::*;

    // 中文注释：测试创建区块链和添加区块
    #[test]
    fn test_chain_add_block() {
        let mut chain = Chain::new();  // 中文注释：创建新区块链
        let block = Block::new(1, "测试数据".to_string());  // 中文注释：创建新区块
        chain.add_block(block);  // 中文注释：添加区块
        assert_eq!(chain.blocks.len(), 1);  // 中文注释：验证区块数量是否为 1
        assert_eq!(chain.blocks[0].id, 1);  // 中文注释：验证区块 ID
        assert_eq!(chain.blocks[0].data, "测试数据");  // 中文注释：验证区块数据
    }

    // 中文注释：测试钱包签名功能
    #[test]
    fn test_wallet_sign() {
        let wallet = Wallet::new("地址1".to_string());  // 中文注释：创建新钱包
        let signature = wallet.sign_transaction("交易数据");  // 中文注释：签名交易
        assert_eq!(signature, "signed_交易数据_地址1");  // 中文注释：验证签名结果
    }
    // 中文注释：测试节点广播功能
    #[tokio::test]
    async fn test_node_broadcast() {
        let node = Node::new("节点1".to_string(), "全节点".to_string(), "127.0.0.1:8083");  // 中文注释：创建节点
        let data = vec![4, 5, 6];  // 中文注释：测试数据
        let result = node.broadcast(data, "127.0.0.1:8083").await;  // 中文注释：广播消息
        assert!(result.is_ok(), "中文注释：广播应成功");
    }
    // 中文注释：测试节点广播功能
    #[tokio::test]
    async fn test_node_broadcast() {
        let node = Node::new("节点1".to_string(), "全节点".to_string(), "127.0.0.1:8083");  // 中文注释：创建节点
        let data = vec![4, 5, 6];  // 中文注释：测试数据
        let result = node.broadcast(data, "127.0.0.1:8083").await;  // 中文注释：广播消息
        assert!(result.is_ok(), "中文注释：广播应成功");
    }
}