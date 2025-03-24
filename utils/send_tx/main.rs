use wmb_interface::{Interface, TransactionRequest};  // 中文注释：引入接口层功能
use clap::Parser;  // 中文注释：引入命令行解析工具

// 中文注释：定义命令行参数结构
#[derive(Parser)]
#[command(name = "wmb-send-tx", about = "发送 WMB 交易")]
struct Cli {
    #[arg(long)]
    sender: String,  // 中文注释：发送者地址
    #[arg(long)]
    data: String,    // 中文注释：交易数据
    #[arg(long, default_value = "127.0.0.1:8088")]
    network_addr: String,  // 中文注释：网络地址
}

// 中文注释：交易发送工具主函数
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();  // 中文注释：解析命令行参数

    let mut interface = Interface::new("utils_tx.dat", &cli.network_addr);  // 中文注释：创建接口实例
    let request = TransactionRequest {
        sender: cli.sender,  // 中文注释：发送者地址
        data: cli.data,      // 中文注释：交易数据
    };
    let signature = interface.process_transaction(request).await?;  // 中文注释：处理交易
    println!("中文注释：交易已发送，签名: {}", signature);

    Ok(())  // 中文注释：返回成功
}