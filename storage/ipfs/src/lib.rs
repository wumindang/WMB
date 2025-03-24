// 中文注释：IPFS 存储模块，为数据归档节点（国储会、省储会、省储行、全节点）提供分布式存储功能
pub mod ipfs {
    use core::Core;  // 中文注释：引入核心层模块
    use std::error::Error;

    // 中文注释：IPFS 存储客户端结构体
    pub struct IpfsClient {
        core: Core,       // 中文注释：核心层实例
        endpoint: String, // 中文注释：IPFS 节点地址
    }

    impl IpfsClient {
        // 中文注释：创建 IPFS 客户端实例
        pub fn new(endpoint: &str) -> Self {
            IpfsClient {
                core: Core::new(),
                endpoint: endpoint.to_string(),
            }
        }

        // 中文注释：存储区块到 IPFS，返回 IPFS 哈希
        pub async fn store_block(&self, block: &str) -> Result<String, Box<dyn Error>> {
            let hash = self.core.ipfs_store(block).await?;  // 中文注释：调用核心层存储方法
            Ok(hash)  // 中文注释：返回存储后的 IPFS 哈希
        }

        // 中文注释：从 IPFS 获取区块，通过哈希读取
        pub async fn get_block(&self, hash: &str) -> Result<String, Box<dyn Error>> {
            let block = self.core.ipfs_get(hash).await?;  // 中文注释：调用核心层读取方法
            Ok(block)  // 中文注释：返回区块数据
        }

        // 中文注释：存储任意数据到 IPFS，返回哈希
        pub async fn store_data(&self, data: &str) -> Result<String, Box<dyn Error>> {
            let hash = self.core.ipfs_store(data).await?;
            Ok(hash)  // 中文注释：返回数据哈希
        }

        // 中文注释：从 IPFS 获取任意数据
        pub async fn get_data(&self, hash: &str) -> Result<String, Box<dyn Error>> {
            let data = self.core.ipfs_get(hash).await?;
            Ok(data)  // 中文注释：返回数据内容
        }
    }
}

// 中文注释：模块测试
#[cfg(test)]
mod tests {
    use super::ipfs::IpfsClient;
    use tokio;  // 中文注释：异步测试需要 tokio

    #[tokio::test]
    async fn test_store_and_get_block() {
        let client = IpfsClient::new("http://localhost:5001");  // 中文注释：假设本地 IPFS 节点
        let block = "区块数据示例";
        let hash = client.store_block(block).await.unwrap();  // 中文注释：存储区块
        let retrieved = client.get_block(&hash).await.unwrap();  // 中文注释：读取区块
        assert_eq!(block, retrieved);  // 中文注释：验证数据一致性
    }
}