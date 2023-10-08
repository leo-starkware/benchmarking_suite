use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::{core::types::{BlockId, FieldElement}, providers::Provider};

use url::Url;
use criterion::{BenchmarkId, BenchmarkGroup, measurement::WallTime};
use utils::{hash_hex_to_fe, parse_block_id};
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

impl RawInputs {
    // Deserialize a json file into a RawInputs instance
    pub fn new_from_json(path: &str) -> RawInputs {
        let contents = fs::read_to_string(path).expect("Config file not found");
        let deserialized: RawInputs = serde_json::from_str(contents.as_str()).unwrap();
        assert!(deserialized.names.len() == deserialized.urls.len());
        deserialized
    }
}

// The BenchRunner struct contains all the information needed to send an RPC call
pub struct BenchRunner {
    pub name: String,
    pub provider: BenchedProvider,
    pub method: String,
    pub block: BlockId,
    pub class_hash: FieldElement,
    pub tx_hash: FieldElement,
}

impl BenchRunner {
    pub fn new(
        name: &str, 
        url: &str, 
        method_name: &str, 
        block_tag: &str, 
        class_hash: &str, 
        tx_hash: &str) -> BenchRunner {

        let name = name;
        let provider = BenchedProvider::new(url);
        let method = method_name;
        let block = parse_block_id(block_tag).unwrap();
        let class_hash = hash_hex_to_fe(class_hash).unwrap();
        let tx_hash = hash_hex_to_fe(tx_hash).unwrap();

        BenchRunner { name: name.to_string(), 
                    provider: provider, 
                    method: method.to_string(), 
                    block: block, 
                    class_hash: class_hash, 
                    tx_hash: tx_hash }
    }

    // Runs benchmarks for RPC calls based on the data contained in &self
    // when 'show_block_number' is set to true, the block number is shown
    // in the path in the criterion report
    pub fn run(
        &self, 
        group: &mut BenchmarkGroup<'_, WallTime>, 
        runner: &tokio::runtime::Runtime,
        show_block_number: bool) -> () {
        
        let provider = &self.provider;
        let formatted_name = match show_block_number {
            true => format!("{:?}/{}", self.block, self.method.as_str()),
                
            false => self.method.clone()
        };
        
        match self.method.as_str() {
            "starknet_getStateUpdate" => {group
                .bench_with_input(
                    BenchmarkId::new(formatted_name, &self.name),
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
                    BenchmarkId::new(formatted_name, &self.name),
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
                    BenchmarkId::new(formatted_name, &self.name),
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
                    BenchmarkId::new(formatted_name, &self.name),
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
                    BenchmarkId::new(formatted_name, &self.name),
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
                    BenchmarkId::new(formatted_name, &self.name),
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
                    BenchmarkId::new(formatted_name, &self.name),
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
                        
            _ => ()
        }
    }
}

