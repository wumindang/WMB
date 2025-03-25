use crate::crypto::signature::generate_keypair; // 导入生成密钥对函数
use chrono::{Utc, Duration}; // 导入时间处理库

/// 国储会权威节点信息结构体
#[derive(Debug, Clone)]
pub struct NationalAuthorityInfo {
    pub node_id: String,         // 节点ID，固定为"NATIONAL_001"
    pub admins: Vec<String>,     // 19个管理员公钥列表
    pub wallet: String,          // 国储会唯一钱包地址
}

/// 国储会权威节点实现
impl NationalAuthorityInfo {
    /// 创建国储会权威节点
    /// 参数:
    /// - admins: 19个管理员的公钥列表
    /// 返回: 国储会权威节点实例
    pub fn new(admins: Vec<String>) -> Self {
        let (_, public_key) = generate_keypair(); // 生成密钥对
        let wallet = hex::encode(public_key.serialize()); // 钱包地址为公钥序列化
        Self {
            node_id: "NATIONAL_001".to_string(),
            admins,
            wallet,
        }
    }

    /// 验证管理员签名
    /// 参数:
    /// - signatures: 提交的签名列表
    /// 返回: 是否至少13个管理员签名有效
    fn verify_signatures(&self, signatures: Vec<String>) -> bool {
        // 检查签名数量是否至少为13，且所有签名都在管理员列表中
        signatures.len() >= 13 && signatures.iter().all(|sig| self.admins.contains(sig))
    }

    /// 获取投票结果
    /// 参数:
    /// - signatures: 管理员签名列表
    /// 返回: 投票票数（13个及以上签名返回19票，否则0票）
    pub fn vote(&self, signatures: Vec<String>) -> usize {
        if self.verify_signatures(signatures) {
            19 // 13个及以上签名代表19票全票同意
        } else {
            0 // 未达阈值，无票
        }
    }

    /// 发起投票
    /// 参数:
    /// - signatures: 管理员签名列表
    /// - proposal: 投票提案内容
    /// 返回: 是否成功发起投票
    pub fn initiate_vote(&self, signatures: Vec<String>, proposal: String) -> Result<String, String> {
        if !self.verify_signatures(signatures) {
            return Err("签名验证失败，至少需要13个管理员签名".to_string());
        }
        Ok(format!("成功发起投票: {}", proposal))
    }

    /// 发行五民币
    /// 参数:
    /// - signatures: 管理员签名列表
    /// - amount: 发行金额
    /// - issue_time: 计划发行时间
    /// - auth_votes: 权威节点投票结果（(赞成票数，总票数)）
    /// - all_votes: 全链可投票节点投票结果（(反对票数，总票数)）
    /// 返回: 是否成功发行
    pub fn issue_currency(
        &self,
        signatures: Vec<String>,
        amount: f64,
        issue_time: chrono::DateTime<Utc>,
        auth_votes: (usize, usize), // (赞成票数，总票数)
        all_votes: (usize, usize),  // (反对票数，总票数)
    ) -> Result<String, String> {
        if !self.verify_signatures(signatures) {
            return Err("签名验证失败，至少需要13个管理员签名".to_string());
        }
        if amount <= 0.0 {
            return Err("发行金额必须大于0".to_string());
        }

        // 检查发行时间是否晚于广播时间30天
        let now = Utc::now();
        let min_issue_time = now + Duration::days(30);
        if issue_time < min_issue_time {
            return Err("发行时间必须晚于广播时间30天".to_string());
        }

        // 检查权威节点投票（需超过65%）
        let (auth_yes, auth_total) = auth_votes;
        let auth_ratio = auth_yes as f64 / auth_total as f64;
        if auth_ratio <= 0.65 {
            return Err(format!(
                "权威节点投票未通过，赞成票 {}/{} 未超过65%",
                auth_yes, auth_total
            ));
        }

        // 检查全链否决投票（反对票超过65%则否决）
        let (all_no, all_total) = all_votes;
        let all_no_ratio = all_no as f64 / all_total as f64;
        if all_no_ratio > 0.65 {
            return Err(format!(
                "全链投票否决，反对票 {}/{} 超过65%",
                all_no, all_total
            ));
        }

        // 模拟广播（实际需调用network模块）
        let broadcast_msg = format!(
            "国储会广播: 计划于 {} 发行 {} 五民币",
            issue_time.to_rfc3339(),
            amount
        );
        println!("{}", broadcast_msg); // 临时模拟广播

        Ok(format!("成功发行 {} 五民币，发行时间: {}", amount, issue_time.to_rfc3339()))
    }

    /// 销毁五民币
    /// 参数:
    /// - signatures: 管理员签名列表
    /// - amount: 销毁金额
    /// 返回: 是否成功销毁
    pub fn destroy_currency(&self, signatures: Vec<String>, amount: f64) -> Result<String, String> {
        if !self.verify_signatures(signatures) {
            return Err("签名验证失败，至少需要13个管理员签名".to_string());
        }
        if amount <= 0.0 {
            return Err("销毁金额必须大于0".to_string());
        }
        Ok(format!("成功销毁 {} 五民币", amount))
    }

