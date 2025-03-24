use super::peer::Peer;
use super::message::Message;
use crate::consensus::pow::FullNode;
use crate::consensus::pos::{ProvincialNodeManager, validate_transactions};
use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use std::net::{TcpListener, SocketAddr};
use std::io::Read;
use std::thread;

/// 网络管理器
pub struct Network {
    pub local_node_id: String,         // 本地节点ID
    pub peers: Vec<Peer>,              // 对等节点列表
    pub listener: TcpListener,         // 本地监听器
}

impl Network {
    /// 创建网络实例
    pub fn new(node_id: String, addr: SocketAddr, peers: Vec<Peer>) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(addr)?; // 返回 Result
        Ok(Self {
            local_node_id: node_id,
            peers,
            listener,
        })
    }

    /// 广播消息到所有对等节点
    pub fn broadcast(&self, message: Message) {
        let bytes = message.to_bytes();
        for peer in &self.peers {
            let _ = peer.send_message(&bytes);
        }
    }

    /// 启动网络监听
    pub fn start(&self) {
        let listener = self.listener.try_clone().unwrap();
        thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(mut stream) = stream {
                    let mut buffer = [0; 1024];
                    if let Ok(size) = stream.read(&mut buffer) {
                        if let Some(msg) = Message::from_bytes(&buffer[..size]) {
                            println!("收到消息: {:?}", msg);
                        }
                    }
                }
            }
        });
    }

    /// 全节点铸块并广播
    pub fn mine_and_broadcast(
        &self,
        full_node: &FullNode,
        registered_nodes: &[String],
        difficulty: u32,
        transactions: Vec<Transaction>,
    ) {
        let prev_hash = "0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let mut block = Block::new(prev_hash, transactions, "全节点铸块".to_string(), false);
        if crate::consensus::pow::mine_block(&mut block, difficulty, full_node, registered_nodes) {
            self.broadcast(Message::Block(block));
        }
    }

    /// 省储行节点验证交易并广播
    pub fn validate_and_broadcast(
        &self,
        manager: &ProvincialNodeManager,
        validator_node_ids: &[String],
        transactions: Vec<Transaction>,
    ) {
        if validate_transactions(&transactions, manager, validator_node_ids) {
            for tx in transactions {
                self.broadcast(Message::Transaction(tx));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{SocketAddrV4, Ipv4Addr};

    #[test]
    fn test_network_broadcast() {
        // 使用随机端口，避免冲突
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));
        let peer_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));
        
        let peer = Peer::new("PEER_001".to_string(), peer_addr, false);
        let network = Network::new("NODE_001".to_string(), addr, vec![peer]).expect("网络初始化失败");
        network.start();

        let tx = Transaction::new("sender".to_string(), "receiver".to_string(), 10.0, 0.05, "链上".to_string());
        network.broadcast(Message::Transaction(tx));
        // 测试广播功能，实际需检查接收端，这里仅验证初始化和广播不崩溃
    }
}