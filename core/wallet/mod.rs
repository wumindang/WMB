use crate::wmb_security::{sign, verify};  // 中文注释：引入安全层的签名和验证函数

// 中文注释：定义钱包结构，包含地址
pub struct Wallet {
    pub address: String,  // 中文注释：钱包地址
}

impl Wallet {
    // 中文注释：创建新钱包，传入地址
    pub fn new(address: String) -> Self {
        Wallet { address }  // 中文注释：返回新创建的钱包实例
    }

    // 中文注释：对交易数据进行签名，使用安全层的签名功能
    pub fn sign_transaction(&self, data: &str) -> String {
        sign(data, &self.address)  // 中文注释：调用安全层签名函数，返回签名结果
    }

    // 中文注释：验证交易签名的有效性
    pub fn verify_transaction(&self, data: &str, signature: &str) -> bool {
        verify(data, &self.address, signature)  // 中文注释：调用安全层验证函数
    }
}