use crate::crypto::signature::generate_keypair;

#[derive(Debug, Clone)]
pub struct GuestLightInfo {
    pub wallets: Vec<String>,  // 多个钱包地址，无ID
}

impl GuestLightInfo {
    pub fn new() -> Self {
        Self { wallets: Vec::new() }
    }

    pub fn add_wallet(&mut self) -> String {
        let (_, public_key) = generate_keypair();
        let wallet = hex::encode(public_key.serialize());
        self.wallets.push(wallet.clone());
        wallet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guest_light() {
        let mut guest = GuestLightInfo::new();
        let wallet = guest.add_wallet();
        assert_eq!(guest.wallets.len(), 1);
        assert_eq!(wallet.len(), 66);
    }
}