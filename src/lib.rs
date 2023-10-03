use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::{core::types::BlockId, providers::Provider};

use url::Url;
use tokio::runtime::Runtime;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, BenchmarkGroup, measurement::WallTime};
use std::fs;
use serde::{Serialize,Deserialize};
use serde_json;
pub mod utils;

#[derive(Debug)]
pub struct BenchedProvider {
    pub provider: JsonRpcClient<HttpTransport>,
    pub url: String,
}

impl BenchedProvider {
    pub fn new(url: &str) -> Self {
        Self {
            provider: BenchedProvider::url_to_client(url),
            url: url.to_string(),
        }
    }

    // Get a client from a url address
    fn url_to_client(address: &str) -> JsonRpcClient<HttpTransport> {
        JsonRpcClient::new(HttpTransport::new(Url::parse(address).unwrap()))
    }
}


#[derive(Serialize, Deserialize)]
pub struct RawInputs {
    pub urls: Vec<String>,
    pub methods: Vec<String>,
}
pub struct BenchRunner {
    pub inputs: RawInputs
}
impl BenchRunner {
    pub fn new_from_json(path: &str) -> BenchRunner {
        let contents = fs::read_to_string(path).expect("Config file not found");
        let deserialized: RawInputs = serde_json::from_str(contents.as_str()).unwrap();
        BenchRunner { inputs: deserialized }
    }

    pub fn run_by_method(group: &mut BenchmarkGroup<'_, WallTime>, provider: &BenchedProvider, method_name: &str, runner: &tokio::runtime::Runtime) {
        match method_name {
            "starknet_getStateUpdate" => {group
                .bench_with_input(
                    BenchmarkId::new("get_state_update", provider.url.as_str()),
                    &provider,
                    |b, provider| {
                        b.to_async(runner).iter(|| {
                            provider
                                .provider
                                .get_state_update(BlockId::Number(10))
                        })
                    },
                )
                .sample_size(10);},

            "starknet_blockNumber" => {group
                .bench_with_input(
                    BenchmarkId::new("block_number", provider.url.as_str()),
                    &provider,
                    |b, provider| {
                        b.to_async(runner).iter(|| {
                            provider
                                .provider
                                .block_number()
                        })
                    },
                )
                .sample_size(10);},

            "starknet_getBlockWithTxHashes" => {group
                .bench_with_input(
                    BenchmarkId::new("get_block_with_tx_hashes", provider.url.as_str()),
                    &provider,
                    |b, provider| {
                        b.to_async(runner).iter(|| {
                            provider
                                .provider
                                .get_block_with_tx_hashes(BlockId::Number(10))
                        })
                    },
                )
                .sample_size(10);},

            "starknet_pending" => (),
            _ => ()
        }
    }
}