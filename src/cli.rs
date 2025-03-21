// 文件路径: src/cli.rs
// 作用: 命令行工具，支持所有功能

use clap::{Args, Parser, Subcommand};        // 命令行解析
use crate::nodes::{NodeManager, NodeType};   // 节点管理
use crate::crypto::Crypto;                   // 加密功能
use crate::network::P2pNetwork;              // 网络功能
use crate::blockchain::Transaction;          // 交易结构
use libp2p::identity;                        // 身份管理

// 命令行主结构
#[derive(Parser)]
#[command(name = "wmb", about = "五民币区块链命令行工具", version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands, // 子命令
}

// 子命令枚举
#[derive(Subcommand)]
enum Commands {
    Keygen,           // 生成密钥对
    Sign(SignArgs),   // 签名交易
    Broadcast(BroadcastArgs), // 广播交易
    Nodes(NodesArgs), // 管理节点
}

// 签名参数
#[derive(Args)]
struct SignArgs {
    sender: String,      // 发送者地址
    receiver: String,    // 接收者地址
    amount: f64,         // 金额
    fee: f64,            // 费用
    private_key: String, // 私钥（十六进制）
}

// 广播参数
#[derive(Args)]
struct BroadcastArgs {
    sender: String,      // 发送者地址
    receiver: String,    // 接收者地址
    amount: f64,         // 金额
    fee: f64,            // 费用
    signature: String,   // 签名（十六进制）
}

// 节点管理参数
#[derive(Args)]
struct NodesArgs {
    #[command(subcommand)]
    action: NodeAction, // 节点操作
}

// 节点操作子命令
#[derive(Subcommand)]
enum NodeAction {
    Add {                  // 添加节点
        peer_id: String,   // 节点 Peer ID
        address: String,   // 节点地址
        #[arg(value_enum)]
        node_type: CliNodeType, // 节点类型
    },
    Remove { peer_id: String }, // 移除节点
    List,                       // 列出所有节点
}

// CLI 节点类型枚举（用于解析命令行输入）
#[derive(clap::ValueEnum, Clone)]
enum CliNodeType {
    CitizenLight,       // 公民轻节点
    GuestLight,         // 访客轻节点
    Super,              // 超级节点
    NationalAuthority,  // 国储会权威节点
    ProvincialAuthority,// 省储会权威节点
    ProvincialEquity,   // 省储行权益节点
    Full,               // 全节点
}

// 将 CLI 节点类型转换为内部 NodeType
impl From<CliNodeType> for NodeType {
    fn from(cli_type: CliNodeType) -> Self {
        match cli_type {
            CliNodeType::CitizenLight => NodeType::CitizenLight,
            CliNodeType::GuestLight => NodeType::GuestLight,
            CliNodeType::Super => NodeType::Super,
            CliNodeType::NationalAuthority => NodeType::NationalAuthority,
            CliNodeType::ProvincialAuthority => NodeType::ProvincialAuthority,
            CliNodeType::ProvincialEquity => NodeType::ProvincialEquity,
            CliNodeType::Full => NodeType::Full,
        }
    }
}

// 运行 CLI
pub fn run(manager: NodeManager) {
    let cli = Cli::parse(); // 解析命令行参数

    match cli.command {
        Commands::Keygen => { // 生成密钥对
            let crypto = Crypto::new();
            let (secret_key, public_key) = crypto.generate_keypair();
            println!("私钥: {}", hex::encode(secret_key.secret_bytes()));
            println!("公钥: {}", hex::encode(public_key.serialize()));
        }
        Commands::Sign(args) => { // 签名交易
            let crypto = Crypto::new();
            let private_key = hex::decode(&args.private_key).expect("无效的私钥");
            let secret_key = secp256k1::SecretKey::from_slice(&private_key).expect("无效的私钥长度");
            let mut tx = Transaction {
                sender: args.sender,
                receiver: args.receiver,
                amount: args.amount,
                fee: args.fee,
                signature: Vec::new(),
            };
            crypto.sign_transaction(&mut tx, &secret_key);
            println!("签名: {}", hex::encode(&tx.signature));
        }
        Commands::Broadcast(args) => { // 广播交易
            let keypair = identity::Keypair::generate_ed25519(); // 生成网络身份
            let mut network = P2pNetwork::new(keypair).expect("网络初始化失败");
            network.listen("/ip4/0.0.0.0/tcp/0").expect("监听失败");

            let tx = Transaction {
                sender: args.sender,
                receiver: args.receiver,
                amount: args.amount,
                fee: args.fee,
                signature: hex::decode(args.signature).expect("无效的签名"),
            };
            network.broadcast_transaction(tx).expect("广播失败");
            println!("交易广播成功");
        }
        Commands::Nodes(args) => match args.action { // 节点管理
            NodeAction::Add { peer_id, address, node_type } => {
                if let Err(e) = manager.add_node(&peer_id, &address, node_type.into()) {
                    println!("错误: {}", e);
                } else {
                    println!("节点添加成功: {}", peer_id);
                }
            }
            NodeAction::Remove { peer_id } => {
                if let Err(e) = manager.remove_node(&peer_id) {
                    println!("错误: {}", e);
                } else {
                    println!("节点移除成功: {}", peer_id);
                }
            }
            NodeAction::List => {
                if let Ok(json) = manager.to_json() {
                    println!("节点列表: {}", json);
                } else {
                    println!("获取节点列表失败");
                }
            }
        },
    }
}