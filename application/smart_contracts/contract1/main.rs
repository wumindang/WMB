use wmb_core::{Chain, Block, Wallet};  // 中文注释：引入核心层功能
use wmb_security::hash;  // 中文注释：引入安全层哈希函数

// 中文注释：智能合约 1 主函数
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut chain = Chain::new();  // 中文注释：初始化区块链
    println!("中文注释：智能合约 1 区块链初始化完成");

    let contract_data = "合约: 转账 100 单位";  // 中文注释：合约数据
    let block = Block::new(1, contract_data.to_string());  // 中文注释：创建合约区块
    chain.add_block(block);  // 中文注释：添加区块
    println!("中文注释：合约区块已添加: {}", contract_data);

    let wallet = Wallet::new("合约地址1".to_string());  // 中文注释：初始化钱包
    let signature = wallet.sign_transaction(contract_data);  // 中文注释：签名合约
    println!("中文注释：合约签名: {}", signature);

    Ok(())  // 中文注释：返回成功
}