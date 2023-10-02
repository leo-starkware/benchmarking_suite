use benchmarking_suite::utils::parse_block_id;
use benchmarking_suite::BenchedProvider;
use starknet::providers::Provider;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const LOCAL_ADDRESS: &str = "http://192.168.1.45:1235";

pub fn bench_blocks(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("blocks");

    let local_provider = BenchedProvider::new(LOCAL_ADDRESS);
    let blocks = vec!["latest", "pending"];

    for block in &blocks {
        group
            .bench_with_input(
                BenchmarkId::new("bench_blocks/", block),
                &block,
                |b, block| {
                    b.to_async(&rt).iter(|| {
                        local_provider
                            .provider
                            .get_block_with_tx_hashes(parse_block_id(block).unwrap())
                    })
                },
            )
            .sample_size(10);
    }
}

criterion_group!(benches, bench_blocks);
criterion_main!(benches);
