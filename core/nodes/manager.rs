use super::types::NodeType;
use super::super_node::SuperNodeInfo;
use super::national_authority::NationalAuthorityInfo;
use super::prov_authority::ProvincialAuthorityInfo;
use super::prov_reserve::ProvincialReserveInfo;
use super::full_node::FullNodeInfo;
use super::citizen_light::CitizenLightInfo;
use super::guest_light::GuestLightInfo;
use std::collections::HashMap;

#[derive(Debug)]
pub struct NodeManager {
    pub nodes: HashMap<String, NodeType>,
    pub ciic_map: HashMap<String, String>,
}

impl NodeManager {
    pub fn new() -> Self {
        let mut nodes = HashMap::new();
        let super_node = SuperNodeInfo::new(vec!["admin".to_string(); 5]);
        nodes.insert(super_node.node_id.clone(), NodeType::SuperNode(super_node));
        Self { nodes, ciic_map: HashMap::new() }
    }

    pub fn initialize_system(&mut self) {
        let national = NationalAuthorityInfo::new(vec!["admin".to_string(); 19]);
        self.nodes.insert(national.node_id.clone(), NodeType::NationalAuthority(national));

        for i in 1..=43 {
            let auth_id = format!("PROV_AUTH_{:03}", i);
            let res_id = format!("PROV_RES_{:03}", i);
            let prov_auth = ProvincialAuthorityInfo::new(auth_id.clone(), vec!["admin".to_string(); 9], "wallet".to_string(), true);
            let prov_res = ProvincialReserveInfo::new(res_id.clone(), vec!["admin".to_string(); 9], "stake".to_string(), "tx".to_string(), 1000.0, true);
            self.nodes.insert(auth_id, NodeType::ProvincialAuthority(prov_auth));
            self.nodes.insert(res_id, NodeType::ProvincialReserve(prov_res));
        }
    }

    pub fn add_full_node(&mut self) -> String {
        let full_node = FullNodeInfo::new();
        let node_id = full_node.node_id.clone();
        self.nodes.insert(node_id.clone(), NodeType::FullNode(full_node));
        node_id
    }

    pub fn add_citizen_light(&mut self) -> String {
        let citizen = CitizenLightInfo::new();
        let node_id = citizen.node_id.clone();
        self.nodes.insert(node_id.clone(), NodeType::CitizenLight(citizen));
        node_id
    }

    pub fn bind_citizen_ciic(&mut self, node_id: &str, ciic: String) -> Result<(), String> {
        if self.ciic_map.contains_key(&ciic) {
            return Err("该CIIC已被绑定".to_string());
        }
        if let Some(NodeType::CitizenLight(citizen)) = self.nodes.get_mut(node_id) {
            citizen.bind_ciic(ciic.clone())?;
            self.ciic_map.insert(ciic, node_id.to_string());
            Ok(())
        } else {
            Err("节点不存在或不是公民轻节点".to_string())
        }
    }

    pub fn add_guest_light(&mut self) -> String {
        let guest = GuestLightInfo::new();
        let wallet = guest.add_wallet();
        self.nodes.insert(wallet.clone(), NodeType::GuestLight(guest));
        wallet
    }

    pub fn get_node(&self, node_id: &str) -> Option<&NodeType> {
        self.nodes.get(node_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_manager() {
        let mut manager = NodeManager::new();
        manager.initialize_system();

        let full_id = manager.add_full_node();
        let citizen_id = manager.add_citizen_light();
        let guest_id = manager.add_guest_light();

        assert!(manager.bind_citizen_ciic(&citizen_id, "C123456789".to_string()).is_ok());
        assert!(manager.bind_citizen_ciic(&citizen_id, "C987654321".to_string()).is_err());
        assert!(manager.bind_citizen_ciic(&full_id, "C987654321".to_string()).is_err());
        assert!(manager.bind_citizen_ciic(&citizen_id, "INVALID".to_string()).is_err());

        if let Some(NodeType::CitizenLight(citizen)) = manager.get_node(&citizen_id) {
            assert!(citizen.is_authenticated);
            assert_eq!(citizen.ciic, Some("C123456789".to_string()));
        }

        assert!(manager.get_node("SUPER_001").is_some());
        assert!(manager.get_node("NATIONAL_001").is_some());
        assert!(manager.get_node("PROV_AUTH_001").is_some());
        assert!(manager.get_node("PROV_RES_001").is_some());
        assert!(manager.get_node(&full_id).is_some());
        assert!(manager.get_node(&citizen_id).is_some());
        assert!(manager.get_node(&guest_id).is_some());
    }
}