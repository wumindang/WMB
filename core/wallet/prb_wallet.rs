use bip32::{XPrv, DerivationPath};
use bip39::{Mnemonic as Bip39Mnemonic, Language};
use secp256k1::Secp256k1;
use tiny_keccak::{Hasher, Keccak};
use std::str::FromStr;
use rand::thread_rng;

/// 省储行权益节点钱包
pub struct PrbWallet {
    mnemonic: Bip39Mnemonic, // 助记词，用户可用此恢复钱包
    seed: Vec<u8>,           // 由助记词派生的种子
    node_id: String,         // 唯一标识此钱包所属的节点
    account_index: u32,      // 账户索引，区分不同的钱包实例
}

impl PrbWallet {
    /// 创建新的省储行权益节点钱包
    /// - `node_id`: 该钱包对应的节点 ID
    /// - `account_index`: 账户索引，决定 BIP-44 派生路径
    pub fn new(node_id: String, account_index: u32) -> Result<Self, Box<dyn std::error::Error>> {
        // 生成一个新的助记词（12 个单词）
        let mnemonic = Bip39Mnemonic::generate((Language::English, 12))?;
        // 通过助记词生成种子
        let seed = mnemonic.to_seed("");
        Ok(PrbWallet {
            mnemonic,
            seed: seed.to_vec(),
            node_id,
            account_index,
        })
    }

    /// 获取助记词（用于备份和恢复钱包）
    pub fn get_mnemonic(&self) -> String {
        self.mnemonic.to_string()
    }

    /// 生成唯一的交易地址
    /// - 采用 BIP-44 规范，路径格式为 `m/44'/888'/account'/0/0`
    pub fn generate_transaction_address(&self) -> Result<String, Box<dyn std::error::Error>> {
        let path = format!("m/44'/888'/{}'/0/0", self.account_index);
        let secp = Secp256k1::new();
        let derivation_path = DerivationPath::from_str(&path)?;
        
        // 从种子派生根私钥
        let root = XPrv::derive_from_path(&self.seed, &derivation_path)?;
        // 从根私钥派生交易地址对应的私钥
        let child = root.derive_priv(&secp, &derivation_path)?;
        
        // 获取公钥，并去掉前缀字节
        let public_key = child.public_key();
        let pub_bytes = public_key.serialize_uncompressed()[1..].to_vec();
        
        // 计算 Keccak 哈希，取后 20 字节生成以太坊兼容地址
        let mut hasher = Keccak::v256();
        hasher.update(&pub_bytes);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        Ok(format!("0x{}", hex::encode(&hash[12..])))
    }

    /// 生成唯一的质押地址
    /// - 采用 BIP-44 规范，路径格式为 `m/44'/888'/account'/0/1`
    pub fn generate_staking_address(&self) -> Result<String, Box<dyn std::error::Error>> {
        let path = format!("m/44'/888'/{}'/0/1", self.account_index);
        let secp = Secp256k1::new();
        let derivation_path = DerivationPath::from_str(&path)?;
        
        // 从种子派生根私钥
        let root = XPrv::derive_from_path(&self.seed, &derivation_path)?;
        // 从根私钥派生质押地址对应的私钥
        let child = root.derive_priv(&secp, &derivation_path)?;
        
        // 获取公钥，并去掉前缀字节
        let public_key = child.public_key();
        let pub_bytes = public_key.serialize_uncompressed()[1..].to_vec();
        
        // 计算 Keccak 哈希，取后 20 字节生成以太坊兼容地址
        let mut hasher = Keccak::v256();
        hasher.update(&pub_bytes);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        Ok(format!("0x{}", hex::encode(&hash[12..])))
    }
}