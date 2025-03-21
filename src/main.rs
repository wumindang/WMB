// 文件路径: src/main.rs
// 作用: 项目主入口

// 引入模块
mod blockchain;  // 区块链模块
mod consensus;   // 共识模块
mod contracts;   // 智能合约模块
mod crypto;      // 加密模块
mod network;     // 网络模块
mod nodes;       // 节点管理模块
mod cli;         // 命令行模块

// 主函数
fn main() {
    let node_manager = nodes::NodeManager::new(); // 创建节点管理器
    cli::run(node_manager); // 运行命令行工具
}