use wmb_interface::Interface;  // 中文注释：引入接口层功能
use clap::Parser;  // 中文注释：引入命令行解析工具

// 中文注释：定义命令行参数结构
#[derive(Parser)]
#[command(name = "wmb-check-blocks", about = "查询 WMB 区块数量")]
struct Cli {
    #[arg(long, default_value = "blockchain.dat")]
    storage_path: String,  // 中文注释：存储路径
}

// 中文注释：区块查询工具主函数
fn main() -> std::io::Result<()> {
    let cli = Cli::parse();  // 中文注释：解析命令行参数

    let interface = Interface::new(&cli.storage_path, "127.0.0.1:8088");  // 中文注释：创建接口实例
    let count = interface.get_block_count();  // 中文注释：获取区块数量
    println!("中文注释：当前区块数量: {}", count);

    Ok(())  // 中文注释：返回成功
}