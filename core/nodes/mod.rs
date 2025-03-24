use crate::wmb_network::{Network, Message};  // 中文注释：引入网络层

// 中文注释：定义节点结构，包含 ID 和角色
pub struct Node {
    pub id: String,    // 中文注释：节点唯一标识
    pub role: String,  // 中文注释：节点角色（例如超级节点、全节点等）
    network: Network,  // 中文注释：网络实例
}

impl Node {
    // 中文注释：创建新节点，传入 ID、角色和网络地址
    pub fn new(id: String, role: String, network_addr: &str) -> Self {
        let network = Network::new(network_addr);  // 中文注释：初始化网络
        Node { id, role, network }  // 中文注释：返回新创建的节点实例
    }

    // 中文注释：广播节点消息
    pub async fn broadcast(&self, data: Vec<u8>, target_addr: &str) -> std::io::Result<()> {
        let message = Message {
            sender: self.id.clone(),  // 中文注释：设置发送者为节点 ID
            content: data,            // 中文注释：设置消息内容
        };
        self.network.broadcast(message, target_addr).await  // 中文注释：调用网络层广播
    }
}