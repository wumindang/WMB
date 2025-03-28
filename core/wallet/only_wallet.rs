use bip32::{Mnemonic, XPrv, DerivationPath};
use bip39::{Mnemonic as Bip39Mnemonic, Language};
use secp256k1::Secp256k1;
use tiny_keccak::{Hasher, Keccak};
use std::str::FromStr;
use super::NodeType;

pub struct OnlyWallet {
    mnemonic: Bip39Mnemonic, // 助记词
    seed: Vec<u8>,           // 种子
    node_type: NodeType,     // 节点类型
    node_id: String,         // 节点唯一标识
    account_index: u32,      // 账户索引
}

impl OnlyWallet {
    /// 创建新的单一地址钱包，仅限超级节点、国储会、省储会、公民轻节点
    pub fn new(node_type: NodeType, node_id: String, account_index: u32) -> Result<Self, Box<dyn std::error::Error>> {
        // 检查节点类型
        if node_type == NodeType::FullNode || node_type == NodeType::GuestNode || node_type == NodeType::ProvincialBankNode {
            return Err("OnlyWallet 不适用于 FullNode、GuestNode 或 ProvincialBankNode".into());
        }
        let mnemonic = Bip39Mnemonic::random(&mut rand::thread_rng(), Language::English)?; // 生成助记词
        let seed = mnemonic.to_seed(""); // 生成种子
        Ok(OnlyWallet {
            mnemonic,
            seed: seed.to_vec(),
            node_type,
            node_id,
            account_index,
        })
    }

    /// 获取助记词
    pub fn get_mnemonic(&self) -> String {
        self.mnemonic.phrase().to_string()
    }

    /// 生成唯一的交易地址
    pub fn generate_transaction_address(&self) -> Result<String, Box<dyn std::error::Error>> {
        // 固定路径：m/44'/888'/<account>'/0/0
        let path = format!("m/44'/888'/{}'/0/0", self.account_index);
        let secp = Secp256k1::new();
        let derivation_path = DerivationPath::from_str(&path)?;
        let root = XPrv::derive_from_path(&self.seed, &derivation_path)?; // 修正私钥派生方法
        let child = root.derive_priv(&secp, &derivation_path)?;
        let public_key = child.public_key();
        let pub_bytes = public_key.serialize_uncompressed()[1..].to_vec();
        let mut hasher = Keccak::v256();
        hasher.update(&pub_bytes);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        Ok(format!("0x{}", hex::encode(&hash[12..])))
    }
}