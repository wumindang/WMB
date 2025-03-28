use bip32::{XPrv, DerivationPath};
use bip39::{Mnemonic as Bip39Mnemonic, Language};
use secp256k1::Secp256k1;
use tiny_keccak::{Hasher, Keccak};
use std::str::FromStr;

pub struct PrbWallet {
    mnemonic: Bip39Mnemonic, // 助记词
    seed: Vec<u8>,           // 种子
    node_id: String,         // 节点唯一标识
    account_index: u32,      // 账户索引
}

impl PrbWallet {
    /// 创建新的省储行权益节点钱包
    pub fn new(node_id: String, account_index: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let mnemonic = Bip39Mnemonic::generate((Language::English, 12))?;
        let seed = mnemonic.to_seed("");
        Ok(PrbWallet {
            mnemonic,
            seed: seed.to_vec(),
            node_id,
            account_index,
        })
    }

    /// 获取助记词
    pub fn get_mnemonic(&self) -> String {
        self.mnemonic.to_string()
    }

    /// 生成交易地址（仅 1 个）
    pub fn generate_transaction_address(&self) -> Result<String, Box<dyn std::error::Error>> {
        // 路径：m/44'/888'/<account>'/0/0
        let path = format!("m/44'/888'/{}'/0/0", self.account_index);
        let secp = Secp256k1::new();
        let derivation_path = DerivationPath::from_str(&path)?;
        let root = XPrv::new(&self.seed)?;
        let child = root.derive_child(derivation_path)?;
        let public_key = child.public_key();
        let pub_bytes = public_key.serialize_uncompressed()[1..].to_vec();
        let mut hasher = Keccak::v256();
        hasher.update(&pub_bytes);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        Ok(format!("0x{}", hex::encode(&hash[12..])))
    }

    /// 生成质押地址（仅 1 个，受限）
    pub fn generate_staking_address(&self) -> Result<String, Box<dyn std::error::Error>> {
        // 路径：m/44'/888'/<account>'/0/1
        let path = format!("m/44'/888'/{}'/0/1", self.account_index);
        let secp = Secp256k1::new();
        let derivation_path = DerivationPath::from_str(&path)?;
        let root = XPrv::new(&self.seed)?;
        let child = root.derive_child(derivation_path)?;
        let public_key = child.public_key();
        let pub_bytes = public_key.serialize_uncompressed()[1..].to_vec();
        let mut hasher = Keccak::v256();
        hasher.update(&pub_bytes);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        Ok(format!("0x{}", hex::encode(&hash[12..])))
    }
}