use criterion::{measurement::WallTime, BenchmarkGroup, BenchmarkId};
use serde::{Deserialize, Serialize};
use serde_json;
use starknet::{
    core::types::{BlockId, FieldElement, FunctionCall},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider,
    SequencerGatewayProvider
    },
};
use std::fs;
use url::Url;

pub mod utils;
use utils::{hash_hex_to_fe, parse_block_id, url_checker};
pub mod constants;
use constants::{BALANCEOF_SELECTOR, ORBITER_ADDR, STARKGATE_TOKEN_CONTRACT};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamInputs {
    pub block: String,
    pub class_hash: String,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    pub name: String,
    pub url: String,
}

// The RawInputs struct will contain the data in the config.json file
#[derive(Serialize, Deserialize, Debug)]
pub struct RawInputs {
    pub targets: Vec<Target>,
    pub include_fgw: String,
    pub methods: Vec<String>,
    pub params: ParamInputs,
}

impl RawInputs {
    // Deserialize a json file into a RawInputs instance
    pub fn new_from_json(path: &str) -> RawInputs {
        let contents = fs::read_to_string(path).expect("Config file not found");
        let deserialized: RawInputs = serde_json::from_str(contents.as_str()).unwrap();
        let urls = &deserialized.targets.iter().map(|x| x.url.clone()).collect();
        url_checker(&urls);

        deserialized
    }
}

// The BenchRunner struct contains all the information needed to send an RPC call
pub struct BenchRunner<T>
where T: Provider {
    pub name: String,
    pub provider: T,
    pub method: String,
    pub block: BlockId,
    pub class_hash: FieldElement,
    pub tx_hash: FieldElement,
}

impl<T> BenchRunner<T>
where T: Provider {
    pub fn new_from_url(
        name: &str,
        url: &str,
        method_name: &str,
        block_tag: &str,
        class_hash: &str,
        tx_hash: &str,
    ) -> BenchRunner<JsonRpcClient<HttpTransport>> {
        let name = name;
        let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(url).unwrap()));
        let method = method_name;
        let block = parse_block_id(block_tag).unwrap();
        let class_hash = hash_hex_to_fe(class_hash).unwrap();
        let tx_hash = hash_hex_to_fe(tx_hash).unwrap();

        BenchRunner {
            name: name.to_string(),
            provider: provider,
            method: method.to_string(),
            block: block,
            class_hash: class_hash,
            tx_hash: tx_hash,
        }
    }
    
    pub fn new_fgw(
        method_name: &str,
        block_tag: &str,
        class_hash: &str,
        tx_hash: &str,
    ) -> BenchRunner<SequencerGatewayProvider> {
        let name = "FGW";
        let provider = SequencerGatewayProvider::starknet_alpha_mainnet();
        let method = method_name;
        let block = parse_block_id(block_tag).unwrap();
        let class_hash = hash_hex_to_fe(class_hash).unwrap();
        let tx_hash = hash_hex_to_fe(tx_hash).unwrap();

        BenchRunner {
            name: name.to_string(),
            provider: provider,
            method: method.to_string(),
            block: block,
            class_hash: class_hash,
            tx_hash: tx_hash,
        }

    }

    // Runs benchmarks for RPC calls based on the data contained in &self
    // when 'show_block_number' is set to true, the block number is shown
    // in the path in the criterion report
    pub fn run(
        &self,
        group: &mut BenchmarkGroup<'_, WallTime>,
        runner: &tokio::runtime::Runtime,
        show_block_number: bool,
    ) -> () {
        let provider = &self.provider;
        let formatted_name = match show_block_number {
            true => format!("{:?}", self.block),

            false => self.method.clone(),
        };

        match self.method.as_str() {
            "starknet_getStateUpdate" => {
                group
                    .bench_with_input(
                        BenchmarkId::new(formatted_name, &self.name),
                        &provider,
                        |b, provider| {
                            b.to_async(runner)
                                .iter(|| provider.get_state_update(self.block))
                        },
                    )
                    .sample_size(10);
            }

            "starknet_blockNumber" => {
                group
                    .bench_with_input(
                        BenchmarkId::new(formatted_name, &self.name),
                        &provider,
                        |b, provider| b.to_async(runner).iter(|| provider.block_number()),
                    )
                    .sample_size(10);
            }

            "starknet_getBlockWithTxHashes" => {
                group
                    .bench_with_input(
                        BenchmarkId::new(formatted_name, &self.name),
                        &provider,
                        |b, provider| {
                            b.to_async(runner)
                                .iter(|| provider.get_block_with_tx_hashes(self.block))
                        },
                    )
                    .sample_size(10);
            }

            "starknet_getBlockWithTxs" => {
                group
                    .bench_with_input(
                        BenchmarkId::new(formatted_name, &self.name),
                        &provider,
                        |b, provider| {
                            b.to_async(runner)
                                .iter(|| provider.get_block_with_txs(self.block))
                        },
                    )
                    .sample_size(10);
            }

            "starknet_getClass" => {
                group
                    .bench_with_input(
                        BenchmarkId::new(formatted_name, &self.name),
                        &provider,
                        |b, provider| {
                            b.to_async(runner)
                                .iter(|| provider.get_class(self.block, self.class_hash))
                        },
                    )
                    .sample_size(10);
            }

            "starknet_getClassHashAt" => {
                group
                    .bench_with_input(
                        BenchmarkId::new(formatted_name, &self.name),
                        &provider,
                        |b, provider| {
                            b.to_async(runner).iter(|| {
                                provider.get_class_hash_at(
                                    self.block,
                                    hash_hex_to_fe(STARKGATE_TOKEN_CONTRACT).unwrap(),
                                )
                            })
                        },
                    )
                    .sample_size(10);
            }

            "starknet_getTransactionByHash" => {
                group
                    .bench_with_input(
                        BenchmarkId::new(formatted_name, &self.name),
                        &provider,
                        |b, provider| {
                            b.to_async(runner)
                                .iter(|| provider.get_transaction_by_hash(self.tx_hash))
                        },
                    )
                    .sample_size(10);
            }

            "starknet_getTransactionByBlockIdAndIndex" => {
                group
                    .bench_with_input(
                        BenchmarkId::new(formatted_name, &self.name),
                        &provider,
                        |b, provider| {
                            b.to_async(runner).iter(|| {
                                provider.get_transaction_by_block_id_and_index(self.block, 2)
                            })
                        },
                    )
                    .sample_size(10);
            }

            "starknet_getTransactionReceipt" => {
                group
                    .bench_with_input(
                        BenchmarkId::new(formatted_name, &self.name),
                        &provider,
                        |b, provider| {
                            b.to_async(runner)
                                .iter(|| provider.get_transaction_receipt(self.tx_hash))
                        },
                    )
                    .sample_size(10);
            }

            "starknet_call" => {
                let contract_address = hash_hex_to_fe(STARKGATE_TOKEN_CONTRACT).unwrap();
                let entry_point_selector = hash_hex_to_fe(BALANCEOF_SELECTOR).unwrap();
                let calldata = hash_hex_to_fe(ORBITER_ADDR).unwrap();

                group
                    .bench_with_input(
                        BenchmarkId::new(formatted_name, &self.name),
                        &provider,
                        |b, provider| {
                            b.to_async(runner).iter(|| {
                                provider.call(
                                    FunctionCall {
                                        contract_address: contract_address,
                                        entry_point_selector: entry_point_selector,
                                        calldata: vec![calldata],
                                    },
                                    self.block,
                                )
                            })
                        },
                    )
                    .sample_size(10);
            }

            _ => (),
        }
    }
}
