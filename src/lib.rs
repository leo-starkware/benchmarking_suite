use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::{core::types::{BlockId, FieldElement}, providers::Provider};

use url::Url;
use tokio::runtime::Runtime;
use criterion::{BenchmarkId, BenchmarkGroup, measurement::WallTime};
use std::fs;
use serde::{Serialize,Deserialize};
use serde_json;
pub mod utils;

const SAMPLE_BLOCK: u64 = 10;
const SAMPLE_CLASS_HASH: &str = "0x025ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918";
const SAMPLE_TX_HASH: &str = "0x071eed7f033331c8d7bd1a4dca8eedf16951a904de3e195005e49aae9e502ca6";

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

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamInputs {
    pub block: String,
    pub class_hash: String,
    pub tx_hash: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawInputs {
    pub urls: Vec<String>,
    pub methods: Vec<String>,
    pub params: ParamInputs
}
pub struct BenchRunner {
    pub inputs: RawInputs,
    pub block: u64,
    pub class_hash: FieldElement,
    pub tx_hash: FieldElement,
}

impl BenchRunner {
    pub fn new_from_json(path: &str) -> BenchRunner {
        let contents = fs::read_to_string(path).expect("Config file not found");
        let deserialized: RawInputs = serde_json::from_str(contents.as_str()).unwrap();

        let block: u64 = deserialized.params.block.parse().unwrap();
        let class_hash = FieldElement::from_hex_be(deserialized.params.class_hash.as_str()).unwrap();
        let tx_hash = FieldElement::from_hex_be(deserialized.params.tx_hash.as_str()).unwrap();

        BenchRunner { inputs: deserialized, block: block, class_hash: class_hash, tx_hash: tx_hash }
    }

    pub fn run_by_method(&self, group: &mut BenchmarkGroup<'_, WallTime>, provider: &BenchedProvider, method_name: &str, runner: &tokio::runtime::Runtime) {
        match method_name {
            "starknet_getStateUpdate" => {group
                .bench_with_input(
                    BenchmarkId::new("get_state_update", provider.url.as_str()),
                    &provider,
                    |b, provider| {
                        b.to_async(runner).iter(|| {
                            provider
                                .provider
                                .get_state_update(BlockId::Number(self.block))
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
                                .get_block_with_tx_hashes(BlockId::Number(self.block))
                        })
                    },
                )
                .sample_size(10);},

            "starknet_pending" => (),

            "starknet_getBlockWithTxs" => {group
                .bench_with_input(
                    BenchmarkId::new("get_block_with_txs", provider.url.as_str()),
                    &provider,
                    |b, provider| {
                        b.to_async(runner).iter(|| {
                            provider
                                .provider
                                .get_block_with_txs(BlockId::Number(self.block))
                        })
                    },
                )
                .sample_size(10);},

            "starknet_getClass" => {group
                .bench_with_input(
                    BenchmarkId::new("get_class", provider.url.as_str()),
                    &provider,
                    |b, provider| {
                        b.to_async(runner).iter(|| {
                            provider
                                .provider
                                .get_class(BlockId::Number(self.block), self.class_hash)
                        })
                    },
                )
                .sample_size(10);},

                "starknet_getTransactionByHash" => {group
                    .bench_with_input(
                        BenchmarkId::new("get_transaction_by_hash", provider.url.as_str()),
                        &provider,
                        |b, provider| {
                            b.to_async(runner).iter(|| {
                                provider
                                    .provider
                                    .get_transaction_by_hash(self.tx_hash)
                            })
                        },
                    )
                    .sample_size(10);},

            _ => ()
        }
    }
}