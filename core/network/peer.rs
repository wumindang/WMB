use std::net::{SocketAddr, TcpStream};
use std::io::{Read, Write};

/// 对等节点信息
#[derive(Debug, Clone)]
pub struct Peer {
    pub node_id: String,         // 节点ID
    pub address: SocketAddr,     // 节点网络地址（如 "127.0.0.1:3030"）
    pub is_provincial: bool,     // 是否为省储行节点
}

impl Peer {
    /// 创建新对等节点
    pub fn new(node_id: String, address: SocketAddr, is_provincial: bool) -> Self {
        Self { node_id, address, is_provincial }
    }

    /// 连接到对等节点
    pub fn connect(&self) -> Option<TcpStream> {
        TcpStream::connect(self.address).ok()
    }

    /// 发送消息到对等节点
    pub fn send_message(&self, message: &[u8]) -> Result<(), std::io::Error> {
        let mut stream = self.connect().ok_or(std::io::Error::new(std::io::ErrorKind::NotFound, "连接失败"))?;
        stream.write_all(message)?;
        Ok(())
    }

    /// 从对等节点接收消息
    pub fn receive_message(&self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        let mut stream = self.connect().ok_or(std::io::Error::new(std::io::ErrorKind::NotFound, "连接失败"))?;
        stream.read(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, SocketAddrV4, Ipv4Addr};

    #[test]
    fn test_peer_connection() {
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));
        let listener = TcpListener::bind(addr).unwrap();
        let peer_addr = listener.local_addr().unwrap();
        let peer = Peer::new("TEST_PEER".to_string(), peer_addr, false);

        let message = b"test message";
        assert!(peer.send_message(message).is_ok());
    }
}