use benchmarking_suite::utils::parse_block_id;
use benchmarking_suite::{BenchedProvider, BenchRunner};
use starknet::providers::Provider;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

const PATH: &str = "./config/config.json";

pub fn bench_providers(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("providers");
    let bench_runner = BenchRunner::new_from_json(PATH);

    for url in bench_runner.inputs.urls.iter() {
        let provider = BenchedProvider::new(url.as_str());
        for method_name in bench_runner.inputs.methods.iter() {
            BenchRunner::run_by_method(&mut group, &provider, method_name, &rt);
        }
    }
}

criterion_group!(benches_p, bench_providers);
criterion_main!(benches_p);
