// 文件路径: src/network.rs
// 作用: 实现 P2P 网络，支持交易广播

use libp2p::{
    floodsub::{Floodsub, FloodsubEvent}, // 广播协议
    identity,                            // 身份管理
    mdns::{Mdns, Config as MdnsConfig, Event as MdnsEvent}, // mDNS 发现（移除 async_io 前缀）
    noise,                               // Noise 加密协议
    swarm::{Swarm, SwarmBuilder, SwarmEvent}, // Swarm 构建器和事件
    tcp,                                 // TCP 传输
    yamux,                               // Yamux 多路复用
    Multiaddr, PeerId, Transport,        // 地址、对等 ID 和 Transport trait
};
use futures::StreamExt;                  // 异步流支持
use crate::blockchain::Transaction;      // 交易结构

// 自定义网络事件枚举
#[derive(Debug)]
enum NetworkEvent {
    Floodsub(FloodsubEvent),
    Mdns(MdnsEvent),
}

// 实现 FloodsubEvent 到 NetworkEvent 的转换
impl From<FloodsubEvent> for NetworkEvent {
    fn from(event: FloodsubEvent) -> Self {
        NetworkEvent::Floodsub(event)
    }
}

// 实现 MdnsEvent 到 NetworkEvent 的转换
impl From<MdnsEvent> for NetworkEvent {
    fn from(event: MdnsEvent) -> Self {
        NetworkEvent::Mdns(event)
    }
}

// 自定义行为结构体，组合 Floodsub 和 Mdns
#[derive(libp2p::swarm::NetworkBehaviour)]
#[behaviour(out_event = "NetworkEvent")]
struct NetworkBehaviour {
    floodsub: Floodsub, // 广播协议
    mdns: Mdns,         // mDNS 发现
}

// P2P 网络管理
pub struct P2pNetwork {
    swarm: Swarm<NetworkBehaviour>, // Swarm 实例，管理网络行为
}

impl P2pNetwork {
    // 创建新 P2P 网络实例
    pub fn new(keypair: identity::Keypair) -> Result<Self, String> {
        let local_peer_id = PeerId::from(keypair.public()); // 从密钥对生成本地 Peer ID

        // 构建 Noise 配置
        let noise_config = noise::Config::new(&keypair)
            .map_err(|e| format!("Noise 配置失败: {}", e))?;

        // 手动构建传输层（拆分步骤）
        let base_transport = tcp::async_io::Transport::new(tcp::Config::default());
        let upgraded_transport = base_transport.upgrade(libp2p::core::upgrade::Version::V1);
        let auth_transport = upgraded_transport.authenticate(noise_config);
        let mux_transport = auth_transport.multiplex(yamux::Config::default());
        let transport = mux_transport.boxed();

        let floodsub = Floodsub::new(local_peer_id); // 初始化广播协议
        let mdns = Mdns::new(MdnsConfig::default(), local_peer_id)
            .map_err(|e| format!("mDNS 初始化失败: {}", e))?; // 初始化 mDNS 发现
        let behaviour = NetworkBehaviour { floodsub, mdns }; // 组合行为

        let swarm = SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id)
            .build(); // 使用 SwarmBuilder 构建 Swarm
        Ok(P2pNetwork { swarm })
    }

    // 开始监听指定地址
    pub fn listen(&mut self, addr: &str) -> Result<(), String> {
        let multiaddr: Multiaddr = addr.parse().map_err(|e| format!("地址解析失败: {}", e))?; // 解析多地址
        self.swarm.listen_on(multiaddr).map_err(|e| e.to_string())?; // 开始监听
        Ok(())
    }

    // 广播交易
    pub fn broadcast_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        let topic = libp2p::floodsub::Topic::new("wmb-transactions"); // 定义广播主题
        let tx_bytes = bincode::serialize(&tx).map_err(|e| e.to_string())?; // 序列化交易
        self.swarm.behaviour_mut().floodsub.publish(topic, tx_bytes); // 发布交易
        Ok(())
    }

    // 运行网络事件循环
    pub async fn run(&mut self) {
        while let Some(event) = self.swarm.next().await { // 监听网络事件
            match event {
                SwarmEvent::Behaviour(NetworkEvent::Floodsub(FloodsubEvent::Message(msg))) => { // 收到广播消息
                    if let Ok(tx) = bincode::deserialize::<Transaction>(&msg.data) {
                        println!("收到交易: {:?}", tx); // 打印收到的交易
                    }
                }
                SwarmEvent::Behaviour(NetworkEvent::Floodsub(FloodsubEvent::Subscribed { peer_id, .. })) => { // 订阅事件
                    println!("节点订阅: {}", peer_id);
                }
                _ => {}
            }
        }
    }
}