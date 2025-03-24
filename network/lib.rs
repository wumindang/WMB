use serde::{Serialize, Deserialize};  // 中文注释：引入序列化和反序列化支持
use tokio::net::{TcpListener, TcpStream};  // 中文注释：引入 TCP 网络支持
use std::io;  // 中文注释：引入输入输出支持

// 中文注释：定义网络消息结构
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    sender: String,    // 中文注释：发送者标识
    content: Vec<u8>,  // 中文注释：消息内容（二进制格式）
}

// 中文注释：网络层的主结构
pub struct Network {
    address: String,  // 中文注释：网络监听地址（例如 "127.0.0.1:8080"）
}

impl Network {
    // 中文注释：创建新的网络实例，指定监听地址
    pub fn new(address: &str) -> Self {
        Network { address: address.to_string() }  // 中文注释：初始化网络地址
    }

    // 中文注释：异步广播消息到指定地址
    pub async fn broadcast(&self, message: Message, target_addr: &str) -> io::Result<()> {
        let mut stream = TcpStream::connect(target_addr).await?;  // 中文注释：连接目标地址
        let encoded = bincode::serialize(&message).unwrap();  // 中文注释：序列化消息
        stream.write_all(&encoded).await?;  // 中文注释：发送消息
        Ok(())  // 中文注释：返回成功
    }

    // 中文注释：异步启动监听服务，接收消息
    pub async fn listen(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;  // 中文注释：绑定监听地址
        println!("中文注释：网络监听启动于 {}", self.address);

        loop {
            let (mut socket, addr) = listener.accept().await?;  // 中文注释：接受客户端连接
            tokio::spawn(async move {
                let mut buffer = Vec::new();  // 中文注释：创建缓冲区
                socket.read_to_end(&mut buffer).await.unwrap();  // 中文注释：读取消息
                let message: Message = bincode::deserialize(&buffer).unwrap();  // 中文注释：反序列化消息
                println!("中文注释：收到消息从 {}: {:?}", addr, message);
            });
        }
    }
}

// 中文注释：测试模块，用于验证网络层功能
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    // 中文注释：测试广播和监听功能
    #[tokio::test]
    async fn test_broadcast_and_listen() {
        let listener_network = Network::new("127.0.0.1:8081");  // 中文注释：创建监听网络实例
        let sender_network = Network::new("127.0.0.1:8082");    // 中文注释：创建发送网络实例

        tokio::spawn(async move {
            listener_network.listen().await.unwrap();  // 中文注释：启动监听服务
        });

        sleep(Duration::from_millis(100)).await;  // 中文注释：等待监听服务启动

        let message = Message {
            sender: "测试节点".to_string(),
            content: vec![1, 2, 3],
        };

        let result = sender_network.broadcast(message, "127.0.0.1:8081").await;  // 中文注释：广播消息
        assert!(result.is_ok(), "中文注释：广播消息应成功");

        sleep(Duration::from_millis(100)).await;  // 中文注释：等待消息处理
    }
}