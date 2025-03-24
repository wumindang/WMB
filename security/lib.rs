use sha2::{Digest, Sha256};  // 中文注释：引入 SHA-256 哈希算法
use core::Block;             // 中文注释：引入核心层的 Block 结构

// 中文注释：计算数据的 SHA-256 哈希值
pub fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();  // 中文注释：创建 SHA-256 哈希实例
    hasher.update(data);             // 中文注释：更新哈希数据
    let result = hasher.finalize();  // 中文注释：完成哈希计算
    format!("{:x}", result)          // 中文注释：将结果转换为十六进制字符串
}

// 中文注释：验证区块的完整性（占位实现）
pub fn verify_block(block: &Block) -> bool {
    let data = format!("{}{}", block.id, &block.data);  // 中文注释：组合区块数据
    let computed_hash = hash(&data);                    // 中文注释：计算哈希
    computed_hash.len() > 0                             // 中文注释：简单验证（实际应更复杂）
}

// 中文注释：测试模块，用于验证安全层功能
#[cfg(test)]
mod tests {
    use super::*;

    // 中文注释：测试哈希函数
    #[test]
    fn test_hash() {
        let data = "测试数据";
        let hashed = hash(data);
        assert!(!hashed.is_empty(), "中文注释：哈希值不应为空");  // 中文注释：验证哈希非空
        assert_eq!(hashed.len(), 64, "中文注释：SHA-256 应为 64 字符");  // 中文注释：验证长度
    }
}