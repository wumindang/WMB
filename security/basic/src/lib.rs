// 中文注释：基础安全模块，为公民轻节点和访客轻节点提供轻量级安全功能
pub mod crypto {
    use core::Core;  // 中文注释：引入核心层模块

    // 中文注释：生成 ECDSA 密钥对
    pub fn generate_keypair() -> (String, String) {
        let core = Core::new();
        core.generate_ecdsa_keypair()  // 中文注释：生成公钥和私钥
    }

    // 中文注释：对数据进行 ECDSA 签名
    pub fn sign(data: &str, private_key: &str) -> String {
        let core = Core::new();
        core.sign_ecdsa(data, private_key)  // 中文注释：返回签名
    }

    // 中文注释：验证 ECDSA 签名
    pub fn verify(data: &str, signature: &str, public_key: &str) -> bool {
        let core = Core::new();
        core.verify_ecdsa(data, signature, public_key)  // 中文注释：返回验证结果
    }

    // 中文注释：AES 加密数据
    pub fn encrypt(data: &str, key: &str) -> String {
        let core = Core::new();
        core.encrypt_aes(data, key)  // 中文注释：返回加密数据
    }
}