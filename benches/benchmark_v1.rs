use benchmarking_suite::{provider, get_block_with_tx_hashes, get_block_with_txs};
use starknet::core::types::BlockId;
// use bench_suite_v3::{BenchInputer, BenchTrait, request, provider};

use criterion::{criterion_group, criterion_main, Criterion};
// use std::env;

const API_ADDRESS: &str = "https://starknet-mainnet.public.blastapi.io";

// https://starknet-mainnet.public.blastapi.io
// https://starknet-mainnet.infura.io/v3/aa3a17de2faa41c39083706ba85ac1c6


pub fn criterion_request(c: &mut Criterion) {
    // let args: Vec<String> = env::args().collect();
    // println!("{}", args[0].as_str());
    let provider = provider(API_ADDRESS);
    c.bench_function("bench", |b| { b.iter(|| get_block_with_txs(&provider, BlockId::Number(20000)))});
}


criterion_group!(benches, criterion_request);
criterion_main!(benches);