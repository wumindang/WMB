/// 加密模块，提供哈希和签名功能
pub mod hash;
pub mod signature;

pub use hash::sha256_hash;
pub use signature::{generate_keypair, sign, verify};