use bip39::{Language, Mnemonic}; // 移除 Seed 导入
use bip32::Seed; // 添加 bip32::Seed 导入
use blake2::{Blake2b512, Digest};
use sp_core::{Pair, sr25519};
use bs58;

const NETWORK_ID: u32 = 880711; // 4 字节主网 ID

fn main() {
    // 1. 生成 24 个助记词（英文）
    let entropy = rand::random::<[u8; 32]>(); // 生成 32 字节的随机熵
    let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
    println!("助记词: {}", mnemonic.to_string());

    // 2. 从助记词生成 64 字节种子
    let seed = Seed::new(mnemonic.to_seed("")); // Use to_seed() to create the seed
    let seed_bytes = seed.as_bytes();
    
    // 3. 生成 sr25519 密钥对
    let pair = sr25519::Pair::from_seed_slice(seed_bytes).unwrap();
    let public_key = pair.public();
    println!("公钥: 0x{}", hex::encode(public_key.0));

    // 4. 计算钱包地址（主网 ID + 公钥 + 校验码）
    let mut address_data = Vec::new();
    address_data.extend(&NETWORK_ID.to_le_bytes()); // 主网 ID 4 字节
    address_data.extend(&public_key.0); // 公钥 32 字节

    // 5. 计算 2 字节校验码（Blake2b-512 前 2 字节）
    let checksum = Blake2b512::digest(&address_data);
    address_data.extend(&checksum[..2]); // 取前 2 字节

    // 6. Base58 编码钱包地址
    let wallet_address = bs58::encode(&address_data).into_string();
    println!("钱包地址: {}", wallet_address);
}
