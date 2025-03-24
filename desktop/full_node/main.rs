use clap::{Parser, Subcommand};  // 中文注释：引入命令行解析工具
use interface::Interface;        // 中文注释：引入接口层

// 中文注释：定义全节点前端的命令行参数结构
#[derive(Parser)]
#[command(name = "wmb-full-node", about = "WMB 全节点前端软件")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// 中文注释：定义支持的子命令
#[derive(Subcommand)]
enum Commands {
    // 中文注释：启动全节点
    Start,
    // 中文注释：查看状态
    Status,
}

// 中文注释：全节点前端主函数
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();  // 中文注释：解析命令行参数

    let mut interface = Interface::new("full_node.dat", "127.0.0.1:8084");  // 中文注释：初始化接口

    match cli.command {
        Commands::Start => {
            println!("中文注释：全节点启动");
            // 中文注释：实际节点逻辑待实现
        }
        Commands::Status => {
            let block_count = interface.get_block_count();  // 中文注释：查询区块数量
            println!("中文注释：全节点状态 - 区块数量: {}", block_count);
        }
    }

    Ok(())
}