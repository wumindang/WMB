use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use serde::{Serialize, Deserialize};

/// 网络消息类型
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    /// 广播新交易
    Transaction(Transaction),
    /// 广播新区块
    Block(Block),
    /// 请求同步（暂未实现）
    SyncRequest,
}

impl Message {
    /// 将消息序列化为字节
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    /// 从字节反序列化为消息
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        serde_json::from_slice(bytes).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let tx = Transaction::new("sender".to_string(), "receiver".to_string(), 10.0, 0.05, "链上".to_string());
        let msg = Message::Transaction(tx.clone());
        let bytes = msg.to_bytes();
        let deserialized = Message::from_bytes(&bytes).unwrap();
        match deserialized {
            Message::Transaction(deserialized_tx) => {
                assert_eq!(deserialized_tx.sender, tx.sender);
                assert_eq!(deserialized_tx.receiver, tx.receiver);
            }
            _ => panic!("反序列化失败"),
        }
    }
}