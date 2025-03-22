use crate::crypto::signature::generate_keypair; // 导入生成密钥对函数

/// 超级节点信息结构体
#[derive(Debug, Clone)]
pub struct SuperNodeInfo {
    pub node_id: String,         // 节点ID，固定为"SUPER_001"
    pub admins: Vec<String>,     // 5个管理员公钥列表
    pub wallet: String,          // 超级节点唯一钱包地址
    pub is_initialized: bool,    // 是否完成系统初始化（第43个区块后为true）
}

/// 超级节点实现
impl SuperNodeInfo {
    /// 创建超级节点
    /// 参数:
    /// - admins: 5个管理员的公钥列表
    /// 返回: 超级节点实例
    pub fn new(admins: Vec<String>) -> Self {
        let (_, public_key) = generate_keypair(); // 生成密钥对
        let wallet = hex::encode(public_key.serialize()); // 钱包地址为公钥序列化
        Self {
            node_id: "SUPER_001".to_string(),
            admins,
            wallet,
            is_initialized: false, // 初始状态未完成初始化
        }
    }

    /// 验证管理员签名
    /// 参数:
    /// - signatures: 提交的签名列表
    /// 返回: 是否至少3个管理员签名有效
    pub fn verify_signatures(&self, signatures: Vec<String>) -> bool {
        // 检查签名数量是否至少为3，且所有签名都在管理员列表中
        signatures.len() >= 3 && signatures.iter().all(|sig| self.admins.contains(sig))
    }

    /// 初始化系统并生成区块（仅在未初始化时可用）
    /// 参数:
    /// - signatures: 管理员签名列表
    /// - block_number: 当前生成的区块号（0为创世区块，1-43为初始区块）
    /// 返回: 是否成功执行
    pub fn initialize_system(&mut self, signatures: Vec<String>, block_number: u32) -> Result<String, String> {
        if self.is_initialized {
            return Err("系统已初始化，超级节点无权再次执行".to_string());
        }
        if !self.verify_signatures(signatures) {
            return Err("签名验证失败，至少需要3个管理员签名".to_string());
        }

        match block_number {
            0 => Ok("生成创世区块并执行创世发行".to_string()),
            1 => Ok("生成第1个区块，创建国储会、中枢省省储会和省储行节点".to_string()),
            2..=43 => Ok(format!("生成第{}个区块，创建省储会和省储行节点", block_number)),
            _ => {
                self.is_initialized = true; // 第43个区块后标记为已初始化
                Ok("系统初始化完成，超级节点失去生成区块权限".to_string())
            }
        }
    }

    /// 修改其他节点管理员（需初始化完成且有投票授权）
    /// 参数:
    /// - signatures: 管理员签名列表
    /// - target_node_id: 目标节点ID
    /// - new_admins: 新管理员列表
    /// - vote_authorized: 是否获得权威节点投票授权
    /// 返回: 是否成功修改
    pub fn modify_admins(&self, signatures: Vec<String>, target_node_id: String, new_admins: Vec<String>, vote_authorized: bool) -> Result<String, String> {
        if !self.is_initialized {
            return Err("系统未初始化，无法修改管理员".to_string());
        }
        if !self.verify_signatures(signatures) {
            return Err("签名验证失败，至少需要3个管理员签名".to_string());
        }
        if !vote_authorized {
            return Err("未获得权威节点投票授权".to_string());
        }
        Ok(format!("成功修改节点 {} 的管理员为 {:?}", target_node_id, new_admins))
    }

    /// 获取钱包地址
    /// 返回: 超级节点的钱包地址
    pub fn get_wallet(&self) -> String {
        self.wallet.clone()
    }
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_super_node() {
        // 创建超级节点
        let mut super_node = SuperNodeInfo::new(vec![
            "admin1".to_string(),
            "admin2".to_string(),
            "admin3".to_string(),
            "admin4".to_string(),
            "admin5".to_string(),
        ]);

        // 测试节点ID和初始状态
        assert_eq!(super_node.node_id, "SUPER_001");
        assert!(!super_node.is_initialized);
        assert_eq!(super_node.wallet.len(), 66); // 公钥序列化长度

        // 测试签名验证
        let valid_signs = vec!["admin1".to_string(), "admin2".to_string(), "admin3".to_string()];
        let invalid_signs = vec!["admin1".to_string(), "admin2".to_string()];
        assert!(super_node.verify_signatures(valid_signs.clone()));
        assert!(!super_node.verify_signatures(invalid_signs));

        // 测试系统初始化
        assert_eq!(
            super_node.initialize_system(valid_signs.clone(), 0).unwrap(),
            "生成创世区块并执行创世发行"
        );
        assert_eq!(
            super_node.initialize_system(valid_signs.clone(), 1).unwrap(),
            "生成第1个区块，创建国储会、中枢省省储会和省储行节点"
        );
        assert_eq!(
            super_node.initialize_system(valid_signs.clone(), 2).unwrap(),
            "生成第2个区块，创建省储会和省储行节点"
        );
        assert_eq!(
            super_node.initialize_system(valid_signs.clone(), 43).unwrap(),
            "生成第43个区块，创建省储会和省储行节点"
        );
        assert_eq!(
            super_node.initialize_system(valid_signs.clone(), 44).unwrap(),
            "系统初始化完成，超级节点失去生成区块权限"
        );
        assert!(super_node.is_initialized);
        assert!(super_node.initialize_system(valid_signs.clone(), 45).is_err());

        // 测试修改管理员
        assert_eq!(
            super_node.modify_admins(valid_signs.clone(), "NATIONAL_001".to_string(), vec!["new_admin".to_string()], true).unwrap(),
            "成功修改节点 NATIONAL_001 的管理员为 [\"new_admin\"]"
        );
        assert!(super_node.modify_admins(valid_signs.clone(), "NATIONAL_001".to_string(), vec![], false).is_err());
    }
}