use benchmarking_suite::BenchedProvider;
use starknet::core::types::BlockId;
use starknet::providers::Provider;

use tokio::runtime::Runtime;

use std::iter;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

const BLAST_ADDRESS: &str = "https://starknet-mainnet.public.blastapi.io";
const LOCAL_ADDRESS: &str = "http://192.168.1.45:1235";

pub fn criterion_request(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("get_block");
    let blast_provider = BenchedProvider::new(BLAST_ADDRESS);
    let local_provider = BenchedProvider::new(LOCAL_ADDRESS);

    let providers = vec![blast_provider, local_provider];
    for provider in providers {
        group.bench_with_input(
            BenchmarkId::from_parameter(provider.url.as_str()),
            &provider,
            |b, provider| {
                b.to_async(&rt).iter(|| {
                    provider
                        .provider
                        .get_block_with_tx_hashes(BlockId::Number(0))
                })
            },
        );
    }
}

criterion_group!(benches, criterion_request);
criterion_main!(benches);