    /// 新增省储会或省储行节点
    /// 参数:
    /// - signatures: 管理员签名列表
    /// - node_id: 新节点ID
    /// - is_provincial_authority: 是否为省储会节点（true为省储会，false为省储行）
    /// 返回: 是否成功新增
    pub fn add_node(&self, signatures: Vec<String>, node_id: String, is_provincial_authority: bool) -> Result<String, String> {
        if !self.verify_signatures(signatures) {
            return Err("签名验证失败，至少需要13个管理员签名".to_string());
        }
        if node_id.starts_with("PROV_AUTH_") || node_id.starts_with("PROV_RES_") {
            if let Ok(num) = node_id[10..].parse::<u32>() {
                if num <= 43 {
                    return Err("不能新增初始省储会或省储行节点".to_string());
                }
            }
        }
        let node_type = if is_provincial_authority { "省储会" } else { "省储行" };
        Ok(format!("成功新增{}节点: {}", node_type, node_id))
    }

    /// 删除省储会或省储行节点
    /// 参数:
    /// - signatures: 管理员签名列表
    /// - node_id: 要删除的节点ID
    /// 返回: 是否成功删除
    pub fn remove_node(&self, signatures: Vec<String>, node_id: String) -> Result<String, String> {
        if !self.verify_signatures(signatures) {
            return Err("签名验证失败，至少需要13个管理员签名".to_string());
        }
        if node_id.starts_with("PROV_AUTH_") || node_id.starts_with("PROV_RES_") {
            if let Ok(num) = node_id[10..].parse::<u32>() {
                if num <= 43 {
                    return Err("不能删除初始省储会或省储行节点".to_string());
                }
            }
        }
        Ok(format!("成功删除节点: {}", node_id))
    }

    /// 获取钱包地址
    /// 返回: 国储会节点的钱包地址
    pub fn get_wallet(&self) -> String {
        self.wallet.clone()
    }
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_national_authority() {
        // 创建国储会权威节点
        let mut national = NationalAuthorityInfo::new(vec![
            "admin1".to_string(), "admin2".to_string(), "admin3".to_string(), "admin4".to_string(),
            "admin5".to_string(), "admin6".to_string(), "admin7".to_string(), "admin8".to_string(),
            "admin9".to_string(), "admin10".to_string(), "admin11".to_string(), "admin12".to_string(),
            "admin13".to_string(), "admin14".to_string(), "admin15".to_string(), "admin16".to_string(),
            "admin17".to_string(), "admin18".to_string(), "admin19".to_string(),
        ]);

        // 测试节点ID和钱包
        assert_eq!(national.node_id, "NATIONAL_001");
        assert_eq!(national.wallet.len(), 66);

        // 测试签名验证和投票
        let valid_signs = vec![
            "admin1".to_string(), "admin2".to_string(), "admin3".to_string(), "admin4".to_string(),
            "admin5".to_string(), "admin6".to_string(), "admin7".to_string(), "admin8".to_string(),
            "admin9".to_string(), "admin10".to_string(), "admin11".to_string(), "admin12".to_string(),
            "admin13".to_string(),
        ];
        let invalid_signs = vec![
            "admin1".to_string(), "admin2".to_string(), "admin3".to_string(), "admin4".to_string(),
            "admin5".to_string(), "admin6".to_string(), "admin7".to_string(), "admin8".to_string(),
            "admin9".to_string(), "admin10".to_string(), "admin11".to_string(), "admin12".to_string(),
        ];
        assert_eq!(national.vote(valid_signs.clone()), 19);
        assert_eq!(national.vote(invalid_signs.clone()), 0);

        // 测试发行五民币
        let issue_time = Utc::now() + Duration::days(31); // 晚于30天
        let auth_votes_pass = (42, 62); // 19+43=62票，42票>65%
        let auth_votes_fail = (40, 62); // 40票<65%
        let all_votes_pass = (100, 1000); // 10%反对<65%
        let all_votes_veto = (700, 1000); // 70%反对>65%

        assert!(national.issue_currency(
            valid_signs.clone(),
            1000.0,
            issue_time,
            auth_votes_pass,
            all_votes_pass
        ).is_ok());
        assert!(national.issue_currency(
            valid_signs.clone(),
            1000.0,
            Utc::now() + Duration::days(10), // 少于30天
            auth_votes_pass,
            all_votes_pass
        ).is_err());
        assert!(national.issue_currency(
            valid_signs.clone(),
            1000.0,
            issue_time,
            auth_votes_fail,
            all_votes_pass
        ).is_err());
        assert!(national.issue_currency(
            valid_signs.clone(),
            1000.0,
            issue_time,
            auth_votes_pass,
            all_votes_veto
        ).is_err());
        assert!(national.issue_currency(
            invalid_signs.clone(),
            1000.0,
            issue_time,
            auth_votes_pass,
            all_votes_pass
        ).is_err());

        // 测试销毁五民币
        assert_eq!(
            national.destroy_currency(valid_signs.clone(), 500.0).unwrap(),
            "成功销毁 500 五民币"
        );

        // 测试新增和删除节点
        assert_eq!(
            national.add_node(valid_signs.clone(), "PROV_AUTH_044".to_string(), true).unwrap(),
            "成功新增省储会节点: PROV_AUTH_044"
        );
        assert!(national.add_node(valid_signs.clone(), "PROV_AUTH_001".to_string(), true).is_err());
        assert_eq!(
            national.remove_node(valid_signs.clone(), "PROV_AUTH_044".to_string()).unwrap(),
            "成功删除节点: PROV_AUTH_044"
        );
        assert!(national.remove_node(valid_signs.clone(), "PROV_AUTH_001".to_string()).is_err());
    }
}