use benchmarking_suite::{get_block_with_tx_hashes, get_block_with_txs, BenchedProvider};
use starknet::core::types::BlockId;

use std::iter;
// use bench_suite_v3::{BenchInputer, BenchTrait, request, provider};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
// use std::env;

const BLAST_ADDRESS: &str = "https://starknet-mainnet.public.blastapi.io";
const LOCAL_ADDRESS: &str = "http://192.168.1.45:1235";

// https://starknet-mainnet.public.blastapi.io
// https://starknet-mainnet.infura.io/v3/aa3a17de2faa41c39083706ba85ac1c6

pub fn criterion_request(c: &mut Criterion) {
    // let args: Vec<String> = env::args().collect();
    // println!("{}", args[0].as_str());
    let mut group = c.benchmark_group("get_block");
    let blast_provider = BenchedProvider::new(BLAST_ADDRESS);
    let local_provider = BenchedProvider::new(LOCAL_ADDRESS);

    let providers = vec![blast_provider, local_provider];
    for provider in providers {
        group.bench_with_input(
            BenchmarkId::from_parameter(provider.url.as_str()),
            &provider,
            |b, provider| {
                b.iter(|| get_block_with_tx_hashes(&provider.provider, BlockId::Number(20000)))
            },
        );
        // c.bench_function("bench", |b| {
        //     b.iter(|| get_block_with_txs(&provider, BlockId::Number(20000)))
        // });
    }
}

criterion_group!(benches, criterion_request);
criterion_main!(benches);
