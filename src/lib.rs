use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::{core::types::{BlockId, FieldElement}, providers::Provider};

use url::Url;
use criterion::{BenchmarkId, BenchmarkGroup, measurement::WallTime};
use utils::{hash_hex_to_fe, block_number_to_id, parse_block_id};
use std::fs;
use serde::{Serialize,Deserialize};
use serde_json;
pub mod utils;
pub mod constants;



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
    pub names: Vec<String>,
    pub urls: Vec<String>,
    pub methods: Vec<String>,
    pub params: ParamInputs
}
pub struct BenchRunner {
    pub inputs: RawInputs,
    pub block: BlockId,
    pub class_hash: FieldElement,
    pub tx_hash: FieldElement,
}

impl BenchRunner {
    pub fn new_from_json(path: &str) -> BenchRunner {
        let contents = fs::read_to_string(path).expect("Config file not found");
        let deserialized: RawInputs = serde_json::from_str(contents.as_str()).unwrap();
        assert!(deserialized.names.len() == deserialized.urls.len());

        let block = block_number_to_id(deserialized.params.block.as_str());
        let class_hash = hash_hex_to_fe(deserialized.params.class_hash.as_str()).unwrap();
        let tx_hash = hash_hex_to_fe(deserialized.params.tx_hash.as_str()).unwrap();

        BenchRunner { inputs: deserialized, block: block, class_hash: class_hash, tx_hash: tx_hash }
    }

    pub fn run_by_method(&self, group: &mut BenchmarkGroup<'_, WallTime>, provider: &BenchedProvider, url_name: &str, method_name: &str, runner: &tokio::runtime::Runtime) {
        match method_name {
            "starknet_getStateUpdate" => {group
                .bench_with_input(
                    BenchmarkId::new("get_state_update", url_name),
                    &provider,
                    |b, provider| {
                        b.to_async(runner).iter(|| {
                            provider
                                .provider
                                .get_state_update(self.block)
                        })
                    },
                )
                .sample_size(10);},

            "starknet_blockNumber" => {group
                .bench_with_input(
                    BenchmarkId::new("block_number", url_name),
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
                    BenchmarkId::new("get_block_with_tx_hashes", url_name),
                    &provider,
                    |b, provider| {
                        b.to_async(runner).iter(|| {
                            provider
                                .provider
                                .get_block_with_tx_hashes(self.block)
                        })
                    },
                )
                .sample_size(10);},

            "starknet_getBlockWithTxs" => {group
                .bench_with_input(
                    BenchmarkId::new("get_block_with_txs", url_name),
                    &provider,
                    |b, provider| {
                        b.to_async(runner).iter(|| {
                            provider
                                .provider
                                .get_block_with_txs(self.block)
                        })
                    },
                )
                .sample_size(10);},

            "starknet_getClass" => {group
                .bench_with_input(
                    BenchmarkId::new("get_class", url_name),
                    &provider,
                    |b, provider| {
                        b.to_async(runner).iter(|| {
                            provider
                                .provider
                                .get_class(self.block, self.class_hash)
                        })
                    },
                )
                .sample_size(10);},

            "starknet_getTransactionByHash" => {group
                .bench_with_input(
                    BenchmarkId::new("get_transaction_by_hash", url_name),
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

            "starknet_getTransactionReceipt" => {group
                .bench_with_input(
                    BenchmarkId::new("get_transaction_receipt", url_name),
                    &provider,
                    |b, provider| {
                        b.to_async(runner).iter(|| {
                            provider
                                .provider
                                .get_transaction_receipt(self.tx_hash)
                        })
                    },
                )
                .sample_size(10);},
            
             "starknet_pending" => (),
            
            _ => ()
        }
    }
    
    pub fn run_by_block(&self, group: &mut BenchmarkGroup<'_, WallTime>, provider: &BenchedProvider, url_name: &str, runner: &tokio::runtime::Runtime) {
        let blocks = vec!["latest", "pending"];
        for block in &blocks {
            group
                .bench_with_input(
                    BenchmarkId::new("bench_blocks/", block),
                    &block,
                    |b, block| {
                        b.to_async(runner).iter(|| {
                            provider
                                .provider
                                .get_block_with_tx_hashes(parse_block_id(block).unwrap())
                        })
                    },
                )
                .sample_size(10);
        }

    }

}