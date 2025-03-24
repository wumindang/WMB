// 中文注释：门限签名模块，为超级节点、国储会等提供分布式签名功能
pub mod threshold {
    use core::Core;  // 中文注释：引入核心层模块

    // 中文注释：节点类型枚举，定义不同节点的门限要求
    pub enum NodeType {
        Super,    // 中文注释：超级节点，5-of-3
        NRC,      // 中文注释：国储会节点，19-of-13
        PRC,      // 中文注释：省储会节点，9-of-6
        PRB,      // 中文注释：省储行节点，9-of-6
        Full,     // 中文注释：全节点，5-of-3
    }

    // 中文注释：根据节点类型获取管理员总数和门限
    fn get_threshold_params(node_type: NodeType) -> (usize, usize) {
        match node_type {
            NodeType::Super => (5, 3),   // 中文注释：5 个管理员，3 个签名
            NodeType::NRC => (19, 13),   // 中文注释：19 个管理员，13 个签名
            NodeType::PRC => (9, 6),     // 中文注释：9 个管理员，6 个签名
            NodeType::PRB => (9, 6),     // 中文注释：9 个管理员，6 个签名
            NodeType::Full => (5, 3),    // 中文注释：5 个管理员，3 个签名
        }
    }

    // 中文注释：生成门限签名密钥分片
    pub fn generate_threshold_keys(node_type: NodeType) -> Vec<String> {
        let core = Core::new();
        let (n, t) = get_threshold_params(node_type);
        core.generate_threshold_keys(n, t)  // 中文注释：生成 n 个密钥分片，需 t 个签名
    }

    // 中文注释：使用门限签名分片签名
    pub fn sign_threshold(data: &str, key_shares: Vec<&str>, node_type: NodeType) -> String {
        let core = Core::new();
        let (_, t) = get_threshold_params(node_type);
        if key_shares.len() < t {
            return String::from("中文注释：分片数量不足，无法签名");
        }
        core.sign_threshold(data, key_shares)  // 中文注释：生成单一门限签名
    }

    // 中文注释：验证门限签名
    pub fn verify_threshold(data: &str, signature: &str, public_key: &str) -> bool {
        let core = Core::new();
        core.verify_threshold(data, signature, public_key)  // 中文注释：验证签名
    }
}