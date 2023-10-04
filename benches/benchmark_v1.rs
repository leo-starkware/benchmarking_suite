use benchmarking_suite::{BenchedProvider, BenchRunner};
use criterion::{criterion_group, criterion_main, Criterion};
use benchmarking_suite::constants::PATH;

pub fn bench_by_method(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("providers");
    let bench_runner = BenchRunner::new_from_json(PATH);

    for url in bench_runner.inputs.urls.iter() {
        let provider = BenchedProvider::new(url.as_str());
        for method_name in bench_runner.inputs.methods.iter() {
            bench_runner.run_by_method(&mut group, &provider, method_name, &rt);
        }
    }
}

pub fn bench_by_block(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("providers");
    let bench_runner = BenchRunner::new_from_json(PATH);

    for url in bench_runner.inputs.urls.iter() {
        let provider = BenchedProvider::new(url.as_str());
        bench_runner.run_by_block(&mut group, &provider, &rt);
        
    }
}

criterion_group!(benches_p, bench_by_method);
criterion_main!(benches_p);
