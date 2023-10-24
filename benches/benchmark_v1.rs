use benchmarking_suite::constants::{PATH, SAMPLE_BLOCK_TAGS};
use benchmarking_suite::{BenchRunner, RawInputs};
use criterion::{criterion_group, criterion_main, Criterion};

// Benches a list of methods on a list of providers,
// (with fixed block number, class hash and tx hash
// when needed by the method).
pub fn bench_by_method(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("By_methods");

    let inputs = RawInputs::new_from_json(PATH);

    for target in inputs.targets {
        for method_name in inputs.methods.iter() {
            let bench_runner = BenchRunner::new(
                target.name.as_str(),
                target.url.as_str(),
                method_name.as_str(),
                inputs.params.block.as_str(),
                inputs.params.class_hash.as_str(),
                inputs.params.tx_hash.as_str(),
            );

            bench_runner.run(&mut group, &rt, false);
        }
    }
}

// Benches a fixed method on a list of block numbers,
// on a list of providers.
pub fn bench_by_block(c: &mut Criterion) {
    let method = "starknet_getStateUpdate";
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group(format!("By_blocks (method: {})", method));

    let inputs = RawInputs::new_from_json(PATH);
    let blocks = SAMPLE_BLOCK_TAGS;

    for target in inputs.targets {
        for block in blocks.iter() {
            let bench_runner = BenchRunner::new(
                target.name.as_str(),
                target.url.as_str(),
                method,
                block,
                inputs.params.class_hash.as_str(),
                inputs.params.tx_hash.as_str(),
            );

            bench_runner.run(&mut group, &rt, true);
        }
    }
}

criterion_group!(name = benches;
                config = Criterion::default();
                targets = bench_by_method, bench_by_block);
criterion_main!(benches);
