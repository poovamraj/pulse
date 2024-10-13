use std::fs;
use std::path::Path;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use heed::EnvOpenOptions;
use storage::new_heed;
use storage::Storage;

pub fn criterion_benchmark(c: &mut Criterion) {
    let path = Path::new("target").join("heed.mdb");
    fs::create_dir_all(&path).expect("failure creating dir");
    let env = unsafe { EnvOpenOptions::new().open(&path) }.expect("failure with env");

    let wtxn = env.write_txn().expect("failure with write"); // We open the default unnamed database
    let mut heed = new_heed(env.clone(), wtxn).expect("Cannot create");

    c.bench_function("benchmark running heed storage write and get", |b| b.iter(|| {
        heed.create_workflow(black_box("test")).expect("Cannot Write");
        heed.get_workflow(black_box("test")).expect("Cannot Read");
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
