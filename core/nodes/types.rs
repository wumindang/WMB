use super::super_node::SuperNodeInfo;
use super::national_authority::NationalAuthorityInfo;
use super::prov_authority::ProvincialAuthorityInfo;
use super::prov_reserve::ProvincialReserveInfo;
use super::full_node::FullNodeInfo;
use super::citizen_light::CitizenLightInfo;
use super::guest_light::GuestLightInfo;

#[derive(Debug, Clone)]
pub enum NodeType {
    SuperNode(SuperNodeInfo),
    NationalAuthority(NationalAuthorityInfo),
    ProvincialAuthority(ProvincialAuthorityInfo),
    ProvincialReserve(ProvincialReserveInfo),
    FullNode(FullNodeInfo),
    CitizenLight(CitizenLightInfo),
    GuestLight(GuestLightInfo),
}