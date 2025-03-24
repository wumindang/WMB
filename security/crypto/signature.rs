use secp256k1::{SecretKey, PublicKey, Secp256k1, Message};
use secp256k1::ecdsa::Signature; // 使用推荐的类型
use crate::crypto::hash::sha256_hash; // 引入哈希函数

/// 生成密钥对
/// 返回：(私钥, 公钥)
pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    secp.generate_keypair(&mut rand::thread_rng())
}

/// 对消息签名
/// 参数：
/// - message: 要签名的消息
/// - secret_key: 私钥
/// 返回：签名
pub fn sign(message: &str, secret_key: &SecretKey) -> Signature {
    let secp = Secp256k1::new();
    let msg_hash = sha256_hash(message);
    let msg = Message::from_slice(&hex::decode(msg_hash).unwrap()).unwrap();
    secp.sign_ecdsa(&msg, secret_key)
}

/// 验证签名
/// 参数：
/// - message: 原始消息
/// - signature: 签名
/// - public_key: 公钥
/// 返回：签名是否有效
pub fn verify(message: &str, signature: &Signature, public_key: &PublicKey) -> bool {
    let secp = Secp256k1::new();
    let msg_hash = sha256_hash(message);
    let msg = Message::from_slice(&hex::decode(msg_hash).unwrap()).unwrap();
    secp.verify_ecdsa(&msg, signature, public_key).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify() {
        let (sk, pk) = generate_keypair();
        let message = "Test transaction";
        let signature = sign(message, &sk);
        assert!(verify(message, &signature, &pk));
    }
}