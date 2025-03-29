use bip32::{Mnemonic, XPrv, DerivationPath, ChildNumber}; // 确保 `ChildNumber` 被正确引入
use bip39::{Mnemonic as Bip39Mnemonic, Language};
use secp256k1::Secp256k1;
use tiny_keccak::{Hasher, Keccak};
use std::str::FromStr;
use super::NodeType;
use rand::thread_rng;

pub struct HDWallet {
    mnemonic: Bip39Mnemonic, // 助记词
    seed: Vec<u8>,           // 种子
    node_type: NodeType,     // 节点类型
    node_id: String,         // 节点唯一标识
    account_index: u32,      // 账户索引，用于区分不同节点
}

impl HDWallet {
    /// 创建新的 HD 钱包，仅限全节点和访客轻节点使用
    pub fn new(node_type: NodeType, node_id: String, account_index: u32) -> Result<Self, Box<dyn std::error::Error>> {
        // 检查节点类型是否符合要求
        if node_type != NodeType::FullNode && node_type != NodeType::GuestNode {
            return Err("HDWallet 仅适用于 FullNode 和 GuestNode".into());
        }

        // 生成 12 个单词的助记词
        let mnemonic = Bip39Mnemonic::generate(Language::English, 12)?;
        
        // 从助记词生成种子
        let seed = mnemonic.to_seed("");

        Ok(HDWallet {
            mnemonic,
            seed: seed.to_vec(),
            node_type,
            node_id,
            account_index,
        })
    }

    /// 获取助记词，用于备份和恢复钱包
    pub fn get_mnemonic(&self) -> String {
        self.mnemonic.to_string()
    }

    /// 生成交易地址，支持无限地址生成
    pub fn generate_transaction_address(&self, index: u32) -> Result<String, Box<dyn std::error::Error>> {
        // 使用 BIP-44 路径：m/44'/888'/<account>'/0/<index>
        let path = format!("m/44'/888'/{}'/0/{}", self.account_index, index);
        let secp = Secp256k1::new();

        // 解析派生路径
        let derivation_path = DerivationPath::from_str(&path)?;
        
        // 从种子生成根私钥
        let root = XPrv::new(&self.seed)?;

        // 获取路径的第一个子路径
        let child = root.derive_child(derivation_path[0])?;

        // 获取公钥
        let public_key = child.public_key();

        // 转换公钥为字节格式
        let pub_bytes = public_key.to_bytes().to_vec(); // 修正 `serialize_uncompressed`

        // 计算 Keccak 哈希
        let mut hasher = Keccak::v256();
        hasher.update(&pub_bytes);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);

        // 生成以太坊兼容地址（取后 20 字节）
        Ok(format!("0x{}", hex::encode(&hash[12..])))
    }
}