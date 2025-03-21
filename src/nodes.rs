// 文件路径: src/nodes.rs
// 作用: 管理不同类型的节点

use serde::{Serialize, Deserialize}; // 序列化和反序列化支持
use serde_json;                     // JSON 序列化支持
use std::collections::HashMap;       // 哈希表
use std::sync::{Arc, Mutex};         // 线程安全锁

// 节点类型枚举
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NodeType {
    CitizenLight,       // 公民轻节点
    GuestLight,         // 访客轻节点
    Super,              // 超级节点
    NationalAuthority,  // 国储会权威节点
    ProvincialAuthority,// 省储会权威节点
    ProvincialEquity,   // 省储行权益节点
    Full,               // 全节点
}

// 节点结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub peer_id: String,    // 节点 Peer ID
    pub address: String,    // 节点地址（Multiaddr）
    pub status: String,     // 节点状态（例如 "active"）
    pub node_type: NodeType,// 节点类型
}

// 节点管理器
#[derive(Clone)]
pub struct NodeManager {
    nodes: Arc<Mutex<HashMap<String, Node>>>, // 线程安全的节点哈希表
}

impl NodeManager {
    // 创建新节点管理器
    pub fn new() -> Self {
        NodeManager {
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // 添加节点
    pub fn add_node(&self, peer_id: &str, address: &str, node_type: NodeType) -> Result<(), String> {
        let mut nodes = self.nodes.lock().map_err(|e| format!("锁失败: {}", e))?; // 获取锁
        if nodes.contains_key(peer_id) {
            return Err(format!("节点 {} 已存在", peer_id)); // 检查是否重复
        }
        nodes.insert(
            peer_id.to_string(),
            Node {
                peer_id: peer_id.to_string(),
                address: address.to_string(),
                status: "active".to_string(),
                node_type,
            },
        ); // 添加新节点
        Ok(())
    }

    // 移除节点
    pub fn remove_node(&self, peer_id: &str) -> Result<(), String> {
        let mut nodes = self.nodes.lock().map_err(|e| format!("锁失败: {}", e))?; // 获取锁
        if nodes.remove(peer_id).is_none() {
            return Err(format!("节点 {} 不存在", peer_id)); // 检查是否存在
        }
        Ok(())
    }

    // 列出所有节点
    pub fn list_nodes(&self) -> Result<Vec<Node>, String> {
        let nodes = self.nodes.lock().map_err(|e| format!("锁失败: {}", e))?; // 获取锁
        Ok(nodes.values().cloned().collect()) // 返回节点列表
    }

    // 将节点列表转换为 JSON
    pub fn to_json(&self) -> Result<String, String> {
        let nodes = self.list_nodes()?; // 获取节点列表
        serde_json::to_string(&nodes).map_err(|e| format!("JSON 序列化失败: {}", e)) // 转换为 JSON
    }
}