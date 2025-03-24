use core::{Chain, Block, Wallet, Node};  // 中文注释：引入核心层功能
use security::hash;  // 中文注释：引入安全层哈希函数
use storage::Storage;  // 中文注释：引入存储层
use network::{Network, Message};  // 中文注释：引入网络层
use serde::{Serialize, Deserialize};  // 中文注释：引入序列化支持

// 中文注释：定义接口层提供的交易请求结构
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionRequest {
    sender: String,    // 中文注释：发送者地址
    data: String,      // 中文注释：交易数据
}

// 中文注释：接口层主结构，整合底层功能
pub struct Interface {
    chain: Chain,        // 中文注释：区块链实例
    storage: Storage,    // 中文注释：存储实例
    network: Network,    // 中文注释：网络实例
}

impl Interface {
    // 中文注释：创建新的接口实例
    pub fn new(storage_path: &str, network_addr: &str) -> Self {
        let chain = Chain::new();  // 中文注释：初始化区块链
        let storage = Storage::new(storage_path);  // 中文注释：初始化存储
        let network = Network::new(network_addr);  // 中文注释：初始化网络
        Interface { chain, storage, network }
    }

    // 中文注释：处理交易请求，添加到区块链并广播
    pub async fn process_transaction(&mut self, request: TransactionRequest) -> std::io::Result<String> {
        let block = Block::new(self.chain.blocks.len() as u32 + 1, request.data.clone());  // 中文注释：创建新区块
        self.chain.add_block(block.clone());  // 中文注释：添加到区块链
        block.save(&self.storage)?;  // 中文注释：保存到存储层

        let wallet = Wallet::new(request.sender.clone());  // 中文注释：创建钱包
        let signature = wallet.sign_transaction(&request.data);  // 中文注释：签名交易

        let message = Message {
            sender: request.sender,  // 中文注释：设置消息发送者
            content: request.data.into_bytes(),  // 中文注释：设置消息内容
        };
        self.network.broadcast(message, "127.0.0.1:8080").await?;  // 中文注释：广播交易

        Ok(signature)  // 中文注释：返回签名作为确认
    }

    // 中文注释：查询区块链中的区块数量
    pub fn get_block_count(&self) -> usize {
        self.chain.blocks.len()  // 中文注释：返回当前区块数量
    }
}

// 中文注释：测试模块，用于验证接口层功能
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    // 中文注释：测试交易处理功能
    #[tokio::test]
    async fn test_process_transaction() {
        let mut interface = Interface::new("test_interface.dat", "127.0.0.1:8086");  // 中文注释：创建接口实例
        let request = TransactionRequest {
            sender: "测试地址".to_string(),  // 中文注释：测试发送者
            data: "测试交易数据".to_string(),  // 中文注释：测试交易数据
        };

        let signature = interface.process_transaction(request).await.unwrap();  // 中文注释：处理交易
        assert!(!signature.is_empty(), "中文注释：验证签名非空");  // 中文注释：检查签名
        assert_eq!(interface.get_block_count(), 1, "中文注释：验证区块数量");  // 中文注释：检查区块数量

        let loaded = interface.storage.load("1").unwrap();  // 中文注释：加载存储数据
        assert_eq!(loaded, Some("1-测试交易数据".as_bytes().to_vec()), "中文注释：验证存储数据");  // 中文注释：检查存储

        // 中文注释：清理测试文件
        std::fs::remove_file("test_interface.dat").unwrap();
    }
}