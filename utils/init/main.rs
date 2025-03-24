use wmb_core::{Chain, Block};  // 中文注释：引入核心层功能
use wmb_storage::Storage;  // 中文注释：引入存储层
use clap::Parser;  // 中文注释：引入命令行解析工具

// 中文注释：定义命令行参数结构
#[derive(Parser)]
#[command(name = "wmb-init", about = "初始化 WMB 区块链")]
struct Cli {
    #[arg(long, default_value = "blockchain.dat")]
    storage_path: String,  // 中文注释：存储路径
}

// 中文注释：初始化工具主函数
fn main() -> std::io::Result<()> {
    let cli = Cli::parse();  // 中文注释：解析命令行参数

    let mut chain = Chain::new();  // 中文注释：创建区块链
    let genesis_block = Block::new(0, "创世区块".to_string());  // 中文注释：创建创世区块
    chain.add_block(genesis_block.clone());  // 中文注释：添加创世区块
    let storage = Storage::new(&cli.storage_path);  // 中文注释：创建存储实例
    genesis_block.save(&storage)?;  // 中文注释：保存创世区块
    println!("中文注释：区块链初始化完成，创世区块已保存到 {}", cli.storage_path);

    Ok(())  // 中文注释：返回成功
}