use benchmarking_suite::{BenchRunner, RawInputs};
use criterion::{criterion_group, criterion_main, Criterion};
use benchmarking_suite::constants::{PATH, SAMPLE_BLOCK_TAGS};
use std::iter::zip;

pub fn bench_by_method(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("by_methods");
    
    let inputs = RawInputs::new_from_json(PATH);

    for (url, name) in zip(inputs.urls.iter(), inputs.names.iter()) {
        for method_name in inputs.methods.iter() {
            let bench_runner = BenchRunner::new(
                name.as_str(), 
                url.as_str(), 
                method_name.as_str(), 
                inputs.params.block.as_str(), 
                inputs.params.class_hash.as_str(), 
                inputs.params.tx_hash.as_str());

            bench_runner.run(&mut group, &rt, false);
        }
    }
}

pub fn bench_by_block(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("by_blocks");
    
    let inputs = RawInputs::new_from_json(PATH);
    let blocks = ["0","latest"];

    for (url, name) in zip(inputs.urls.iter(), inputs.names.iter()) {
        for block in blocks.iter() {
            let bench_runner = BenchRunner::new(
                name.as_str(), 
                url.as_str(), 
                "starknet_blockNumber", 
                block, 
                inputs.params.class_hash.as_str(), 
                inputs.params.tx_hash.as_str());

            bench_runner.run(&mut group, &rt, true);
        }
    }
}

criterion_group!(name = benches;
                config = Criterion::default();
                targets = bench_by_method, bench_by_block);
criterion_main!(benches);
