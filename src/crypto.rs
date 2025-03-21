// 文件路径: src/crypto.rs
// 作用: 处理密钥生成和交易签名

use secp256k1::{Message, PublicKey, SecretKey, Secp256k1, SignOnly}; // 椭圆曲线加密
use sha3::{Digest, Keccak256}; // Keccak256 哈希
use crate::blockchain::Transaction; // 交易结构

// 加密管理
pub struct Crypto {
    secp: Secp256k1<SignOnly>, // Secp256k1 实例（仅签名）
}

impl Crypto {
    // 创建新加密实例
    pub fn new() -> Self {
        Crypto {
            secp: Secp256k1::signing_only(),
        }
    }

    // 生成密钥对
    pub fn generate_keypair(&self) -> (SecretKey, PublicKey) {
        let mut rng = rand::thread_rng(); // 获取随机数生成器
        let mut secret_bytes = [0u8; 32]; // 创建 32 字节的私钥数组
        rand::Rng::fill(&mut rng, &mut secret_bytes); // 填充随机字节
        let secret_key = SecretKey::from_slice(&secret_bytes).expect("私钥生成失败"); // 从字节生成私钥
        let public_key = PublicKey::from_secret_key(&self.secp, &secret_key); // 由私钥生成公钥
        (secret_key, public_key)
    }

    // 签名交易
    pub fn sign_transaction(&self, tx: &mut Transaction, secret_key: &SecretKey) {
        let message = Self::hash_transaction(tx); // 计算交易哈希
        let signature = self.secp.sign_ecdsa(&message, secret_key); // 使用私钥签名
        tx.signature = signature.serialize_compact().to_vec(); // 保存签名
    }

    // 计算交易哈希
    fn hash_transaction(tx: &Transaction) -> Message {
        let input = format!("{}{}{}{}", tx.sender, tx.receiver, tx.amount, tx.fee); // 拼接交易数据
        let mut hasher = Keccak256::new(); // 创建哈希器
        hasher.update(input); // 输入数据
        Message::from_slice(&hasher.finalize()).expect("哈希长度必须为 32 字节") // 返回消息
    }
}