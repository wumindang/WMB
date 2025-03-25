use super_node_broadcast::{SuperNodeBroadcast, Peer, Block, Transaction};
use other_nodes_broadcast::{OtherNodesBroadcast, NodeRole};
use std::thread;
use std::time::Duration;

fn main() {
    // 初始化对等节点列表
    let peers = vec![
        Peer { id: "nrc_node_1".to_string(), address: "192.168.1.101:5001".to_string() },
        Peer { id: "prc_node_1".to_string(), address: "192.168.1.102:5001".to_string() },
    ];

    // 测试超级节点广播
    println!("开始测试超级节点广播...");
    let mut super_bm = SuperNodeBroadcast::new("super_node_1".to_string(), peers.clone());
    let block = Block { height: 1, data: vec![1, 2, 3] };
    super_bm.broadcast_block(block.clone()); // 初始化时广播
    super_bm.set_initialized();              // 设置初始化完成
    super_bm.broadcast_block(block);         // 不再广播

    // 测试其他归档节点广播
    println!("开始测试其他归档节点广播...");
    let full_bm = OtherNodesBroadcast::new(NodeRole::FullNode, "full_node_1".to_string(), peers.clone());
    let tx = Transaction { id: "tx1".to_string(), data: vec![4, 5, 6] };
    full_bm.broadcast_transaction(tx);

    let nrc_bm = OtherNodesBroadcast::new(NodeRole::NrcNode, "nrc_node_1".to_string(), peers);
    let block2 = Block { height: 2, data: vec![7, 8, 9] };
    nrc_bm.broadcast_block(block2);

    thread::sleep(Duration::from_millis(500)); // 等待所有广播完成
    println!("WMB 广播功能测试完成");
}