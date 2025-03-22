#[derive(Debug, Clone)]
pub struct ProvincialReserveInfo {
    pub node_id: String,       // "PROV_RES_001" 等
    pub admins: Vec<String>,   // 9个管理员公钥
    pub stake_wallet: String,  // 质押钱包地址
    pub tx_wallet: String,     // 交易钱包地址
    pub staked_amount: f64,    // 质押金额
    pub is_initial: bool,      // 是否初始节点
}

impl ProvincialReserveInfo {
    pub fn new(node_id: String, admins: Vec<String>, stake_wallet: String, tx_wallet: String, staked_amount: f64, is_initial: bool) -> Self {
        Self { node_id, admins, stake_wallet, tx_wallet, staked_amount, is_initial }
    }
}