// 声明钱包模块的子模块
pub mod hd_wallet;
pub mod only_wallet;
pub mod prb_wallet;
pub mod wallet;

// 导出公共接口，方便外部使用
pub use hd_wallet::HDWallet;
pub use only_wallet::OnlyWallet;
pub use prb_wallet::PrbWallet;

// 定义节点类型枚举，用于区分不同节点的钱包行为
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    SuperNode,         // 超级节点 (super_node)
    NationalNode,      // 国储会权威节点 (nrc_node)
    ProvincialAuthNode,// 省储会权威节点 (prc_nodes)
    ProvincialBankNode,// 省储行权益节点 (prb_nodes)
    CitizenNode,       // 公民轻节点 (citizen_nodes)
    FullNode,          // 全节点 (full_nodes)
    GuestNode,         // 访客轻节点 (guest_nodes)
}