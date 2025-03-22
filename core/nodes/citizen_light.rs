use crate::crypto::signature::generate_keypair;

#[derive(Debug, Clone)]
pub struct CitizenLightInfo {
    pub node_id: String,       // 钱包地址即ID
    pub ciic: Option<String>,  // 公民身份识别码（绑定后才有）
    pub is_authenticated: bool,// 是否完成认证
}

impl CitizenLightInfo {
    pub fn new() -> Self {
        let (_, public_key) = generate_keypair();
        let node_id = hex::encode(public_key.serialize()); // ID即钱包地址
        Self { node_id, ciic: None, is_authenticated: false }
    }

    pub fn bind_ciic(&mut self, ciic: String) -> Result<(), String> {
        if self.ciic.is_some() {
            return Err("该节点已绑定CIIC".to_string());
        }
        if Self::verify_ciic(&ciic) {
            self.ciic = Some(ciic);
            self.is_authenticated = true;
            Ok(())
        } else {
            Err("CIIC校验失败".to_string())
        }
    }

    fn verify_ciic(ciic: &str) -> bool {
        ciic.starts_with("C") && ciic.len() == 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_citizen_light() {
        let mut citizen = CitizenLightInfo::new();
        assert_eq!(citizen.node_id.len(), 66);
        assert!(!citizen.is_authenticated);
        assert!(citizen.bind_ciic("C123456789".to_string()).is_ok());
        assert!(citizen.is_authenticated);
        assert_eq!(citizen.ciic, Some("C123456789".to_string()));
        assert!(citizen.bind_ciic("C987654321".to_string()).is_err());
    }
}