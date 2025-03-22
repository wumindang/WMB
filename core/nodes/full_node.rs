use crate::crypto::signature::generate_keypair;
use chrono::Utc;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct FullNodeInfo {
    pub node_id: String,       // "FULL_20250321123456_ABCD1234"
    pub public_key: secp256k1::PublicKey,
}

impl FullNodeInfo {
    pub fn new() -> Self {
        let (_, public_key) = generate_keypair();
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let random_str = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>();
        let node_id = format!("FULL_{}_{}", timestamp, random_str);
        Self { node_id, public_key }
    }
}