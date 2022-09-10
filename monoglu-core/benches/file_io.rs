use criterion::{black_box, criterion_group, criterion_main, Criterion};
use monoglu_core::{load_file, storage_path};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| 
        load_file(black_box(&storage_path().unwrap()), "Git-2.37.1-64-bit.exe")
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);