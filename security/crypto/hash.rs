use sha2::{Digest, Sha256}; // 引入SHA256哈希算法

/// 计算SHA256哈希值
/// 参数：
/// - data: 输入字符串
/// 返回：哈希值的16进制字符串
pub fn sha256_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        let input = "Wuminbi Blockchain";
        let hash = sha256_hash(input);
        assert_eq!(hash.len(), 64); // SHA256输出长度为64个16进制字符
    }
}