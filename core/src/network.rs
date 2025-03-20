// 文件路径: WMB/core/src/network.rs

use libp2p::{identity, PeerId, Transport};
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::floodsub::{Floodsub, Topic, FloodsubEvent};
use libp2p::tcp::async_io::Transport as TcpTransport;
use libp2p::core::upgrade;
use libp2p::noise;
use libp2p::yamux;
use libp2p::Multiaddr;
use serde::{Serialize, Deserialize};
use futures::executor::block_on;
use std::error::Error;
use crate::blockchain::{Block, Transaction};

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkMessage {
    NewBlock(Block),
    NewTransaction(Transaction),
}

pub struct P2pNetwork {
    swarm: Swarm<Floodsub>,
    topic: Topic,
}

impl P2pNetwork {
    pub fn new(local_key: identity::Keypair) -> Result<Self, Box<dyn Error>> {
        let local_peer_id = PeerId::from(local_key.public());
        let transport = TcpTransport::new(libp2p::tcp::Config::default())
            .upgrade(upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key)?)
            .multiplex(yamux::Config::default())
            .boxed();

        let behaviour = Floodsub::new(local_peer_id);
        let mut swarm = Swarm::with_async_std_executor(transport, behaviour, local_peer_id);
        let topic = Topic::new("wmb-global-topic");
        swarm.behaviour_mut().subscribe(topic.clone());

        Ok(P2pNetwork { swarm, topic })
    }

    pub fn listen(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
        let multiaddr: Multiaddr = addr.parse()?;
        self.swarm.listen_on(multiaddr)?;
        Ok(())
    }

    pub fn broadcast_block(&mut self, block: Block) -> Result<(), Box<dyn Error>> {
        let message = NetworkMessage::NewBlock(block);
        let serialized = bincode::serialize(&message)?;
        self.swarm.behaviour_mut().publish(self.topic.clone(), serialized);
        Ok(())
    }

    pub fn broadcast_transaction(&mut self, tx: Transaction) -> Result<(), Box<dyn Error>> {
        let message = NetworkMessage::NewTransaction(tx);
        let serialized = bincode::serialize(&message)?;
        self.swarm.behaviour_mut().publish(self.topic.clone(), serialized);
        Ok(())
    }

    pub fn poll_events(&mut self) -> Option<NetworkMessage> {
        use futures::StreamExt;
        if let Some(event) = block_on(futures::future::poll_fn(|cx| self.swarm.poll_next_unpin(cx))) {
            match event {
                SwarmEvent::Behaviour(FloodsubEvent::Message(msg)) => {
                    bincode::deserialize(&msg.data).ok()
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2p_network() {
        let keypair = identity::Keypair::generate_ed25519();
        let mut network = P2pNetwork::new(keypair).expect("网络初始化失败");
        network.listen("/ip4/0.0.0.0/tcp/0").expect("监听失败");

        let tx = Transaction {
            sender: "addr1".to_string(),
            receiver: "addr2".to_string(),
            amount: 10.00,
            fee: 0.50,
            signature: vec![0x01, 0x02, 0x03],
        };

        network.broadcast_transaction(tx.clone()).expect("广播交易失败");

        let block = Block::new("0000".to_string(), vec![tx]);
        network.broadcast_block(block).expect("广播区块失败");

        println!("网络测试完成");
    }
}