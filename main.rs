use core::{HDWallet, OnlyWallet, PrbWallet, NodeType}; // 直接使用 core crate

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prb_wallet = PrbWallet::new("prb_node_gds_0001".to_string(), 0)?;
    let prb_tx = prb_wallet.generate_transaction_address()?;
    let prb_stake = prb_wallet.generate_staking_address()?;
    println!("省储行权益节点助记词: {}", prb_wallet.get_mnemonic());
    println!("交易地址: {}", prb_tx);
    println!("质押地址: {}", prb_stake);

    let nrp_wallet = OnlyWallet::new(NodeType::NationalNode, "nrp_node_0001".to_string(), 1)?;
    let nrp_tx = nrp_wallet.generate_transaction_address()?;
    println!("\n国储会权威节点助记词: {}", nrp_wallet.get_mnemonic());
    println!("交易地址: {}", nrp_tx);

    let full_wallet = HDWallet::new(NodeType::FullNode, "full_node_001".to_string(), 2)?;
    println!("\n全节点助记词: {}", full_wallet.get_mnemonic());
    for i in 0..3 {
        let tx_address = full_wallet.generate_transaction_address(i)?;
        println!("交易地址 {}: {}", i, tx_address);
    }

    Ok(())
}