// 文件路径: WMB/core/src/crypto.rs

use rand::{RngCore, rngs::OsRng};
use secp256k1::{Message, Secp256k1, SecretKey, PublicKey, SignOnly, VerifyOnly};
use sha2::{Sha256, Digest};
use crate::blockchain::Transaction;

pub struct Crypto {
    secp_sign: Secp256k1<SignOnly>,
    secp_verify: Secp256k1<VerifyOnly>,
}

impl Crypto {
    pub fn new() -> Self {
        Crypto {
            secp_sign: Secp256k1::signing_only(),
            secp_verify: Secp256k1::verification_only(),
        }
    }

    pub fn generate_keypair(&self) -> (SecretKey, PublicKey) {
        let mut rng = OsRng;
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);
        let secret_key = SecretKey::from_slice(&seed).expect("32 bytes, within curve order");
        let public_key = PublicKey::from_secret_key(&self.secp_sign, &secret_key);
        (secret_key, public_key)
    }

    pub fn sign_transaction(&self, tx: &mut Transaction, secret_key: &SecretKey) {
        let tx_hash = Self::hash_transaction(tx);
        let message = Message::from_slice(&tx_hash).expect("交易哈希长度必须为 32 字节");
        let signature = self.secp_sign.sign_ecdsa(&message, secret_key);
        tx.signature = signature.serialize_compact().to_vec();
    }

    pub fn verify_transaction(&self, tx: &Transaction, public_key: &PublicKey) -> bool {
        let tx_hash = Self::hash_transaction(tx);
        let message = match Message::from_slice(&tx_hash) {
            Ok(msg) => msg,
            Err(_) => return false,
        };
        let signature = match secp256k1::ecdsa::Signature::from_compact(&tx.signature) {
            Ok(sig) => sig,
            Err(_) => return false,
        };
        self.secp_verify.verify_ecdsa(&message, &signature, public_key).is_ok()
    }

    fn hash_transaction(tx: &Transaction) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&tx.sender);
        hasher.update(&tx.receiver);
        hasher.update(tx.amount.to_string());
        hasher.update(tx.fee.to_string());
        let result = hasher.finalize();
        result.into()
    }
}

#[cfg(test)]
mod Secp256k1 {
    use super::*;

    #[test]
    fn test_ecdsa_sign_and_verify() {
        let crypto = Crypto::new();
        let (secret_key, public_key) = crypto.generate_keypair();

        let mut tx = Transaction {
            sender: "addr1".to_string(),
            receiver: "addr2".to_string(),
            amount: 10.00,
            fee: 0.50,
            signature: Vec::new(),
        };

        crypto.sign_transaction(&mut tx, &secret_key);
        assert!(!tx.signature.is_empty());

        let is_valid = crypto.verify_transaction(&tx, &public_key);
        assert!(is_valid);

        println!("签名: {:?}", tx.signature);
    }
}