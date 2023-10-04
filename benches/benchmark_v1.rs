use benchmarking_suite::{BenchedProvider, BenchRunner};
use criterion::{criterion_group, criterion_main, Criterion};
use benchmarking_suite::constants::PATH;
use std::iter::zip;

pub fn bench_by_method(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("providers");
    let bench_runner = BenchRunner::new_from_json(PATH);

    let url_iter = bench_runner.inputs.urls.iter();
    let name_iter = bench_runner.inputs.names.iter();

    for (url, name) in zip(url_iter, name_iter) {
        let provider = BenchedProvider::new(url.as_str());

        for method_name in bench_runner.inputs.methods.iter() {
            bench_runner.run_by_method(&mut group, &provider, name, method_name, &rt);
        }
    }
}

pub fn bench_by_block(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("providers");
    let bench_runner = BenchRunner::new_from_json(PATH);

    let url_iter = bench_runner.inputs.urls.iter();
    let name_iter = bench_runner.inputs.names.iter();

    for (url, name) in zip(url_iter, name_iter) {
        let provider = BenchedProvider::new(url.as_str());
        bench_runner.run_by_block(&mut group, &provider, name, &rt);
        
    }
}

criterion_group!(benches_p, bench_by_method);
criterion_main!(benches_p);
