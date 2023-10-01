use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use url::Url;

pub mod utils;

#[derive(Debug)]
pub struct BenchedProvider {
    pub provider: JsonRpcClient<HttpTransport>,
    pub url: String,
}

impl BenchedProvider {
    pub fn new(url: &str) -> Self {
        Self {
            provider: provider(url),
            url: url.to_string(),
        }
    }
}

// Get a client from a url address
pub fn provider(address: &str) -> JsonRpcClient<HttpTransport> {
    JsonRpcClient::new(HttpTransport::new(Url::parse(address).unwrap()))
}
