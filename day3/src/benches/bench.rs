use std::{
    fs::File,
    io::BufReader,
};

use criterion::{criterion_group, criterion_main, Criterion};
use day3::parts_sum;

// time:   [122.17 µs 122.69 µs 123.39 µs]
pub fn pt1_benchmark(c: &mut Criterion) {
    c.bench_function("parts_sum official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(File::open("src/benches/input.txt").unwrap()),
            |reader| assert_eq!(parts_sum(reader), 531561),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, pt1_benchmark);
criterion_main!(benches);
