//! ERC721 NFT 原创合约 - 唯一数字资产
use std::collections::HashMap;

pub struct ERC721NFT {
    name: String,
    symbol: String,
    owners: HashMap<u64, String>,
    balances: HashMap<String, u64>,
    token_uri: HashMap<u64, String>,
    next_token_id: u64,
}

impl ERC721NFT {
    pub fn new(name: &str, symbol: &str) -> Self {
        Self {
            name: name.into(),
            symbol: symbol.into(),
            owners: HashMap::new(),
            balances: HashMap::new(),
            token_uri: HashMap::new(),
            next_token_id: 1,
        }
    }

    pub fn mint(&mut self, to: String, uri: String) -> u64 {
        let token_id = self.next_token_id;
        self.owners.insert(token_id, to.clone());
        *self.balances.entry(to).or_insert(0) += 1;
        self.token_uri.insert(token_id, uri);
        self.next_token_id += 1;
        token_id
    }
}

fn main() {
    let mut nft = ERC721NFT::new("RustNFT", "RNFT");
    let id = nft.mint("User1".into(), "ipfs://rustnft".into());
    println!("Minted NFT ID: {}", id);
}
