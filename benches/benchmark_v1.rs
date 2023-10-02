use benchmarking_suite::utils::parse_block_id;
use benchmarking_suite::BenchedProvider;
use starknet::{core::types::BlockId, providers::Provider};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
mod config_reader;
use config_reader::RawInputs;

// const BLAST_ADDRESS: &str = "https://starknet-mainnet.public.blastapi.io";
// const LOCAL_ADDRESS: &str = "http://192.168.1.45:1235";
const PATH: &str = "./config/config.json";

pub fn bench_providers(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("providers");
    let inputs = RawInputs::new_from_json(PATH);
    let blocks = vec!["latest", "pending"];

    // let blast_provider = BenchedProvider::new(BLAST_ADDRESS);
    // let local_provider = BenchedProvider::new(LOCAL_ADDRESS);
    // let providers = vec![blast_provider, local_provider];

    for url in inputs.urls {
        let provider = BenchedProvider::new(url.as_str());
        group
            .bench_with_input(
                BenchmarkId::new("get_block_with_tx_hashes/", provider.url.as_str()),
                &provider,
                |b, provider| {
                    b.to_async(&rt).iter(|| {
                        provider
                            .provider
                            .get_block_with_tx_hashes(BlockId::Number(0))
                    })
                },
            )
            .sample_size(10);

        group
            .bench_with_input(
                BenchmarkId::new("pending_transactions/", provider.url.as_str()),
                &provider,
                |b, provider| {
                    b.to_async(&rt)
                        .iter(|| provider.provider.pending_transactions())
                },
            )
            .sample_size(10);

        group
        .bench_with_input(
            BenchmarkId::new("block_number/", provider.url.as_str()),
            &provider,
            |b, provider| {
                b.to_async(&rt).iter(|| {
                    provider
                        .provider
                        .block_number()
                })
            },
        )
        .sample_size(10);

        group
        .bench_with_input(
            BenchmarkId::new("get_state_update/", provider.url.as_str()),
            &provider,
            |b, provider| {
                b.to_async(&rt).iter(|| {
                    provider
                        .provider
                        .get_state_update(BlockId::Number(10))
                })
            },
        )
        .sample_size(10);
        
    }
}

pub fn bench_blocks(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("blocks");
    let local_address: &str = "http://192.168.1.45:1235";
    let local_provider = BenchedProvider::new(local_address);
    let blocks = vec!["latest", "pending"];

    for block in &blocks {
        group
            .bench_with_input(BenchmarkId::new("get_block/", block), &block, |b, block| {
                b.to_async(&rt).iter(|| {
                    local_provider
                        .provider
                        .get_block_with_tx_hashes(parse_block_id(block).unwrap())
                })
            })
            .sample_size(10);
    }
}

criterion_group!(benches_p, bench_providers);
criterion_main!(benches_p);
