
#[derive(Debug, Clone)]
pub struct ProvincialAuthorityInfo {
    pub node_id: String,       // "PROV_AUTH_001" 等
    pub admins: Vec<String>,   // 9个管理员公钥
    pub wallet: String,        // 交易钱包地址
    pub is_initial: bool,      // 是否初始节点
}

impl ProvincialAuthorityInfo {
    pub fn new(node_id: String, admins: Vec<String>, wallet: String, is_initial: bool) -> Self {
        Self { node_id, admins, wallet, is_initial }
    }
}