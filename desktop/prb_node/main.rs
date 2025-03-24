use clap::{Parser, Subcommand};  // 中文注释：引入命令行解析工具
use interface::Interface;        // 中文注释：引入接口层

// 中文注释：定义省储备银行节点前端的命令行参数结构
#[derive(Parser)]
#[command(name = "wmb-prb-node", about = "WMB 省储备银行节点前端软件")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// 中文注释：定义支持的子命令
#[derive(Subcommand)]
enum Commands {
    // 中文注释：启动省储备银行节点
    Start,
    // 中文注释：查看状态
    Status,
}

// 中文注释：省储备银行节点前端主函数
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();  // 中文注释：解析命令行参数

    let mut interface = Interface::new("prb_node.dat", "127.0.0.1:8083");  // 中文注释：初始化接口

    match cli.command {
        Commands::Start => {
            println!("中文注释：省储备银行节点启动");
            // 中文注释：实际节点逻辑待实现，例如管理省级银行交易
        }
        Commands::Status => {
            let block_count = interface.get_block_count();  // 中文注释：查询区块数量
            println!("中文注释：省储备银行节点状态 - 区块数量: {}", block_count);
        }
    }

    Ok(())
}